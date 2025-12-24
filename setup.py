import os
import re

# Cargo.toml を読み込む
toml_path = "rust/Cargo.toml"

if os.path.exists(toml_path):
    with open(toml_path, "r") as f:
        content = f.read()

    # 現在のパスからプロジェクト名を作成
    # 例: .../abc300/a  -> contest="abc300", task="a" -> name="abc300_a"
    cwd = os.getcwd()
    task = os.path.basename(cwd)          # "a"
    contest = os.path.basename(os.path.dirname(cwd))  # "adt_easy_20251210_1"
    new_name = f"{contest}_{task}"

    # package.name を正規表現で置換
    # name = "old_name" -> name = "new_name"
    name_match = re.search(r'^name\s*=\s*"(.*?)"', content, re.MULTILINE)
    if name_match:
        package_name = name_match.group(1)
        content = re.sub(r'^name\s*=\s*".*?"',
                         f'name = "{new_name}"', content, count=1, flags=re.MULTILINE)

        # Cargo.toml を保存
        with open(toml_path, "w") as f:
            f.write(content)

        # main.rs の use 文を書き換え
        main_rs_path = "rust/src/main.rs"
        if os.path.exists(main_rs_path):
            with open(main_rs_path, "r") as f:
                rs_content = f.read()

            # 古いパッケージ名を新しいパッケージ名に置換
            rs_content = rs_content.replace(f"use {package_name}::", f"use {new_name}::")

            with open(main_rs_path, "w") as f:
                f.write(rs_content)

# このスクリプト自身を削除
os.remove(__file__)
