base_file=./src/bin/$1/main.rs
submit_file=./src/bin/submit.rs
rm -f $submit_file

# mod xxx; 以外を抜き出して提出ファイルに追加
cat $base_file | grep -v "^mod*" > $submit_file
# mod xxx; からxxxを抽出
items=(`cat $base_file | grep "^mod*" | sed -E "s/mod (.*);(.*)/\1/"`)

for item in "${items[@]}" ; do
    echo -e "\n" >> $submit_file

    # オプション
    # ビジュアライザのコードはローカルでのみ使用
    # 提出時にエラーにならないようにしておく
    if [ $item = "vis" ]; then
        echo '#[cfg(feature = "local")]' >> $submit_file
    fi

    echo "mod $item {" >> $submit_file
    cat ./src/bin/$1/$item.rs >> $submit_file
    echo "}" >> $submit_file
done

rustfmt $submit_file