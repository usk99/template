base_file=./rust/src/main.rs
submit_file=./rust/src/submit.rs
library_dir=./rust/src/library
# ディレクトリ名からクレート名を取得 (例: abc400/a -> abc400_a)
crate_name=$(basename "$(dirname "$PWD")")_$(basename "$PWD")
crate_name=${crate_name//-/_}

rm -f $submit_file

echo -ne "提出用ファイルを生成します"
# mod xxx; 以外を抜き出して提出ファイルに追加
cat $base_file | grep -v "^use ${crate_name}::library::*" | sed "/^use ${crate_name}::\*;/d" > $submit_file
# mod xxx; からxxxを抽出
items=(`cat $base_file | grep "^use ${crate_name}::library::*" | sed -E "s/use ${crate_name}::library::(.*)::\*;(.*)/\1/"`)

for item in "${items[@]}" ; do
# echo $item
    echo -e "\n" >> $submit_file
	
	# コード整形
	rustfmt $library_dir/$item.rs
	# コメント行と空行を除去して提出ファイルに追加
	echo mod $item{ >> $submit_file
	cat $library_dir/$item.rs | grep -vE "^\s*(#!|//|$)" | sed '/#\[cfg(test/,/^}/d' | sed -E 's/super::/crate::/'g>> $submit_file
	echo } >> $submit_file
	echo "use crate::$item::*;" >> $submit_file
done

rustfmt $submit_file

# 文字列圧縮（Rustの構文に影響のない範囲）
# エラー出たらここコメントアウト
#---------------------------------------------------------------------------------------
perl -i -pe 's/([^ ]) +\/\/.*/\1/' $submit_file
perl -i -pe 's/^ +\/\/.*//' $submit_file
for i in {0..5}; do
sed -i -e ':a;N;$!ba;s/\n[[:space:]]*\.\([^\.]\)/.\1/g' $submit_file
perl -i -0pe '1 while s/\(\n[ \t]*([^\n]+?)\n[ \t]*\)/($1)/g' $submit_file
perl -i -0pe '1 while s/\{\n[ \t]*([^\n]+?)\n[ \t]*\}/{$1}/g' $submit_file
sed -i -E ':a;N;$!ba;s/,\n([^,]*,)/,\1/g' $submit_file
sed -i -E ':a;N;$!ba;s/\{\n([^\n]*\})/{\1/g' $submit_file
sed -i -E ':a;N;$!ba;s/=\n([^\n]*;)/=\1/g' $submit_file
sed -i -E ':a;N;$!ba;s/\[\n([^\n]*\])/[\1/g' $submit_file
sed -i -E ':a;N;$!ba;s/\n(\/[^\/])/\1/g' $submit_file
perl -i -pe '
	my @quoted;
	# 引用符内を保存してプレースホルダーに置換
	s/("(?:[^"\\]|\\.)*")/push(@quoted, $1), "___Q" . $#quoted . "___"/ge;

	# 削除処理
	s/(?<=\S)\s*([,:;+\-*%<>=\{\}\[\]\(\)\^])\s*(?=\S)/$1/g;   # 演算子や記号周囲のスペース削除
	s/(\S)\s*([\}\]])/$1$2/g;                            # 閉じ括弧直前のスペース削除
	s/(\S),([\}\]])/$1$2/g;                              # 閉じ括弧直前のカンマ削除
	s/(\S)\s*\{/$1\{/g;                                  # 開き中括弧の後のスペース削除
	s/(\S)\s*\}/$1\}/g;                                  # 閉じ中括弧の後のスペース削除
	s/\{\s+(\S)/\{$1/g;                                  # 開き中括弧の前のスペース削除
	s/\}\s+(\S)/\}$1/g;                                  # 閉じ中括弧の前のスペース削除
	s/ +\/([^\/])/\/\1/g;                                # 削除
	s/([^\/]\/) */\1/g;                                  # 削除

	# プレースホルダーを元の文字列に復元
	s/___Q(\d+)___/$quoted[$1]/ge;
	' $submit_file
done
perl -i -pe 's/^\s*\n$// if /^\s*\n$/' $submit_file
perl -i -0pe 's/\n\s*\}/\}/g' $submit_file
perl -i -0pe 's/\n\s*return/return/g' $submit_file
perl -i -0pe 's/((?:^|\n)[^#\n]*[^#\n;])\s*\n\s*(?!.*;)(?!.*\/\/)([^\n]+)/$1 $2/g' $submit_file
perl minify_braces.pl
#---------------------------------------------------------------------------------------

cat $submit_file | xsel -bi

rm -f $submit_file
#echo -e "\r\033[k提出用ファイルを生成&コピーしました"
echo -e "\r\033[k提出用ファイルをコピーしました"
du -b $submit_file | awk '{printf "%'\''d bytes (%.2f KB)\t%s\n", $1, $1/1024, $2}'
echo "念のため動作確認をしましょう"