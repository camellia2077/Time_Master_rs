import subprocess
import shutil
import os
import re
import sys

# --- 全局配置 ---
# 定义可执行文件的路径
CPP_EXECUTABLE = "log_generator.exe"
RUST_EXECUTABLE = "log_generator_crate.exe"

# 定义命令行参数
ARGUMENTS = ["1900", "4000", "40"]

# 定义每个程序循环运行的次数
RUN_COUNT = 10

# 定义要清理的文件夹名称
DATE_DIR = "Date"
# 定义输出结果的文件名
OUTPUT_FILENAME = "benchmark.txt"
# --- 配置结束 ---


def cleanup_directory():
    """如果Date文件夹存在，则删除它"""
    if os.path.exists(DATE_DIR):
        try:
            shutil.rmtree(DATE_DIR)
        except OSError as e:
            print(f"Error: Failed to remove directory {DATE_DIR}: {e}")
            sys.exit(1)


def parse_output(output_text):
    """从程序输出中解析时间统计（单位：毫秒）"""
    try:
        pattern = re.compile(r":\s*[\d.]+\s*s\s*\((\d+)ms\)")
        matches = pattern.findall(output_text)
        
        if len(matches) == 3:
            total_ms = int(matches[0])
            generate_ms = int(matches[1])
            io_ms = int(matches[2])
            return {"total": total_ms, "generate": generate_ms, "io": io_ms}
        else:
            print(f"Warning: Could not parse output correctly.\nOutput:\n{output_text}")
            return None
    except (ValueError, IndexError) as e:
        print(f"Warning: Error parsing output: {e}\nOutput:\n{output_text}")
        return None


def run_benchmark(executable_path, run_count):
    """运行指定的可执行文件N次并收集时间数据"""
    if not os.path.exists(executable_path):
        print(f"Error: Executable not found at '{executable_path}'")
        return None

    results = {"total": [], "generate": [], "io": []}
    
    print(f"\nRunning benchmark for '{executable_path}' ({run_count} iterations)...")

    for i in range(run_count):
        print(f"  Iteration {i + 1}/{run_count}...", end='\r')
        
        cleanup_directory()
        
        try:
            process = subprocess.run(
                [executable_path] + ARGUMENTS,
                capture_output=True,
                text=True,
                check=True,
                encoding='utf-8'
            )
            
            times = parse_output(process.stdout)
            if times:
                results["total"].append(times["total"])
                results["generate"].append(times["generate"])
                results["io"].append(times["io"])

        except FileNotFoundError:
            print(f"\nError: Command not found '{executable_path}'. Please check the path.")
            return None
        except subprocess.CalledProcessError as e:
            print(f"\nError running {executable_path}:")
            print(e.stderr)
            return None
        except Exception as e:
            print(f"\nAn unexpected error occurred: {e}")
            return None

    print(f"\nBenchmark for '{executable_path}' complete.")
    return results


def format_results(results_data, lang_name, executable_name):
    """计算平均时间和总时间，并返回格式化的字符串"""
    if not results_data or not results_data["total"]:
        return f"No valid data to report for {executable_name}."

    run_count = len(results_data["total"])
    
    # 计算总毫秒数
    sum_total_ms = sum(results_data["total"])

    # 计算平均毫秒数
    avg_total_ms = sum_total_ms / run_count
    avg_generate_ms = sum(results_data["generate"]) / run_count
    avg_io_ms = sum(results_data["io"]) / run_count

    # 将时间转换为秒
    sum_total_s = sum_total_ms / 1000.0
    avg_total_s = avg_total_ms / 1000.0
    avg_generate_s = avg_generate_ms / 1000.0
    avg_io_s = avg_io_ms / 1000.0

    # 构建结果字符串
    report = []
    # MODIFIED: 添加了语言标识 (C++/Rust)
    report.append(f"--- Results for {lang_name} ({executable_name}, {run_count} runs) ---")
    report.append("--- Average Times ---")
    report.append("---------------------------")
    report.append(f"total time:    {avg_total_s:6.2f} s ({int(avg_total_ms)}ms)")
    report.append(f"text generate: {avg_generate_s:6.2f} s ({int(avg_generate_ms)}ms)")
    report.append(f"io:            {avg_io_s:6.2f} s ({int(avg_io_ms)}ms)")
    report.append("---------------------------")
    report.append("--- Total Time ---")
    report.append(f"Sum of all 'total time' runs: {sum_total_s:.2f} s ({sum_total_ms}ms)")
    
    return "\n".join(report)


def main():
    """主函数"""
    print("Starting benchmarks...")
    
    cpp_results = run_benchmark(CPP_EXECUTABLE, RUN_COUNT)
    rust_results = run_benchmark(RUST_EXECUTABLE, RUN_COUNT)

    final_report_parts = []
    if cpp_results:
        # MODIFIED: 传入 "C++" 标识
        final_report_parts.append(format_results(cpp_results, "C++", CPP_EXECUTABLE))

    if rust_results:
        # MODIFIED: 传入 "Rust" 标识
        final_report_parts.append(format_results(rust_results, "Rust", RUST_EXECUTABLE))

    # 将所有结果写入文件
    try:
        with open(OUTPUT_FILENAME, 'w', encoding='utf-8') as f:
            f.write("\n\n".join(final_report_parts))
        print(f"\nBenchmark results have been saved to '{OUTPUT_FILENAME}'")
    except IOError as e:
        print(f"\nError: Failed to write results to file: {e}")


if __name__ == "__main__":
    main()