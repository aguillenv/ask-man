use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Option<Vec<Choice>>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

pub fn get_command_sync(user_prompt: &str) -> Result<String, reqwest::Error> {
    let system_prompt = include_str!("../prompt.txt");

    let req = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            Message {
                role: "system".into(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".into(),
                content: user_prompt.to_string(),
            },
        ],
        temperature: 0.0,
    };

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let client = Client::new();
    let res: OpenAIResponse = client
        .post(OPENAI_URL)
        .bearer_auth(api_key)
        .json(&req)
        .send()?
        .json()?;

    let choices = res.choices;

    match choices {
        Some(choices) if !choices.is_empty() => Ok(choices
            .first()
            .map_or_else(|| "".to_string(), |c| c.message.content.trim().to_string())),
        _ => Ok("Command unknown".to_string()),
    }
}
