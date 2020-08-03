use cc_version::cc_version;

fn main() {
    let builder = cc::Build::new();
    let tool = builder.get_compiler();
    let version = cc_version(&tool).unwrap();
    println!("cargo:warning=cc version {} is detected.", version);
}
