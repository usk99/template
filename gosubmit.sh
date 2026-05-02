#!/bin/bash
go_dir=./go
submit_file=$go_dir/submit.go
SAVE_FILE=${1:-false}

echo -ne "提出用ファイルを生成します"

# ファイル結合順: Main.goを先頭に、それ以外を後に
files=("$go_dir/Main.go")
for f in "$go_dir"/*.go; do
    [[ "$f" == "$go_dir/Main.go" ]] && continue
    [[ "$f" == "$go_dir/submit.go" ]] && continue
    [[ "$f" == *_test.go ]] && continue
    files+=("$f")
done

# 各ファイルから package宣言とimportブロックを除去して結合
contents=""
for f in "${files[@]}"; do
    contents+=$(perl -0777 -pe '
        s/^package\s+\S+\s*\n//m;
        s/^import\s*\(.*?\)\s*\n//ms;
        s/^import\s+"[^"]+"\s*\n//mg;
        s{^//\s*={10,}.*?^//\s*={10,}[^\n]*\n}{}msg;
        s{^\s*//[^\n]*\n}{}mg;
    ' "$f")
    contents+=$'\n'
done

# package main + goimports で import を自動補完
tmp=$(mktemp --suffix=.go)
printf "//go:build !test\n\npackage main\n\n%s" "$contents" > "$tmp"
~/go/bin/goimports -w "$tmp"
contents=$(tail -n +1 "$tmp")
rm -f "$tmp"

printf "%s" "$contents" > "$submit_file"
cat "$submit_file" | xsel -bi
printf "\r\033[K"
du -b "$submit_file" | awk '{printf "%'"'"'d bytes (%.2f KB)\t%s\n", $1, $1/1024, $2}'
rm -f "$submit_file"
echo "提出用ファイルをコピーしました"
echo "念のため動作確認をしましょう"
