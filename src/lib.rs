// =========================
// === SETTING UP OLLAMA ===
// =========================

fn start(client: &reqwest::blocking::Client) {
    let stdout_gag = gag::Gag::stdout().unwrap();
    let stderr_gag = gag::Gag::stderr().unwrap();
    if !is_running(client) {
        serve();
    }
    if !model_exists(client) {
        create();
    }
    drop(stdout_gag);
    drop(stderr_gag);
}

fn is_running(client: &reqwest::blocking::Client) -> bool {
    let attempt = client.get("http://localhost:11434/api/version")
        .timeout(std::time::Duration::from_secs(10))
        .send();
    if let Ok(response) = attempt {
        if let Ok(text) = response.text() {
            if text.contains("version") {
                return true;
            }
        }
    }
    false
}

fn model_exists(client: &reqwest::blocking::Client) -> bool {
    let attempt = client.post("http://localhost:11434/api/show")
        .body(r#"{ "model": "RustGGUF" }"#)
        .timeout(std::time::Duration::from_secs(15))
        .send();
    if let Ok(response) = attempt {
        if let Ok(text) = response.text() {
            if !text.contains("not found") {
                return true;
            }
        }
    }
    false
}

#[cfg(target_os = "windows")]
fn create() {
    run_command(
        ".\\ollama-win\\ollama.exe create RustGGUF -f .\\ollama-model\\ModelFile"
        )
        .wait()
        .unwrap();
}

#[cfg(not(target_os = "windows"))]
fn create() {
    run_command(
        "./ollama-lin/ollama create RustGGUF -f ./ollama-model/ModelFile"
        )
        .wait()
        .unwrap();
} // TODO: untested

#[cfg(target_os = "windows")]
fn serve() {
    let _ = run_command(".\\ollama-win\\ollama.exe serve");
}

#[cfg(not(target_os = "windows"))]
fn serve() {
    let _ = run_command("./ollama-lin/ollama serve");
} // TODO: untested

#[cfg(target_os = "windows")]
pub fn run_command(cmd: &str) -> std::process::Child {
    std::process::Command::new("cmd")
        .args(["/C", cmd])
        .spawn()
        .unwrap()
}

#[cfg(not(target_os = "windows"))]
pub fn run_command(cmd: &str) -> std::process::Child {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .unwrap()
} // TODO: untested


// =============================
// === HTTP REQUEST CHATTING ===
// =============================

#[derive(serde::Serialize, serde::Deserialize)]
struct Request {
    model: String,
    prompt: String,
    stream: bool,
    think: bool,
}
impl Request {
    fn new(prompt: &str) -> Self {
        Self {
            model: "RustGGUF".to_owned(),
            prompt: prompt.to_owned(),
            stream: false,
            think: false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Response {
    response: Option<String>,
}

pub fn chat(client: &reqwest::blocking::Client, prompt: &str) -> String {
    if !is_running(client) || !model_exists(client) {
        start(client);
    }
    let body = serde_json::to_string(&Request::new(prompt)).expect("JSON to error");
    let url = "http://localhost:11434/api/generate";
    let result = client
        .post(url)
        .timeout(std::time::Duration::from_secs(60))
        .body(body)
        .send()
        .expect("LLM endpoint error");
    let text = result.text().expect("LLM text error");
    let response: Response = serde_json::from_str(&text).expect("JSON from error");
    let text = response.response.expect("Broken response");
    sanitize(&text)
}

fn sanitize(original: &str) -> String {
    let replaces = vec![
        ("’", "'"),
        ("“", "\""),
        ("”", "\""),
        ("\n", " "),
        ("\r", " "),
        ("—", " - "),
    ];
    let mut new = remove_think_tags(original);
    for (o, n) in &replaces {
        new = new.replace(o, n);
    }
    new
}

fn remove_think_tags(input: &str) -> String {
    let re = regex::Regex::new(r#"<think>[^<]*</think>\s*"#).unwrap();
    re.replace_all(input, "").to_string()
}