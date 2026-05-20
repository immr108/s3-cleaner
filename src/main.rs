use aws_sdk_s3::{Client, primitives::Length};
use dialoguer::Select;
use std::io;

#[tokio::main]
async fn main() {
    println!("AWSに接続しています");

    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);

    let response = match client.list_buckets().send().await {
        Ok(res) => res,
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    let buckets = response.buckets();
    let bucket_names: Vec<&str> = buckets.iter().filter_map(|b| b.name()).collect();

    if bucket_names.is_empty() {
        println!("バケットが存在しません");
        return;
    }

    let selection = Select::new()
        .with_prompt("削除するバケットを選択してください")
        .items(&bucket_names)
        .interact()
        .expect("選択エラー");

    let selected_bucket = bucket_names[selection];
    println!("選択されたバケット: {}", selected_bucket);
    println!("本当に削除しますか？削除する場合は[delete]と入力してください: ");

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("入力エラー");
    let confirm = confirm.trim();

    if confirm != "delete" {
        println!("キャンセルしました");
        return;
    }

    println!("削除処理を開始します。");

    let objects = client
        .list_objects_v2()
        .bucket(selected_bucket)
        .send()
        .await;

    match objects {
        Ok(res) => {
            let contents = res.contents();
            println!("オブジェクト一覧");
            for object in contents {
                println!("- {}", object.key().unwrap_or("名前なし"));
            }
        }
        Err(e) => {
            println!("オブジェクト取得エラー: {}", e);
            return;
        }
    }
}
