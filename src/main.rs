use clap::{Arg, ArgAction, Command};
use std::time::Instant;
use sysinfo::{DiskUsage, Pid, ProcessExt, System, SystemExt};
use tokio;
use tokio::time::{sleep, Duration};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;
pub struct Monitor {
    sys: System,
    pid: Pid,
}
impl Monitor {
    pub fn new(process_name: &str) -> Option<Monitor> {
        let mut sys = System::new();
        sys.refresh_all();
        let pid: Pid;
        for (pid_p, process) in sys.processes() {
            if process.name() == process_name {
                pid = pid_p.clone();
                {
                    print!("Requested process {} has an id {}\n\n", process_name, pid);
                }
                return Some(Monitor { sys, pid });
            }
        }
        None
    }

    pub fn monitor(&mut self) -> Option<(u64, f32, DiskUsage)> {
        self.sys.refresh_all();
        let process = self.sys.process(self.pid);
        if let Some(proc) = process {
            Some((proc.memory(), proc.cpu_usage(), proc.disk_usage()))
        } else {
            None
        }
    }
}
fn parse_cli() -> (Option<String>, Vec<String>) {
    let matches = Command::new("perf")
        .version("1.01")
        .author("Alex Semenov")
        .about("Monitoring tools")
        .arg(Arg::new("args").action(ArgAction::Append))
        .arg(Arg::new("process").short('p').long("process"))
        .get_matches();
    let args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();
    if let Some(process) = matches.get_one::<String>("process") {
        (Some(process.to_owned()), args)
    } else {
        (None, args)
    }
}

async fn run_monitor(process_name: &str) {
    sleep(Duration::from_millis(1)).await; // make sure that the process is running
    let start_time = Instant::now();
    let mut file = OpenOptions::new()
    .create_new(true).write(true)
    .append(true)
    .open("log.txt")
    .unwrap();
    if let Some(mut mon) = Monitor::new(process_name) {
        println!("Running process {}", process_name);
        let mut cpu_usage: f32;
        let mut memory_usage: u64;
        let mut disk_info : (u64,u64,u64,u64);
        loop {
            let res = mon.monitor();
            if let Some((mem, cpu, diskutil)) = res {
                cpu_usage = cpu;
                memory_usage = mem;
                disk_info = (diskutil.read_bytes,diskutil.total_read_bytes,diskutil.written_bytes,diskutil.total_written_bytes);
            } else {
                println!("Process has ended 💖💖💖💖💖\n");
                break;
            }
            let end = Instant::now();
            let duration = end.duration_since(start_time);
            let res = writeln!(file,"Time: {:?}",duration.as_millis());
            if res.is_err()
            {
                println!("Error : can't write the file");
                process::exit(0x0100);
            }
            println!(
                "Time: {:?}, Cpu usage: {}, memory usage: {} MB, disk util {:?}",
                duration.as_millis(),
                cpu_usage,
                memory_usage as f32 / 1024.0 / 1024.0,disk_info
            );
            sleep(Duration::from_millis(50)).await;
        }
    } else {
        println!("Process {} not found 😍😍😍😍\n", process_name);
    }
}
#[tokio::main]
async fn main() {
    let ret = parse_cli();
    // println!("{:?}",ret);
    if let Some(process_name) = ret.0 
    {
        let f = run_monitor(&process_name);
        f.await;
    };
}
