use clap::{Arg, ArgAction, Command};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::process;
use std::time::Instant;
use sysinfo::{DiskUsage, Pid, ProcessExt, System, SystemExt};
use tokio;
use tokio::time::{sleep, Duration};
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

async fn write_log(mon : & mut Monitor, start_time : &Instant, file : & mut File)
{
    loop {
        let cpu_usage: f32;
        let memory_usage: u64;
        let disk_info: (u64, u64, u64, u64);
        let res = mon.monitor();
        if let Some((mem, cpu, diskutil)) = res {
            cpu_usage = cpu;
            memory_usage = mem;
            disk_info = (
                diskutil.read_bytes,
                diskutil.total_read_bytes,
                diskutil.written_bytes,
                diskutil.total_written_bytes,
            );
        } else {
            println!("Process has ended 💖💖💖💖💖\n");
            break;
        }
        let end = Instant::now();
        let duration = end.duration_since(*start_time);
        let res = writeln!(
            file,
            "Time: {:?}, Cpu usage: {}, memory usage: {} MB, disk util {:?}",
            duration.as_millis(),
            cpu_usage,
            memory_usage as f32 / 1024.0 / 1024.0,
            disk_info
        );

        if res.is_err() {
            println!("Error : can't write the file");
            process::exit(0x0100);
        }
        sleep(Duration::from_millis(50)).await;
    }
}

async fn run_monitor(process_name: &str) {
    sleep(Duration::from_millis(1)).await; // make sure that the process is running

    if let Some(mut mon) = Monitor::new(process_name) {
        let pid = mon.pid.to_string();
        println!(
            "Running process {} and process id {:?}",
            process_name, pid
        );
        let start_time = Instant::now();
        let filename = format!("log_pid_{pid}_name_{process_name}.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        let writer_f = write_log(&mut mon, &start_time, &mut file);
        writer_f.await;
    } else {
        println!("Process {} not found 😍😍😍😍\n", process_name);
    }
}
#[tokio::main]
async fn main() {
    let ret = parse_cli();
    // println!("{:?}",ret);
    if let Some(process_name) = ret.0 {
        let f = run_monitor(&process_name);
        f.await;
    };
}
