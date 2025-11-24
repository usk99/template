base_file=./src/main.rs
submit_file=./src/submit.rs
library_dir=./src/library
rm -f $submit_file


echo -ne "提出用ファイルを生成します"
# mod xxx; 以外を抜き出して提出ファイルに追加
cat $base_file | grep -v "^use rust::library::*" | sed '/^use rust::\*;/d' > $submit_file
# mod xxx; からxxxを抽出
items=(`cat $base_file | grep "^use rust::library::*" | sed -E "s/use rust::library::(.*)::\*;(.*)/\1/"`)

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

echo -e "\r\033[k提出用ファイルを生成&コピーしました"
du -h $submit_file
echo "念のため動作確認をしましょう"