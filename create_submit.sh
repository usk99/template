base_file=./rust/src/main.rs
submit_file=./rust/src/submit.rs
library_dir=./rust/src/library
# ディレクトリ名からクレート名を取得 (例: abc400/a -> abc400_a)
crate_name=$(basename "$(dirname "$PWD")")_$(basename "$PWD")

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
cat $submit_file | xsel -bi

rm -f $submit_file
#echo -e "\r\033[k提出用ファイルを生成&コピーしました"
#du -h $submit_file
echo -e "\r\033[k提出用ファイルをコピーしました"
echo "念のため動作確認をしましょう"