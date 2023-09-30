use tokio::process::Command;
use std::{path::PathBuf, process::ExitStatus};
pub async fn run_process(process_path: &PathBuf, args: &Vec<String>) -> (Vec<u8>, Vec<u8>, ExitStatus) {
    let process = Command::new(process_path)
        .args(args).kill_on_drop(true).output();
    
    let status = process.await.unwrap();
    (status.stdout, status.stderr, status.status)
}