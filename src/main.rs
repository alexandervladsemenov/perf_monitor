use clap::{Arg, ArgAction, Command};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::process;
use std::sync::Arc;
use std::time::Instant;
use sysinfo::{DiskUsage, Pid, ProcessExt, System, SystemExt};
use tokio;
use tokio::sync::Mutex;
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

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

async fn read_command(process_name: Arc<Mutex<String>>) -> bool {
    loop {
        if let Ok(res) = read_line() {
            let trimmed = res.trim();
            if trimmed.eq("END") {
                println!(
                    "ENDING THE PROGRAM 🥰 😘 😗 😙 😚 😋 😛 😝 😜 🤪 🤨 🧐 🤓 😎 🥸 🤩 🥳 😏",
                );
                return true;
            } else {
                println!("The user requested the process {}", trimmed);
                *process_name.lock().await = trimmed.to_string();
            }
        }
    }
}

async fn write_log(
    mon: &mut Monitor,
    start_time: &Instant,
    file: &mut File,
    flag: Arc<Mutex<i32>>,
) {
    'outer: loop {
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
            break 'outer;
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

        let total_sleep_time = 50u64;
        let mut current_sleep = 0u64;
        let increment = 5u64;
        while current_sleep < total_sleep_time {
            let lock = flag.lock().await;
            if *lock > 0 {
                println!("Stop Monitoring: 👻 💀 ☠️ 👽 👾 🤖 🎃 😺 😸 😹 😻 😼 😽 🙀 😿 😾");
                break 'outer;
            }
            sleep(Duration::from_millis(increment)).await;
            current_sleep += increment;
        }
    }
}

async fn run_monitor(process_name: &str, flag: Arc<Mutex<i32>>) {
    sleep(Duration::from_millis(1)).await; // make sure that the process is running

    if let Some(mut mon) = Monitor::new(process_name) {
        let pid = mon.pid.to_string();
        println!("Running process {} and process id {:?}", process_name, pid);
        let start_time = Instant::now();
        let filename = format!("log_pid_{pid}_name_{process_name}.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        let writer_f = write_log(&mut mon, &start_time, &mut file, flag);
        writer_f.await;
    } else {
        println!("Process {} not found 😍😍😍😍\n", process_name);
    }
}
#[tokio::main]
async fn main() {
    let ret = parse_cli();
    let flag1 = Arc::new(Mutex::new(0));
    let flag2 = Arc::clone(&flag1);
    let process1 = Arc::new(Mutex::new("".to_string()));
    let process2 = Arc::clone(&process1);
    // println!("{:?}",ret);
    let handle_read = tokio::spawn(async move {
        // Do some async work
        let res = read_command(process2).await;
        if res {
            let mut lock = flag2.lock().await;
            *lock += 1;
        }
    });

    // let handle_something_to_run = tokio::spawn(async {loop{sleep(Duration::from_millis(3000)).await;println!("We are working");}}) ;

    let handle_monitor = tokio::spawn(async move {
        // Do some async work
        if let Some(process_name) = ret.0 {
            run_monitor(&process_name, flag1).await;
        } else {
            let flag4 = Arc::clone(&flag1);
            let mut savedprocessedname : String = "".to_string();
            while *(flag4.lock().await) == 0 {
                // println!("lockr is {}",lockr);
                let flag3 = Arc::clone(&flag1);
                let process_name = (*process1.lock().await).clone();
                if process_name.len() > 0 && (process_name != savedprocessedname) {
                    println!("The user wants {} and {}", process_name,savedprocessedname);
                    savedprocessedname = process_name.clone();
                    run_monitor(&process_name, flag3).await;
                }
                sleep(Duration::from_millis(500)).await;
            }
        }
    });
    let res1 = handle_monitor.await;
    let res2 = handle_read.await;
    // if handle_something_to_run.await.is_err() {};
    if res1.is_err() {
        println!("Something went wrong with the monitor");
    }
    if res2.is_err() {
        println!("Something went wrong with the reader");
    }
}
