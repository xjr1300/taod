# TAOD(Traffic Accident Open Data)

## 交通事故統計情報オープンデータ

* 出典: 警察庁ウェブサイト<https://www.npa.go.jp/publications/statistics/koutsuu/opendata/index_opendata.html>

## 免責事項

* 本データは、警察庁が公表している交通事故統計情報オープンデータを加工したものです。
* 当該オープンデータを扱いやすくするために、正規化などしてデータベースに格納しています。
* 当該オープンデータが記録している内容を可能な限り正確に反映していますが、その内容の正確性を保証するものではありません。
* 本リポジトリを利用したことにより生じたいかなる損害についても、当方は一切の責任を負いません。
* 当該オープンデータを公開している警察庁は、本リポジトリについて関知されておられません。

## 動作確認環境

* rust 1.73.0 (cc66ad468 2023-10-03)
  * <https://www.rust-lang.org/tools/install>
* sqlx-cli 0.7.2
  * `cargo install sqlx-cli --no-default-features --features rustls,postgres`
* Docker version 24.0.6, build ed223bc
* Docker Compose version v2.21.0
* psql (PostgreSQL) 15.2

## 運用環境構築

### PostgreSQL + PostGISコンテナのビルドと起動

```sh
docker-compose up -d
```

### マイグレーションの実行

```sh
sqlx migrate run
```
