from __future__ import annotations

import subprocess
from multiprocessing import cpu_count
from pathlib import Path
from math import log2
import sys
import joblib
import pandas as pd

from common.settings import TEST_PATH

tle_time = 3.0
test_case_num = 100

args = sys.argv
code_name = "a"
if len(args) >= 2:
    code_name = args[1]
print(f"code name: {code_name}.rs")

color_dic = {"white": "\033[37m", "black": "\033[30m", "red": "\033[31m", "green": "\033[32m", "yellow": "\033[33m", "blue": "\033[34m", "end": "\033[0m"}


def print_color(text, color="black"):
    print(color_dic[color] + text + color_dic["end"])


def worker(n):
    path = Path(TEST_PATH)
    filename = str(n).zfill(4)
    in_file = f"tools/in/{filename}.txt"
    out_file = f"tools/out/{filename}.txt"
    cmd = f"cargo run -r --features local --bin {code_name} < {in_file} > {out_file}"
    proc = subprocess.Popen(cmd, shell=True, cwd=path, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stderr_list = proc.communicate()

    score = 0
    elapsed_time = 0
    count = 0
    # print(stderr_list)
    
    for stderr in stderr_list:
        out_list = stderr.decode().split("\n")
        for out in out_list:
            if "score" in out.lower():
                score = int(out.split()[1])
            if "elapsed" in out.lower():
                elapsed_time = round(float(out.split()[1]), 2)
            if "count" in out.lower():
                count = int(out.split()[1])


    out = f"{filename} score: {score}, count: {count}, elapsed: {elapsed_time}sec"
    color = "white"
    if elapsed_time >= tle_time:
        out += ", TLE"
        color = "yellow"
    print_color(out, color)

    return [score, log2(score)]


def main():
    if TEST_PATH is None:
        return
    cores = cpu_count()
    data = joblib.Parallel(n_jobs=cores)(joblib.delayed(worker)(i) for i in range(test_case_num))
    df = pd.DataFrame(data=data, columns=["score", "log_score"])
    score_sum = df["score"].sum()
    log_score_sum = df["log_score"].sum()
    print(f"Score total: {score_sum}")
    print(f"Log score total: {log_score_sum}")
    df.to_csv(f"{code_name}.csv")


if __name__ == "__main__":
    main()
