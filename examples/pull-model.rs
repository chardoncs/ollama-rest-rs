use std::io::Write;

use ollama_rest::{models::model::{ModelPullStatus, ModelSyncRequest}, Ollama};

// Use llama3.2:1b because it is good for demonstration due to its size.
const MODEL_NAME: &str = "llama3.2:1b";

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    let mut prev_status = String::new();

    // Callback API
    //
    // For Stream API, see `Ollama::pull_model_streamed()`.
    ollama.pull_model(&serde_json::from_value::<ModelSyncRequest>(serde_json::json!({
        "name": MODEL_NAME,
    })).unwrap(), Some(|res: &ModelPullStatus| {
        if !prev_status.starts_with(res.status.as_str()) {
            prev_status = res.status.clone();
            println!("\n{}", res.status);
        }

        if let Some(progress) = &res.download_info {
            print!("\r{} / {}", progress.completed.unwrap_or(0), progress.total);
            std::io::stdout().flush().unwrap();
        }
    })).await.unwrap();
}
