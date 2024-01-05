use reqwest::blocking;

pub fn placeholder() -> Result<(), reqwest::Error> {
    let body = blocking::get("http://bittorrent-test-tracker.codecrafters.io/announce")?.text()?;

    println!("body = {:?}", body);
    Ok(())
}
