// Add the reqwest crate to the project
use reqwest;

// The main function
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Define the URL you want to make a request to
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // Send a GET request and await the response
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Print the response body as text
        let body = response.text().await?;
        println!("Response body:\n{}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
