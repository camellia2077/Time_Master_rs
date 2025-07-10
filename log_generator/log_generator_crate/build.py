import os
import subprocess
import sys
import shutil

def clean_target_directory(script_dir):
    """
    检查并删除 Rust 项目的 target 目录。
    """
    target_path = os.path.join(script_dir, "target")
    print(f"检查 target 目录: {target_path}")
    if os.path.exists(target_path) and os.path.isdir(target_path):
        print(f"发现 target 目录，正在删除: {target_path}...")
        shutil.rmtree(target_path)
        print("target 目录删除成功。")
    else:
        print("target 目录不存在或不是一个目录，无需删除。")
    return True

def build_rust_project():
    """
    切换到当前脚本所在的目录，删除 target 文件夹（如果存在），
    然后执行 'cargo build --release' 命令并实时打印输出。
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))
    print(f"当前脚本目录: {script_dir}")

    os.chdir(script_dir)
    print(f"已切换到目录: {os.getcwd()}")

    clean_target_directory(script_dir)

    print("正在执行 'cargo build --release' (实时输出)...")
    process = subprocess.Popen(
        ["cargo", "build", "--release"],
        stdin=subprocess.PIPE,
        stdout=sys.stdout,
        stderr=sys.stderr,
        text=True,
        bufsize=1,
        universal_newlines=True
    )

    return_code = process.wait()

    if return_code == 0:
        print("\n命令执行成功！")
    else:
        print(f"\n命令执行失败，退出码 {return_code}")

if __name__ == "__main__":
    build_rust_project()
