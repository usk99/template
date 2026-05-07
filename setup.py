import os
import re

# Cargo.toml を読み込む
toml_path = "rust/Cargo.toml"

if os.path.exists(toml_path):
    with open(toml_path, "r", encoding="utf-8") as f:
        content = f.read()

    # 現在のパスからプロジェクト名を作成
    # 例: .../abc300/a  -> contest="abc300", task="a" -> name="abc300_a"
    cwd = os.getcwd()
    task = os.path.basename(cwd)          # "a"
    contest = os.path.basename(os.path.dirname(cwd))  # "adt_easy_20251210_1"
    new_name = f"{contest}_{task}"

    # Rustの識別子として有効な名前に変換 (ハイフンをアンダースコアに)
    # Cargo.tomlのnameはハイフンOKだが、コード内のuseではアンダースコアにする必要がある
    new_package_id = new_name.replace("-", "_")
    name_match = re.search(r'^name\s*=\s*"(.*?)"', content, re.MULTILINE)
    if name_match:
        package_name = name_match.group(1)
        # コード内で使われている古い名前（ハイフン変換済み）を特定
        old_package_id = package_name.replace("-", "_")
        content = re.sub(r'^name\s*=\s*".*?"',
                         f'name = "{new_name}"', content, count=1, flags=re.MULTILINE)

        # Cargo.toml を保存
        with open(toml_path, "w", encoding="utf-8") as f:
            f.write(content)

        # main.rs と main_heuristic.rs の use 文を書き換え
        for rs_file in ["rust/src/main.rs", "rust/src/main_heuristic.rs"]:
            if os.path.exists(rs_file):
                with open(rs_file, "r", encoding="utf-8") as f:
                    rs_content = f.read()

                # 古いパッケージ名を新しいパッケージ名に置換
                rs_content = rs_content.replace(
                    f"use {old_package_id}::", f"use {new_package_id}::")

                with open(rs_file, "w", encoding="utf-8") as f:
                    f.write(rs_content)

# go のテストファイルを削除
import glob
for f in glob.glob("go/*_test.go"):
    os.remove(f)

# rust のテストファイルを削除
for f in glob.glob("rust/src/library/*_test.rs"):
    os.remove(f)

# mod.rs から #[cfg(test)] ブロックを削除
mod_rs = "rust/src/library/mod.rs"
if os.path.exists(mod_rs):
    with open(mod_rs, encoding="utf-8") as f:
        lines = f.readlines()
    lines = [l for l in lines if not l.startswith("#[cfg(test)]") and not l.startswith("#[path =")]
    with open(mod_rs, "w", encoding="utf-8") as f:
        f.writelines(lines)

# このスクリプト自身を削除
os.remove(__file__)
