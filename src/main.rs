use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("環境變數 GEMINI_API_KEY 未設定，請於 GitHub Secrets 或 .env 中提供");
    let client = Client::new();

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "llama-3.3-70b-specdec",
            "messages": [
                {"role": "system", "content": "你是一位資深獵頭，請用 JSON 格式提供一個虛釋的遠端前端職缺。"},
                {"role": "user", "content": "請開始。"}
            ]
        }))
        .send()
        .await?
        .text()
        .await?;

    println!("Groq 回傳結果: \n{}", response);
    Ok(())
}
