use aws_sdk_s3::Client;

#[tokio::main]
async fn main() {
    println!("AWSに接続しています");

    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);

    match client.list_buckets().send().await {
        Ok(response) => {
            let buckets = response.buckets();
            println!("バケット一覧:");
            for bucket in buckets {
                println!(" - {}", bucket.name().unwrap_or("名前なし"));
            }
        }
        Err(e) => println!("エラー: {}", e),
    }
}