use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    get_ollama_exe();
    setup_model();
    copy_deploy_scripts();
    println!("cargo:rerun-if-changed=*");
}

fn download_file(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = get_project_directory();
    let target_path = Path::new(&target_dir);
    let output_path = target_path.join(output);
    if Path::new(&output_path).exists() {
        return Ok(());
    }
    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build()?;
    let response = client.get(url).send()?;
    let mut file = File::create(&output_path)?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    Ok(())
}

fn get_project_directory() -> String {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .to_str().unwrap()
        .to_owned()
}

#[cfg(target_os = "windows")]
fn get_ollama_exe() {
    let target_dir = get_project_directory();
    let target_path = Path::new(&target_dir);
    let output_dir = target_path.join("ollama-win");
    if output_dir.exists() {
        return;
    }
    download_file(
        "https://github.com/ollama/ollama/releases/download/v0.11.4/ollama-windows-amd64.zip",
        "ollama-win.zip",
    ).unwrap();
    let output_zip = target_path.join("ollama-win.zip");
    std::fs::create_dir_all(&output_dir).unwrap();
    let file = std::fs::File::open(&output_zip).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive.extract(output_dir).unwrap();
    std::fs::remove_file(&output_zip).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn get_ollama_exe() {
    let target_dir = get_project_directory();
    let target_path = Path::new(&target_dir);
    let output_dir = target_path.join("ollama-lin");
    if output_dir.exists() {
        return;
    }
    download_file(
        "https://github.com/ollama/ollama/releases/download/v0.11.4/ollama-linux-amd64.tgz",
        "ollama-lin.tgz",
    ).unwrap();
    let output_tgz = target_path.join("ollama-lin.tgz");
    std::fs::create_dir_all(&output_dir).unwrap();
    let tar_gz_file = std::fs::File::open(&output_tgz).unwrap();
    let tar_file = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar_file);
    archive.unpack(&output_dir).unwrap();
    std::fs::remove_file(&output_tgz).unwrap();
} // TODO: untested

fn setup_model() {
    let target_dir = get_project_directory();
    let target_path = Path::new(&target_dir);
    let output_dir = target_path.join("ollama-model");
    if output_dir.exists() {
        return;
    }
    std::fs::create_dir_all(&output_dir).unwrap();
    let output_modelfile = output_dir.join("ModelFile");
    let _ = std::fs::write(&output_modelfile, "");
}

fn copy_deploy_scripts() {
    let target_dir = get_project_directory();
    let target_path = Path::new(&target_dir);
    let win_path = target_path.join("deploy-win.bat");
    if !win_path.exists() {
        std::fs::copy("deploy-win.bat", &win_path).unwrap();
    }
    let lin_path = target_path.join("deploy-lin.bat");
    if !lin_path.exists() {
        std::fs::copy("deploy-lin.bat", &lin_path).unwrap();
    }
}