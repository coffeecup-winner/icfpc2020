use hyper::header::LOCATION;
use hyper::{body, Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;

fn string_from_bytes(bytes: &[u8]) -> String {
    String::from(String::from_utf8_lossy(&bytes[..]))
}

#[tokio::main]
pub async fn request(
    base: &String,
    token: Option<String>,
    content: &String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut endpoint = match token {
        None => base.clone(),
        Some(token) => format!("{}?apiKey={}", base, token),
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    loop {
        let body = Body::from(content.clone());
        let req = Request::builder()
            .method(Method::POST)
            .uri(endpoint)
            .body(body)?;

        let mut res = match client.request(req).await {
            Ok(res) => res,
            Err(err) => panic!("Unexpected server response:\n{}", err),
        };

        let body_data = body::to_bytes(res.body_mut()).await?;

        print!("Server response: ");
        println!("{:?}", body_data);

        match res.status() {
            StatusCode::OK => break Ok(string_from_bytes(&body_data[..])),
            StatusCode::FOUND => {
                endpoint = string_from_bytes(res.headers()[LOCATION].as_bytes());
                println!("updated endpoint to {:?}", endpoint);
            }
            _ => panic!("Unexpected server response: {}", res.status()),
        }
    }
}
