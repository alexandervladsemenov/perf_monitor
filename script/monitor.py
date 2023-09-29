import os
from matplotlib import pyplot as plt
import re
import numpy as np
import argparse
path_format = "Time: 2240, Cpu usage: 0, memory usage: 119.546875 MB, disk util (0, 26071040, 0, 323584)"


def isfloat(element: str):
    try:
        float(element)
        return True
    except ValueError:
        return False


def get_val(line: str) -> list[float]:
    new_st = line.replace('(', ' ')
    new_st = new_st.replace(')', ' ')
    _data_ = re.split(', | ', new_st)
    numeric = [float(s) for s in _data_ if isfloat(s)]
    return numeric


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--ifile", nargs='*', type=str,
                        help="Provide a list of files to analyze",
                        default=None)
    files = [os.path.join("../", x)
             for x in os.listdir("../") if ".txt" in x and "log" in x]
    args = parser.parse_args()
    if args.ifile:
        files = args.ifile
    print(files)
    for file in files:
        time = []
        mem = []
        cpu_usage = []
        write_bytes = []
        read_bytes = []
        total_write = []
        total_read  = []
        res1 = file.find("_name_")
        res2 = file.find(".txt")
        res3 = file.find("log_pid_")
        p_name = file[res1 + len("_name_"):res2]
        pid = file[res3 + len("log_pid_"):res1]
        with open(file, mode="r") as f:
            lines = f.readlines()
            for line in lines:
                data = get_val(line)
                time.append(data[0])
                cpu_usage.append(data[1])
                mem.append(data[2])
                write_bytes.append(data[3])
                read_bytes.append(data[5])
                total_write.append(data[4])
                total_read.append(data[6])
        time = np.array(time)
        mem = np.array(mem)
        cpu_usage = np.array(cpu_usage)
        write_bytes = np.array(write_bytes,dtype=np.float)
        total_write = np.array(total_write,dtype=np.float)
        read_bytes = np.array(read_bytes,dtype=np.float)
        total_read = np.array(total_read,dtype=np.float)
        plt.plot(time, mem)
        plt.title("Memory usage")
        plt.xlabel("Time, ms")
        plt.ylabel("Memory RAM, MG")
        plt.show()
        plt.close()
        plt.plot(time, cpu_usage)
        plt.title("CPU usage")
        plt.xlabel("Time, ms")
        plt.ylabel("CPU usage, %")
        plt.show()
        plt.close()
        plt.plot(time, write_bytes/1024.0)
        plt.title("Written bytes")
        plt.xlabel("Time, ms")
        plt.ylabel("Bytes written, KB")
        plt.show()
        plt.close()
        plt.plot(time, read_bytes/1024.0)
        plt.title("Read bytes")
        plt.xlabel("Time, ms")
        plt.ylabel("Bytes read, KB")
        plt.show()
        plt.close()
        plt.plot(time, total_read/1024.0/1024.0)
        plt.title("Total read bytes")
        plt.xlabel("Time, ms")
        plt.ylabel("Bytes read, MB")
        plt.show()
        plt.close()
        plt.plot(time, total_write/1024.0/1024.0)
        plt.title("Total written bytes")
        plt.xlabel("Time, ms")
        plt.ylabel("Bytes written, MB")
        plt.show()
        plt.close()
    pass
