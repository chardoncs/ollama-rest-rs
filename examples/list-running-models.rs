use ollama_rest::Ollama;

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    let models = ollama.running_models().await.unwrap();
    
    println!("Models online");
    println!("==========");

    for model in models.models.iter() {
        println!("{}", model.name);
    }
}
