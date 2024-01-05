use reqwest;
use tokio::runtime::Runtime;

pub fn placeholder() -> Result<(), reqwest::Error> {
    let body = Runtime::new().unwrap().block_on(async {
        reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await
    })?;

    println!("body = {:?}", body);
    Ok(())
}
