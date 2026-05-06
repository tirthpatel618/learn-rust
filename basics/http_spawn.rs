use reqwest::Client;
use std::env;
use std::error::Error;
use std::time::Instant;


/*
tokio::spawn starts each request as its own async task.
async move means the task takes ownership of client and url. That matters because the spawned task may outlive the loop iteration.
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = read_urls_from_args();
    let client = Client::new();
    let start = Instant::now();
    let mut handles = Vec::new();

    for url in urls {
        let client = client.clone();

        let handle = tokio::spawn(async move {
            fetch_url(client, url).await
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(summary)) => println!("{}", summary),
            Ok(Err(error)) => println!("request failed: {}", error),
            Err(error) => println!("task failed: {}", error),
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

async fn fetch_url(client: Client, url: String) -> Result<String, reqwest::Error> {
    let response = client.get(&url).send().await?;
    let status = response.status();
    let body = response.text().await?;

    Ok(format!("{} -> {} ({} bytes)", url, status, body.len()))
}
