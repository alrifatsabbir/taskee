use std::fs;
use std::path::Path;
use taskee_website::pages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("dist");
    println!("Exporting static website to: {:?}", out_dir.display());

    // Create directories
    fs::create_dir_all(out_dir)?;
    fs::create_dir_all(out_dir.join("about"))?;
    fs::create_dir_all(out_dir.join("install"))?;
    fs::create_dir_all(out_dir.join("contact"))?;
    fs::create_dir_all(out_dir.join("static/css"))?;
    fs::create_dir_all(out_dir.join("static/js"))?;
    fs::create_dir_all(out_dir.join("static/images"))?;
    fs::create_dir_all(out_dir.join("static/downloads"))?;
    fs::create_dir_all(out_dir.join("download"))?;

    // Render HTML pages
    fs::write(out_dir.join("index.html"), pages::home::render_home())?;
    fs::write(out_dir.join("about/index.html"), pages::about::render_about())?;
    fs::write(out_dir.join("install/index.html"), pages::install::render_install())?;
    fs::write(out_dir.join("contact/index.html"), pages::contact::render_contact())?;
    fs::write(out_dir.join("404.html"), pages::not_found::render_not_found("/404"))?;

    // Copy static assets
    let static_src = Path::new("static");
    if static_src.exists() {
        copy_dir_all(static_src, &out_dir.join("static"))?;
    }

    // Copy root favicon
    if Path::new("static/images/favicon.svg").exists() {
        fs::copy("static/images/favicon.svg", out_dir.join("favicon.svg"))?;
    }

    // Direct download route for Vercel
    if Path::new("static/downloads/taskee-windows-x86_64.exe").exists() {
        fs::copy(
            "static/downloads/taskee-windows-x86_64.exe",
            out_dir.join("download/taskee-windows-x86_64.exe"),
        )?;
    }

    // Write vercel.json for clean routing
    let vercel_json = r#"{
  "cleanUrls": true,
  "trailingSlash": false
}
"#;
    fs::write(out_dir.join("vercel.json"), vercel_json)?;

    println!("Static export completed successfully in dist/!");
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
