# イベント用

## 前提

nightly 版の Rust が必要です
入っていない場合は

```sh
rustup install nightly
```

でインストールしてください

以下は

```sh
cd shanten-algorithm-collection
```

で実行します

## テスト実行

```sh
cargo +nightly test --package my_algo
```

## ベンチマーク実行

```sh
cargo +nightly bench --package my_algo
```
