use reqwest::Client;
use std::env;
use std::error::Error;
use std::time::Instant;
use tokio::sync::mpsc;


// This example demonstrates how to use Tokio's mpsc (multi-producer, single-consumer) channels to communicate between async tasks.
//  Each task fetches a URL and sends events about its progress back to the main task, which prints them out.
// The FetchEvent enum defines the different types of events that can occur during the fetching process, 
// such as when a fetch starts, completes successfully, or fails with an error. 
// Each event includes relevant information like the URL being fetched, the HTTP status code, and any error messages.
#[derive(Debug)]
enum FetchEvent {
    Started {
        url: String,
    },
    Completed {
        url: String,
        status: reqwest::StatusCode,
        bytes: usize,
    },
    Failed {
        url: String,
        error: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = read_urls_from_args();
    let client = Client::new();
    let start = Instant::now();
    let mut handles = Vec::new();
    // Create a channel with a buffer size of 32 to send FetchEvent messages from the spawned tasks back to the main task.
    let (tx, mut rx) = mpsc::channel::<FetchEvent>(32);

    for url in urls {
        // clone the client and transmitter for each task, since they will be moved into the async block.
        let client = client.clone();
        let tx = tx.clone();

        let handle = tokio::spawn(async move {
            fetch_url(client, url, tx).await;
        });

        handles.push(handle);
    }

    drop(tx);

    while let Some(event) = rx.recv().await {
        match event {
            FetchEvent::Started { url } => println!("started: {}", url),
            FetchEvent::Completed {
                url,
                status,
                bytes,
            } => println!("completed: {} -> {} ({} bytes)", url, status, bytes),
            FetchEvent::Failed { url, error } => println!("failed: {} -> {}", url, error),
        }
    }

    for handle in handles {
        if let Err(error) = handle.await {
            println!("task failed: {}", error);
        }
    }

    println!("elapsed: {:?}", start.elapsed());
    Ok(())
}

fn read_urls_from_args() -> Vec<String> {
    let urls: Vec<String> = env::args().skip(1).collect();

    if urls.is_empty() {
        vec![
            String::from("https://www.rust-lang.org"),
            String::from("https://tokio.rs"),
            String::from("https://docs.rs/reqwest"),
        ]
    } else {
        urls
    }
}

async fn fetch_url(client: Client, url: String, tx: mpsc::Sender<FetchEvent>) {
    let _ = tx.send(FetchEvent::Started { url: url.clone() }).await;

    let response = match client.get(&url).send().await {
        Ok(response) => response,
        Err(error) => {
            let _ = tx
                .send(FetchEvent::Failed {
                    url,
                    error: error.to_string(),
                })
                .await;
            return;
        }
    };

    let status = response.status();
    let body = match response.text().await {
        Ok(body) => body,
        Err(error) => {
            let _ = tx
                .send(FetchEvent::Failed {
                    url,
                    error: error.to_string(),
                })
                .await;
            return;
        }
    };

    let _ = tx
        .send(FetchEvent::Completed {
            url,
            status,
            bytes: body.len(),
        })
        .await;
}
