# プロジェクト改善ロードマップ

## 管理方法

- 状態: `[x]` 完了、`[ ]` 未完了、`[-]` 保留
- 優先度は P0 が最高。
- 各項目の「依存」は、着手または完了判定の前提を示す。
- 公開READMEとアルゴリズムREADMEは英語、このROADMAPは日本語で管理する。

## P0 — 既存実装と外部crate比較の統合

依存: なし。P1以降のworkspace横断作業より先に行う。

- [x] `feat/dfs` の `pruning_dfs_ymatsux` を統合した。
  - [x] `main` のtoolchain設定を維持した。
  - [x] 英語READMEに出典、疑似コード、枝刈り、正確性、制約を記載した。
  - [x] 共通の33正解ケースと4 benchmark targetを組み込んだ。
- [x] `test/xiangting` の比較アダプタを、DFSとの重複差分を除いて統合した。
  - [x] `xiangting = 6.0.1` を固定した。
  - [x] 一般形・七対子・国士の最小値とvalidationを含む条件を明記した。
- [x] `sac_shanten_dp` を追加した。
  - [x] `shanten-dp = 0.3.2` を固定した。
  - [x] `make_tile_limits(false)`、`Mode::STANDARD`、2つのboolをfalseに設定した。
  - [x] `m = sum(hand) / 3` とし、代表手のsmoke testを追加した。
- [x] 外部crate READMEの順序を「用途、upstream・固定版、license、設定、対象、validation、結果、コマンド」に統一した。
- [x] 比較モードが同一でないことを各adapter READMEに記載した。
- [x] `Cargo.lock` と `THIRD-PARTY-NOTICES.md` に固定版とlicenseを記録した。

受入条件:

- [x] 両adapterで4種類の共通benchmarkを生成できる。
- [x] 固定バージョン、設定、対象範囲、validation、結果の意味を追跡できる。

## P1 — package設定、テスト基盤、lint、CI

依存: P0のworkspace memberが確定していること。

- [x] `[workspace.package] publish = false` と全memberの `publish.workspace = true` を設定した。
- [x] アルゴリズムcrateは `lib.rs` を維持し、calculator型を `pub` にした。
- [x] 内部補助型・関数はprivateのまま維持した。
- [x] `dummy` を共通正解テストから外し、常に0を返す専用テストへ変更した。
- [x] 名前付きprofileと `reason = "..."` で既知の不正解を明示し、厳密解の実装にはignoreを置かなかった。
- [x] Rust warnings、Clippy `all`、Clippy `pedantic` をworkspace全体でdenyした。
- [x] `allow` を追加せず、コード修正で警告を解消した。
- [x] `rustfmt.toml` でunstable featuresと `StdExternalCrate` を設定した。
- [x] toolchainを `nightly-2026-08-21` に固定した。
- [x] fmt、Clippy、test、publish禁止を検査するGitHub Actionsを追加した。

受入条件:

- [x] `cargo metadata` 上で全memberのpublish配列が空である。
- [x] `cargo fmt --all -- --check` が成功する。
- [x] `cargo clippy --workspace --all-targets --all-features` が成功する。
- [x] `cargo test --workspace` が成功する。

## P2 — handgenと全実装のリファクタリング

依存: P1のテスト・lint基盤。

- [x] `handgen` の牌IDを右寄せ2桁幅にした。
- [x] 固定seedで4ファイルを再生成した。
- [x] 各ファイル10,000行、各行14要素、値域0–33、再生成一致を確認した。
- [x] resourcesの大規模差分を独立コミットにした。
- [x] 共通型、テスト・benchmark macro、ファイル解析、handgenを厳格lintに合わせて整理した。
- [x] 全calculator型と境界の型変換を整理した。
- [x] 原典由来アルゴリズムの探索順序、枝刈り、状態表現を過度に変更していない。

受入条件:

- [x] workspace testとClippyで全crateを検証できる。
- [x] アルゴリズム変更は機械的整理または明示したバグ修正に限定されている。

## P3 — mjai-manue-go由来の向聴数専用DFS

依存: P0の2つの厳密実装、P1の共通テスト、P2の固定resource。

- [x] `pruning_dfs_mjai_manue_go` を独立crateとして追加した。
- [x] `mjai-manue-go v0.3.0-beta.5` のcommit `1ead84275f75d1b4aafe68a6c6c6867e107379cb` に由来を固定した。
- [x] 面子一覧の再利用、探索距離の差分更新、刻子重複防止、共通牌のない候補の枝刈りを移植した。
- [x] 4枚使いと牌枚数境界を共通回帰テストで確認した。
- [x] 七対子、国士、Goal列挙、`AllowedExtraTiles`、`UpperBound` を追加していない。
- [x] 共通正解ケースをignoreなしで通した。
- [x] 4万手を `pruning_dfs_ymatsux` と `decomp_fixed_pruned` に差分がないことを確認した。
- [x] 4種類の共通benchmarkを追加した。
- [x] 由来とBSD-3-Clause noticeを記録した。
- [-] 既知バグを持つGimite原典版の忠実移植は保留する。

受入条件:

- [x] 通常テストは高速で、4万手差分テストを明示的に実行できる。
- [x] upstreamとの差とGoal allocationを含めない条件がREADMEから分かる。

## P4 — README刷新

依存: P0–P3の比較対象と検証結果が確定していること。

- [x] 「目的と対象範囲、正確性・制約、実行方法、追加方法、license」の順に再構成した。
- [x] 比較表、DFSの実装詳細、外部crateの比較条件はルートREADMEに置かず、個別READMEへ集約した。
- [x] ルートREADMEにpublish可否や外部利用方針を記載していない。
- [x] プロジェクト構成用Mermaidを作成していない。
- [x] アルゴリズム説明は疑似コードを基本とした。

## 保留バックログ

- [-] Gimite原典版DFS: 4枚使いの既知バグがあるため、修正方針と比較目的が確定するまで追加しない。

## プロジェクト完了条件

- [x] 優先度、依存、状態、受入条件をこのROADMAPで管理できる。
- [x] 全workspace memberがpublish禁止である。
- [x] fmt、厳格Clippy、workspace testがCIの必須検査である。
- [x] 厳密解の共通正解テストにignoreがない。
- [x] 既知不正解のテストID、理由、README上の制約が対応している。
- [x] 外部crateと移植コードの固定版、設定、license、比較条件を追跡できる。
- [x] 全アルゴリズムと外部adapterが4種類のbenchmarkを持つ。
