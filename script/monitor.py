import subprocess
from matplotlib import pyplot as plt
import re
path_format = "Time: 2240, Cpu usage: 0, memory usage: 119.546875 MB, disk util (0, 26071040, 0, 323584)"

if __name__ == "__main__":
    new_st = path_format.replace('(',' ')
    new_st = new_st.replace(')',' ')
    data = re.split(', | ',new_st)
    data = [s for s in data if s.isnumeric()]
    print(data)
    pass