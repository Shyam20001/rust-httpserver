use reqwest;
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
// Other imports and your code

fn main() -> std::io::Result<()> {
    // Get the address and port from the environment variables
    let address = env::var("ROCKET_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("ROCKET_PORT").unwrap_or("8080".to_string());

    let listener = TcpListener::bind(format!("{}:{}", address, port))?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        // Fetch the content of index.html from a GitHub repository
        let github_url =
            "https://raw.githubusercontent.com/chromecruzer/self-learn/main/index.html";
        let content = reqwest::blocking::get(github_url)
            .expect("Failed to fetch index.html")
            .text()
            .expect("Failed to read response text");

        // Prepare the HTTP response
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            content.len(),
            content
        );

        // Send the response
        stream.write(response.as_bytes())?;
        stream.flush()?;
    }
    Ok(())
}
