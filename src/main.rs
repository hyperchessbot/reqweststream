use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut stream = reqwest::get("http://httpbin.org/ip")
  .await?
  .bytes_stream();

  while let Some(item) = stream.next().await {
      println!("Chunk: {:?}", item?);
  }

  Ok(())
}
