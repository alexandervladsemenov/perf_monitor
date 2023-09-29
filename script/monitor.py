import os
from matplotlib import pyplot as plt
import re
import numpy as np
path_format = "Time: 2240, Cpu usage: 0, memory usage: 119.546875 MB, disk util (0, 26071040, 0, 323584)"

def isfloat(element : str):
    try:
        float(element)
        return True
    except ValueError:
        return False
def get_val(line : str) -> list[float]:
    new_st = line.replace('(', ' ')
    new_st = new_st.replace(')', ' ')
    _data_ = re.split(', | ', new_st)
    numeric = [float(s) for s in _data_ if isfloat(s)]
    print(numeric)
    return numeric


if __name__ == "__main__":
    files = [os.path.join("../",x) for x in os.listdir("../") if ".txt" in x and "log" in x]
    print(files)
    for file in files:
        time = []
        mem = []
        cpu_usage = []
        res1 = file.find("_name_")
        res2 = file.find(".txt")
        res3 = file.find("log_pid_")
        p_name = file[res1 + len("_name_"):res2]
        pid = file[res3 + len("log_pid_"):res1]
        with open(file,mode="r") as f:
            lines = f.readlines()
            for line in lines:
                data = get_val(line)
                time.append(data[0])
                cpu_usage.append(data[1])
                mem.append(data[2])
        time = np.array(time)
        mem = np.array(mem)
        cpu_usage = np.array(cpu_usage)
        plt.plot(time,mem)
        plt.show()
        plt.close()
        plt.plot(time,cpu_usage)
        plt.show()
        plt.close()
    pass
