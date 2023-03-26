use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug)]
pub struct Config {
    pub organization: String,
    pub api_key: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let env_vars = env::vars();

        if env_vars.count() < 2 {
            return Err("Not enough arguments have been supplied");
        }

        let organization = match env::var("ORGANIZATION_ID") {
            Ok(value) => value,
            Err(_) => return Err("ORGANIZATION_ID must exists"),
        };

        let api_key = match env::var("API_KEY") {
            Ok(value) => value,
            Err(_) => return Err("API_KEY must exists"),
        };

        Ok(Config {
            organization,
            api_key,
        })
    }
}

// TODO: Handle Request Options such as max_tokens

#[derive(Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}
/// Get a chat completion from Open AI, you would need to borrow an instance a `Config` object
/// and a Request body, defined by the struct `CompletionRequest`
pub async fn new_chat_completion(
    config: &Config,
    body: CompletionRequest,
) -> Result<CompletionResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let request = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&config.api_key)
        .header("OpenAI-Organization", &config.organization)
        .json(&body)
        .send()
        .await?;

    let response = request.json::<CompletionResponse>().await?;

    Ok(response)
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let env_args: Vec<String> = env::args().collect();

    let mut messages: Vec<Message> = Vec::new();
    messages.push(Message {
        role: String::from("user"),
        content: String::from(&env_args[1]),
    });

    let body = CompletionRequest {
        model: String::from("gpt-3.5-turbo"),
        messages,
        temperature: 0.0,
    };

    println!("Asking: \"{}\" with model: \"gpt-3.5-turbo\"", &env_args[1]);

    let completion = new_chat_completion(&config, body).await?;

    println!("Response: {}", completion.choices[0].message.content);

    Ok(())
}
