use cargo_toml::Manifest;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = cargo_manifest_dir.join("Cargo.toml");
    let manifest = Manifest::from_path(cargo_toml)?;

    let manifest = manifest
        .package
        .expect("package not specified in Cargo.toml")
        .metadata
        .expect("no metadata specified in Cargo.toml");

    let metadata = manifest
        .get("assets")
        .expect("cargo manifest package.metadata.assets is set");

    let web_path = metadata
        .get("root")
        .and_then(|x| x.as_str())
        .map(PathBuf::from)
        .expect("root path is set");

    let web_path = web_path
        .canonicalize()
        .expect("valid path that can be canonicalized");

    let blog_path = metadata
        .get("mdfiles")
        .and_then(|x| x.as_str())
        .map(PathBuf::from)
        .expect("mdfiles path is set");

    println!("cargo:rerun-if-changed={}", web_path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", blog_path.to_str().unwrap());

    let mut web_build_resource = static_files::NpmBuild::new(&web_path)
        .stdout(std::io::stdout())
        .stderr(std::io::stderr())
        .install()
        .unwrap()
        .stdout(std::io::stdout())
        .stderr(std::io::stderr())
        .run("build")
        .unwrap()
        .target(web_path.join("dist").to_str().unwrap())
        .to_resource_dir();

    web_build_resource
        .with_generated_filename(Path::new(&std::env::var("OUT_DIR").unwrap()).join("web.rs"))
        .with_module_name("web_sets");

    web_build_resource.build().unwrap();

    static_files::resource_dir(blog_path).build().unwrap();

    Ok(())
}
