# s3-cleaner

S3バケットをオブジェクトごと強制削除するCLIツールです。

## 必要なもの

- Rust
- AWS CLI（`aws login` で認証済みであること）

## インストール

```bash
git clone https://github.com/ユーザー名/s3-cleaner.git
cd s3-cleaner
cargo build --release
```

## 使い方

```bash
cargo run
```

1. バケット一覧が表示されるので、カーソルで削除するバケットを選択
2. `delete` と入力して削除を確定

```
AWSに接続しています
? 削除するバケットを選択してください
❯ my-bucket-1
  my-bucket-2

選択されたバケット: my-bucket-1
本当に削除しますか？削除する場合は[delete]と入力してください:
delete
削除処理を開始します。
削除しました: image.png
削除しました: document.pdf
全オブジェクトの削除が完了しました
バケットを削除しました: my-bucket-1
```

## 注意

- 削除したバケットとオブジェクトは復元できません
- 実行前に必ず対象のバケットを確認してください
