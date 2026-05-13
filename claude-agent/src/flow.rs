use crate::{Agent, AgentEvent, Provider, run_agent};
use reqwest::Client;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub(crate) enum FlowKind {
    Concurrent,
    ResearchThenWrite,
}

pub(crate) fn read_flow(args: &[String]) -> FlowKind {
    let flow_arg = args
        .windows(2)
        .find(|pair| pair[0] == "--flow")
        .map(|pair| pair[1].as_str())
        .unwrap_or("research-then-write");

    match flow_arg {
        "concurrent" => FlowKind::Concurrent,
        "research-then-write" | "sequential" => FlowKind::ResearchThenWrite,
        _ => FlowKind::ResearchThenWrite,
    }
}

pub(crate) async fn run_flow(
    flow: FlowKind,
    provider: Provider,
    client: Client,
    researcher: Agent,
    writer: Agent,
    tx: mpsc::Sender<AgentEvent>,
) {
    match flow {
        FlowKind::Concurrent => {
            run_concurrent(provider, client, researcher, writer, tx).await;
        }
        FlowKind::ResearchThenWrite => {
            run_research_then_write(provider, client, researcher, writer, tx).await;
        }
    }
}

async fn run_concurrent(
    provider: Provider,
    client: Client,
    researcher: Agent,
    writer: Agent,
    tx: mpsc::Sender<AgentEvent>,
) {
    let researcher_handle = {
        let tx = tx.clone();
        let client = client.clone();
        let provider = provider.clone();

        tokio::spawn(async move {
            run_agent(
                researcher,
                "Explore why Rust is a strong fit for AI-agent orchestration runtimes. Keep it concise.",
                provider,
                client,
                tx,
            )
            .await;
        })
    };

    let writer_handle = {
        let tx = tx.clone();
        let client = client.clone();
        let provider = provider.clone();

        tokio::spawn(async move {
            run_agent(
                writer,
                "Write an academic-style paragraph explaining the value of a Rust runtime for agent DAG execution.",
                provider,
                client,
                tx,
            )
            .await;
        })
    };

    for handle in [researcher_handle, writer_handle] {
        if let Err(error) = handle.await {
            let _ = tx
                .send(AgentEvent::Failed {
                    agent: String::from("flow"),
                    error: format!("agent task failed: {error}"),
                })
                .await;
        }
    }
}

async fn run_research_then_write(
    provider: Provider,
    client: Client,
    researcher: Agent,
    writer: Agent,
    tx: mpsc::Sender<AgentEvent>,
) {
    let research_output = run_agent(
        researcher,
        "Explore why Rust is a strong fit for AI-agent orchestration runtimes. Keep it concise.",
        provider.clone(),
        client.clone(),
        tx.clone(),
    )
    .await;

    let Some(research_output) = research_output else {
        return;
    };

    let writer_input = format!(
        "Use these research notes to write one concise academic paragraph.\n\nResearch notes:\n{research_output}"
    );

    run_agent(writer, &writer_input, provider, client, tx).await;
}
