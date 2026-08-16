#!/usr/bin/env python3
"""pahcer の結果 JSON を数行に要約する。

使い方:
    python3 scripts/summarize.py                  # 最新2件を比較
    python3 scripts/summarize.py A.json B.json    # A(前) と B(後) を比較
    python3 scripts/summarize.py --last           # 最新1件のみ表示

Max/Min は pahcer_config.toml の objective から自動判定する。
判定を上書きしたいときだけ --min / --max を付ける。
"""

import argparse
import json
import re
import sys
from pathlib import Path

DEFAULT_DIR = Path("rust/pahcer/json")
CONFIG_CANDIDATES = ["pahcer_config.toml", "rust/pahcer_config.toml",
                     "../pahcer_config.toml", "../../pahcer_config.toml"]


def detect_objective():
    """pahcer_config.toml から objective を読む。戻り値: ("Max"|"Min", 出典) or (None, None)"""
    for cand in CONFIG_CANDIDATES:
        path = Path(cand)
        if not path.is_file():
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        obj = None
        try:
            import tomllib
            obj = tomllib.loads(text).get("problem", {}).get("objective")
        except Exception:
            # tomllib が無い (Python < 3.11) か TOML が壊れている場合の保険
            m = re.search(r'^\s*objective\s*=\s*["\'](\w+)["\']', text, re.M)
            obj = m.group(1) if m else None
        if isinstance(obj, str) and obj.capitalize() in ("Max", "Min"):
            return obj.capitalize(), str(path)
    return None, None


def load(path):
    with open(path) as f:
        d = json.load(f)
    cases = {c["seed"]: c for c in d["cases"]}
    return d, cases


def find_latest(dirpath, n):
    files = sorted(Path(dirpath).glob("result_*.json"))
    if not files:
        sys.exit(f"[error] {dirpath} に result_*.json が見つかりません")
    return files[-n:]


def fmt_time(sec):
    return f"{sec * 1000:.0f}ms"


def main():
    p = argparse.ArgumentParser()
    p.add_argument("files", nargs="*", help="比較する JSON (前, 後)")
    p.add_argument("--dir", default=str(DEFAULT_DIR), help="結果 JSON のディレクトリ")
    p.add_argument("--last", action="store_true", help="最新1件のみ表示")
    g = p.add_mutually_exclusive_group()
    g.add_argument("--min", action="store_true", help="小さいほど良い (自動判定を上書き)")
    g.add_argument("--max", action="store_true", help="大きいほど良い (自動判定を上書き)")
    p.add_argument("--worst", type=int, default=5, help="表示する悪化ケース数")
    args = p.parse_args()

    if args.min or args.max:
        objective = "Min" if args.min else "Max"
        obj_src = "コマンドライン指定"
    else:
        objective, src = detect_objective()
        if objective is None:
            objective, obj_src = "Max", None
        else:
            obj_src = src

    better = (lambda a, b: a < b) if objective == "Min" else (lambda a, b: a > b)

    if args.files:
        paths = [Path(f) for f in args.files]
    elif args.last:
        paths = find_latest(args.dir, 1)
    else:
        paths = find_latest(args.dir, 2)

    # ---- 最新1件のみ ----
    if len(paths) == 1:
        d, cases = load(paths[0])
        n = len(cases)
        print(f"file    : {paths[0].name}")
        print(f"cases   : {n}")
        print(f"rel_mean: {d['total_relative_score'] / n:.3f}")
        print(f"max_time: {fmt_time(d['max_execution_time'])}")
        if d["wa_seeds"]:
            print(f"[WA]    : {d['wa_seeds']}")
        print("(比較対象がありません。1件のみ表示しました)")
        return

    (dp, dc), (np_, nc) = load(paths[0]), load(paths[1])

    common = sorted(set(dc) & set(nc))
    if not common:
        sys.exit("[error] 共通する seed がありません。比較できません。")

    only_prev, only_new = set(dc) - set(nc), set(nc) - set(dc)
    n = len(common)

    # 共通 seed のみで集計する（seed 範囲を変えた直後の誤判定を防ぐ）
    prev_rel = sum(dc[s]["relative_score"] for s in common) / n
    new_rel = sum(nc[s]["relative_score"] for s in common) / n

    win = lose = tie = 0
    deltas = []
    for s in common:
        a, b = dc[s]["score"], nc[s]["score"]
        if b == a:
            tie += 1
        elif better(b, a):
            win += 1
        else:
            lose += 1
        deltas.append((nc[s]["relative_score"] - dc[s]["relative_score"], s))

    max_time = max(nc[s]["execution_time"] for s in common)
    slow_seed = max(common, key=lambda s: nc[s]["execution_time"])

    print(f"prev    : {paths[0].name}")
    print(f"new     : {paths[1].name}")
    print(f"cases   : {n} (共通seedのみ)")
    print(f"rel_mean: {new_rel:.3f}  (prev {prev_rel:.3f}, {new_rel - prev_rel:+.3f})")
    print(f"win/lose/tie: {win} / {lose} / {tie}   勝率 {win / max(win + lose, 1):.0%}"
          f"  (objective={objective})")
    print(f"max_time: {fmt_time(max_time)} (seed {slow_seed})")

    worst = sorted(d for d in deltas if d[0] < 0)[: args.worst]
    if worst:
        s = ", ".join(f"seed{sd}({dl:+.2f})" for dl, sd in worst)
        print(f"worst   : {s}")

    best = sorted((d for d in deltas if d[0] > 0), reverse=True)[: args.worst]
    if best:
        s = ", ".join(f"seed{sd}({dl:+.2f})" for dl, sd in best)
        print(f"best    : {s}")

    ups = [d for d, _ in deltas if d > 0]
    downs = [d for d, _ in deltas if d < 0]
    if ups or downs:
        up_avg = sum(ups) / len(ups) if ups else 0.0
        down_avg = sum(downs) / len(downs) if downs else 0.0
        print(f"avg_gain: 改善時 {up_avg:+.2f} / 悪化時 {down_avg:+.2f}"
              f"  (無影響 {len(deltas) - len(ups) - len(downs)}件)")

    if nc_wa := np_["wa_seeds"]:
        print(f"[WA]    : {nc_wa}")
    if only_prev or only_new:
        print(f"[warn] seed集合が変化: prevのみ{len(only_prev)}件 / newのみ{len(only_new)}件")
    if obj_src is None:
        print("[warn] pahcer_config.toml が見つからず objective=Max と仮定した。"
              "Min の問題なら勝敗が逆になるため --min を付けること。")

    print("[note] relative_score は best_scores.json 基準のため、"
          "ベスト更新後は過去の値と厳密には比較できない。win/lose は生スコア基準。")


if __name__ == "__main__":
    main()
