# ==========================================
# 設定とファイルパスの定義
# ==========================================
base_file=./rust/src/main.rs
submit_file=./rust/src/submit.rs
library_dir=./rust/src/library
# true: 提出用ファイルを作成してからコピー, false: 直接クリップボードにコピーのみ
SAVE_FILE=${1:-false}

echo -ne "提出用ファイルを生成します"

# ==========================================
# main.rs の処理
# ==========================================
contents=""
# `use library::xxx::*;` 行と `mod library;` 行を除外して提出ファイルに追加
contents=$(cat "$base_file" | grep -v "^use library::" | grep -v "^mod library;")
# `use library::xxx::*;` から xxx (モジュール名) を抽出
items=(`cat "$base_file" | grep "^use library::" | sed -E "s/use library::(.*)::\*;(.*)/\1/"`)

# ==========================================
# algorithm 関数の退避
# ==========================================
algorithm_file=$(mktemp)
algorithm_saved=false

contents=$(printf '%s' "$contents" | python3 -c '
import sys

path = sys.argv[1]
src = sys.stdin.read()

begin = "// BEGIN_KEEP_ALGORITHM\n"
end = "// END_KEEP_ALGORITHM\n"

# マーカーが両方ない場合は従来どおり処理
if begin not in src and end not in src:
	sys.stdout.write(src)
	sys.exit(0)

# マーカーの数を確認
if src.count(begin) != 1 or src.count(end) != 1:
	raise SystemExit("algorithm の退避マーカーが不正です")

a = src.index(begin)
b = src.index(end, a) + len(end)

if b <= a:
	raise SystemExit("algorithm のマーカー位置が不正です")

# マーカーの内側をそのまま保存
algorithm = src[a + len(begin):b - len(end)]

with open(path, "w") as f:
	f.write(algorithm)

# マーカーごとプレースホルダーに置換
sys.stdout.write(src[:a] + "__KEEP_ALGORITHM_PLACEHOLDER__" + src[b:])
' "$algorithm_file") || {
	rm -f "$algorithm_file"
	exit 1
}

# 空の関数本体でも正しく判定できるよう、マーカーの有無で判定
if grep -q "__KEEP_ALGORITHM_PLACEHOLDER__" <<< "$contents"; then
	algorithm_saved=true
fi

# ==========================================
# ライブラリファイルの結合 (インライン化)
# ==========================================
for item in "${items[@]}" ; do
	# いったん整形
	rustfmt "$library_dir/$item.rs"
	
	# 整形したコードを文字列変数に格納
	contents+=$'\nmod '"$item"$' {\n'
	contents+="$(cat "$library_dir/$item.rs")"
	contents+=$'\n}'
done

# ==========================================
# テスト部分以外を抽出
# ==========================================
contents=$(echo "$contents" \
	| tr -d '\r' \
	| perl -0777 -pe '
		# 0. #[cfg(test)] mod xxx { ... } を再帰正規表現で正確に削除 (ネスト対応)
		s/#\[cfg\(test[^\]]*\)\]\s*mod\s+\w+\s*(\{(?:[^{}]++|(?1))*\})//gs;

		my @q;
		# 1. 文字列を退避させる
		s/("(?:[^"\\]|\\.)*"|\x27(?:\\u\{[^\}]+\}|\\.|[^\\\x27\n])\x27)/push(@q, $1), "___Q".$#q."___"/ges;

		# 2. コメント削除,末尾空白も削除
		s/\/\/.*//g;
		s/[ \t]+$//mg;

		# 3. 空行を削除
		s/^\s*\n//mg;

		# 4. カンマで終わる行を結合し、末尾カンマを削除
		s/\n\s*\././g;
		1 while s/,\s*\n\s*(?![}\)\]])/,/g;
		s/,\n/\n/g;

		# 5. where句の成形
		s/\s*where\s*/ where /g;

		# 6. 属性を1行にまとめる
		s/(#!*\[.+?\])\s*\n\s*/$1/g;

		# 7. 括弧を結合 (閾値なし: 最も大きい括弧を一括結合)
		# スペース詰め用サブルーチン
		# 受け取った文字列内の「改行」と「不要なスペース」を全て削除します
		my $minify = sub {
			my $s = shift;
			$s =~ s/\s*\n\s*/ /g;
			$s =~ s/ (?=[^\w\x27&|])(?<!&)|(?<=[^\w&|])(?!&) //g;
			return $s;
		};
		# メイン処理:
		# 正規表現 (?0) は「対応する括弧のネスト」を正しく解析し、
		# 「最も外側のブロック全体」にマッチします。
		# マッチしたブロック全体を $minify に渡すことで、中のネストごと一気に1行にします。
		# {} の処理
		s/(\{(?:[^{}]++|(?0))*\})/$minify->($1)/ge;
		# [] の処理
		s/(\[(?:[^\[\]]++|(?0))*\])/$minify->($1)/ge;
		# () の処理
		s/(\((?:[^()]++|(?0))*\))/$minify->($1)/ge;

		# 8. 全体の細かいスペース削除
		# s/\s*([}\)\]])/$1/g;
		# s/\s*\n/ /g;
		s/ (?=[^\w\x27&|])(?<!&)|(?<=[^\w&|])(?!&) //g;
		s/\$crate::library::[^:]+::/ crate::/g;
		s/use crate::library::/use crate::/g;

		# 999. 退避した文字列を復元する
		s/___Q(\d+)___/$q[$1]/ge;
	'\
	| sed -E 's/super::/crate::/g'\
	| sed -E ':a; /\\$/{N; s/\\\n\s*//; ba}'\
	| cat)

# ==========================================
# algorithm 関数の復元
# ==========================================
if [ "$algorithm_saved" = true ]; then
	contents=$(printf '%s' "$contents" | python3 -c '
import sys

path = sys.argv[1]
src = sys.stdin.read()

marker = "__KEEP_ALGORITHM_PLACEHOLDER__"

if src.count(marker) != 1:
	raise SystemExit("algorithm の復元位置が不正です")

with open(path) as f:
	algorithm = f.read()

sys.stdout.write(src.replace(marker, algorithm, 1))
' "$algorithm_file") || {
		rm -f "$algorithm_file"
		exit 1
	}
fi

rm -f "$algorithm_file"

# ==========================================
# クレートの一括呼び出し
# ==========================================
contents+=$'\nuse crate::{'$(IFS=,; echo "${items[*]/%/::*}")'};'

# ==========================================
# 参照URLを先頭に付与
# ==========================================
LIB_URL=${LIB_URL:-https://github.com/usk99/template}
contents="// library: ${LIB_URL}"$'\n'"$contents"

# ==========================================
# ファイル書き出し&コピー
# ==========================================
printf "%s" "$contents" > "$submit_file"
cat "$submit_file" | xsel -bi
printf "\r\033[K"
du -b "$submit_file" | awk '{printf "%'\''d bytes (%.2f KB)\t%s\n", $1, $1/1024, $2}'

# ==========================================
# 確認
# ==========================================
if [ "$SAVE_FILE" = false ]; then
	rm -f "$submit_file"
	echo "提出用ファイルをコピーしました"
else
	echo "提出用ファイルを作成&コピーしました"
fi
echo "念のため動作確認をしましょう"
