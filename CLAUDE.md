# AHC0XX A

## 参照ファイル

- 問題仕様: `docs/spec.md`（制約・入出力・スコア式）
- 解法方針: `docs/plan.md`（**実装前に必ず読む**）
- 実験ログ: `docs/log.md`（変更の採否を決めたら追記する）
- 問題文全文: `docs/statement_full.txt`（仕様に疑問が出たときだけ開く）

## 実行手順

```bash
cd rust && pahcer run          # ビルドとテスト
cd .. && python3 scripts/summarize.py   # 結果の要約（8行）
```

- 結果の判断は `summarize.py` の出力だけで行う
- `rust/pahcer/json/` の生JSONは開かない（1件17KB程度あるため）

## 読まないファイル

- `tools/` は公式配布物。読まない・編集しない
- `rust/target/`、`rust/ahc***_a`（バイナリ）、`*.html`
- `memo/` は競プロ全般のメモ。この問題には関係しない

## この問題固有のメモ

（実装中に判明した仕様の罠・実装上の制約をここに追記する）
