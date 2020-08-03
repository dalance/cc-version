use cc::Tool;
use std::cmp::Ordering;
use std::fmt::Display;
use thiserror::Error;
#[derive(Debug, Eq)]
pub struct Version {
    pub major: usize,
    pub minor: Option<usize>,
    pub patch: Option<usize>,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut version = format!("{}", self.major);
        if let Some(x) = self.minor {
            version.push_str(&format!(".{}", x));
        }
        if let Some(x) = self.patch {
            version.push_str(&format!(".{}", x));
        }

        write!(f, "{}", version)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let major = self.major == other.major;
        let minor = self.minor.unwrap_or(0) == other.minor.unwrap_or(0);
        let patch = self.patch.unwrap_or(0) == other.patch.unwrap_or(0);
        major && minor && patch
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major > other.major {
            Ordering::Greater
        } else if self.major < other.major {
            Ordering::Less
        } else if self.minor.unwrap_or(0) > other.minor.unwrap_or(0) {
            Ordering::Greater
        } else if self.minor.unwrap_or(0) < other.minor.unwrap_or(0) {
            Ordering::Less
        } else if self.patch.unwrap_or(0) > other.patch.unwrap_or(0) {
            Ordering::Greater
        } else if self.patch.unwrap_or(0) < other.patch.unwrap_or(0) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CommandFailed(#[from] std::io::Error),
    #[error(transparent)]
    ParseFailed(#[from] std::num::ParseIntError),
    #[error("failed to detect compiler")]
    UnknownCompiler,
}

impl Version {
    pub fn parse<T: AsRef<str>>(x: T) -> Result<Self, Error> {
        let version: Vec<_> = x.as_ref().split('.').collect();
        let major = version[0].parse().map_err(Error::from)?;
        let minor = version
            .get(1)
            .map(|x| x.parse().map_err(Error::from))
            .transpose()?;
        let patch = version
            .get(2)
            .map(|x| x.parse().map_err(Error::from))
            .transpose()?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

pub fn cc_version(tool: &Tool) -> Result<Version, Error> {
    if tool.is_like_gnu() || tool.is_like_clang() {
        let ret = tool
            .to_command()
            .args(&["-dumpversion"])
            .output()
            .map_err(Error::from)?;
        let version = String::from_utf8_lossy(&ret.stdout);
        Ok(Version::parse(version.trim())?)
    } else if tool.is_like_msvc() {
        let ret = std::process::Command::new("cl")
            .output()
            .map_err(Error::from)?;
        dbg!(&ret.stderr);
        let version = String::from_utf8_lossy(&ret.stderr);
        let version = get_msvc_version(&version);
        Ok(Version::parse(version.trim())?)
    } else {
        Err(Error::UnknownCompiler)
    }
}

fn get_msvc_version(s: &str) -> String {
    dbg!(s);
    let start = s.find("Version ").unwrap();
    let end = s.find(" for ").unwrap();
    let version = s.get(start + 8..end).unwrap();
    String::from(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_parse() {
        let version = Version::parse("1.0.0").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, Some(0));
        assert_eq!(version.patch, Some(0));

        let version = Version::parse("1.0").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, Some(0));
        assert_eq!(version.patch, None);

        let version = Version::parse("1").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, None);
        assert_eq!(version.patch, None);
    }

    #[test]
    fn version_eq() {
        let a = Version::parse("1.0.0").unwrap();
        let b = Version::parse("1.0").unwrap();
        let c = Version::parse("1").unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(b, c);
    }

    #[test]
    fn version_cmp() {
        let a = Version::parse("1.0.0").unwrap();
        let b = Version::parse("1.0.0").unwrap();

        assert!(a >= b);
        assert!(!(a > b));
        assert!(!(a < b));
        assert!(a <= b);

        let a = Version::parse("2.0.0").unwrap();
        let b = Version::parse("1.0.0").unwrap();

        assert!(a >= b);
        assert!(a > b);
        assert!(!(a < b));
        assert!(!(a <= b));

        let a = Version::parse("1.2.0").unwrap();
        let b = Version::parse("1.0.0").unwrap();

        assert!(a >= b);
        assert!(a > b);
        assert!(!(a < b));
        assert!(!(a <= b));

        let a = Version::parse("1.0.1").unwrap();
        let b = Version::parse("1.0.0").unwrap();

        assert!(a >= b);
        assert!(a > b);
        assert!(!(a < b));
        assert!(!(a <= b));
    }

    #[test]
    fn msvc_version() {
        let version = get_msvc_version(
            "Microsoft(R) C/C++ Optimizing Compiler Version 19.16.27027.1 for x64
Copyright (C) Microsoft Corporation.  All rights reserved.",
        );
        assert_eq!(version, "19.16.27027.1");

        let version = Version::parse(version).unwrap();
        assert_eq!(version.major, 19);
        assert_eq!(version.minor, Some(16));
        assert_eq!(version.patch, Some(27027));
    }
}
