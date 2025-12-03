import os
import toml

# Cargo.toml を読み込む
toml_path = "rust/Cargo.toml"
package_name = ""

if os.path.exists(toml_path):
    with open(toml_path, "r") as f:
        data = toml.load(f)

    # 現在のパスからプロジェクト名を作成
    # 例: .../abc300/a  -> contest="abc300", task="a" -> name="abc300_a"
    cwd = os.getcwd()
    task = os.path.basename(cwd)          # "a"
    contest = os.path.basename(os.path.dirname(cwd))  # "abc300"

    new_name = f"{contest}_{task}"

    # name を書き換え
    package_name = data["package"]["name"]
    data["package"]["name"] = new_name

    # 保存
    with open(toml_path, "w") as f:
        toml.dump(data, f)

    print(f"Updated package name to: {new_name}")

    # main.rs の use 文を書き換え
    main_rs_path = "rust/src/main.rs"
    if os.path.exists(main_rs_path):
        with open(main_rs_path, "r") as f:
            content = f.read()

        # 古いパッケージ名を新しいパッケージ名に置換
        content = content.replace(f"use {package_name}::", f"use {new_name}::")

        with open(main_rs_path, "w") as f:
            f.write(content)

        print(f"Updated main.rs: {package_name} -> {new_name}")

# このスクリプト自身を削除 (ゴミを残さないため)
os.remove(__file__)
