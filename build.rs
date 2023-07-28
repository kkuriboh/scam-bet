use std::{
    fs::{create_dir, read_dir, File},
    io::Write,
    path::Path,
    process::Command,
};

const OUT_DIR: &str = "./public/games";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=games");
    let _ = create_dir(OUT_DIR);

    for dir in read_dir("games")?.filter_map(Result::ok) {
        let p = dir.path();
        let dirname = p.file_name().unwrap().to_string_lossy();
        let out = Command::new("bun")
            .arg("build")
            .arg("--minify")
            .arg(p.join("index.ts"))
            .output()?;
        let out_path = Path::new(OUT_DIR).join(format!("{dirname}.js"));
        let mut f = File::create(out_path)?;
        f.write_all(&out.stdout)?;
        f.flush()?;
    }

    Command::new("bun")
        .env("NODE_ENV", "production")
        .arg("tailwindcss")
        .arg("-c")
        .arg("./tailwind.config.js")
        .arg("-o")
        .arg("./style/tailwind.css")
        .arg("--minify")
        .output()?;

    Ok(())
}
