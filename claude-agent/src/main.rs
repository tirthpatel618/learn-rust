mod flow;

use flow::{read_flow, run_flow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub(crate) struct Agent {
    name: String,
    system_prompt: String,
}

#[derive(Debug, Clone)]
pub(crate) enum Provider {
    Fake,
    Anthropic,
    OpenAi,
}

#[derive(Debug)]
pub(crate) enum AgentEvent {
    Started { agent: String, provider: String },
    Token { agent: String, text: String },
    Completed { agent: String, output: String },
    Failed { agent: String, error: String },
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
}

#[derive(Debug, Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponseMessage {
    content: String,
}

#[tokio::main]
async fn main() {
    load_env();

    let args: Vec<String> = env::args().collect();
    let provider = read_provider(&args);
    let flow = read_flow(&args);
    let client = Client::new();

    let researcher = Agent {
        name: String::from("researcher"),
        system_prompt: String::from(
            "You are an exploratory research agent. Find the important angles, surface tradeoffs, avoid hype, and produce concise notes with concrete claims.",
        ),
    };
    let writer = Agent {
        name: String::from("writer"),
        system_prompt: String::from(
            "You are an academic writer. Turn rough research notes into precise, structured prose with a careful, evidence-oriented tone.",
        ),
    };

    let (tx, mut rx) = mpsc::channel::<AgentEvent>(64);

    let flow_tx = tx.clone();
    let flow_handle = tokio::spawn(async move {
        run_flow(flow, provider, client, researcher, writer, flow_tx).await;
    });

    drop(tx);

    while let Some(event) = rx.recv().await {
        match event {
            AgentEvent::Started { agent, provider } => {
                println!("{agent} started with {provider}");
            }
            AgentEvent::Token { agent, text } => {
                println!("{agent} token: {text}");
            }
            AgentEvent::Completed { agent, output } => {
                println!("{agent} completed:\n{output}\n");
            }
            AgentEvent::Failed { agent, error } => {
                println!("{agent} failed: {error}");
            }
        }
    }

    if let Err(error) = flow_handle.await {
        println!("flow task failed: {}", error);
    }
}

fn load_env() {
    if dotenvy::dotenv().is_ok() {
        return;
    }

    dotenvy::from_filename("src/.env").ok();
}

fn read_provider(args: &[String]) -> Provider {
    let provider_arg = args
        .windows(2)
        .find(|pair| pair[0] == "--provider")
        .map(|pair| pair[1].as_str())
        .unwrap_or("fake");

    match provider_arg {
        "anthropic" | "claude" => Provider::Anthropic,
        "openai" => Provider::OpenAi,
        _ => Provider::Fake,
    }
}

pub(crate) async fn run_agent(
    agent: Agent,
    input: &str,
    provider: Provider,
    client: Client,
    tx: mpsc::Sender<AgentEvent>,
) -> Option<String> {
    let provider_name = provider_name(&provider).to_string();
    let _ = tx
        .send(AgentEvent::Started {
            agent: agent.name.clone(),
            provider: provider_name,
        })
        .await;

    let output = match provider {
        Provider::Fake => fake_agent_output(&agent, input).await,
        Provider::Anthropic => call_anthropic(&client, &agent, input).await,
        Provider::OpenAi => call_openai(&client, &agent, input).await,
    };

    match output {
        Ok(output) => {
            stream_completed_output(agent.name, output.clone(), tx).await;
            Some(output)
        }
        Err(error) => {
            let _ = tx
                .send(AgentEvent::Failed {
                    agent: agent.name,
                    error: error.to_string(),
                })
                .await;
            None
        }
    }
}

async fn fake_agent_output(
    agent: &Agent,
    input: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    sleep(Duration::from_millis(250)).await;

    Ok(format!(
        "{}\n\nInput: {}\n\nFake response: Rust is useful here because scheduling, bounded channels, typed events, and low-overhead concurrency are runtime concerns rather than prompt-writing concerns.",
        agent.system_prompt, input
    ))
}

async fn call_anthropic(
    client: &Client,
    agent: &Agent,
    input: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| "ANTHROPIC_API_KEY is not set")?;
    let model =
        env::var("ANTHROPIC_MODEL").unwrap_or_else(|_| String::from("claude-3-5-haiku-20241022"));

    let request = AnthropicRequest {
        model,
        max_tokens: 500,
        system: agent.system_prompt.clone(),
        messages: vec![AnthropicMessage {
            role: String::from("user"),
            content: input.to_string(),
        }],
    };

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!("Anthropic API returned {status}: {body}").into());
    }

    let parsed = serde_json::from_str::<AnthropicResponse>(&body)?;
    let text = parsed
        .content
        .into_iter()
        .filter(|block| block.block_type == "text")
        .filter_map(|block| block.text)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(text)
}

async fn call_openai(
    client: &Client,
    agent: &Agent,
    input: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY is not set")?;
    let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| String::from("gpt-4o-mini"));

    let request = OpenAiRequest {
        model,
        messages: vec![
            OpenAiMessage {
                role: String::from("developer"),
                content: agent.system_prompt.clone(),
            },
            OpenAiMessage {
                role: String::from("user"),
                content: input.to_string(),
            },
        ],
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!("OpenAI API returned {status}: {body}").into());
    }

    let parsed = serde_json::from_str::<OpenAiResponse>(&body)?;
    let text = parsed
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| String::from("OpenAI response did not include a choice"))?;

    Ok(text)
}

async fn stream_completed_output(agent: String, output: String, tx: mpsc::Sender<AgentEvent>) {
    for token in output.split_whitespace() {
        sleep(Duration::from_millis(60)).await;

        let _ = tx
            .send(AgentEvent::Token {
                agent: agent.clone(),
                text: token.to_string(),
            })
            .await;
    }

    let _ = tx.send(AgentEvent::Completed { agent, output }).await;
}

fn provider_name(provider: &Provider) -> &'static str {
    match provider {
        Provider::Fake => "fake",
        Provider::Anthropic => "anthropic",
        Provider::OpenAi => "openai",
    }
}
