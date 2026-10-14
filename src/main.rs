use std::{
    io::{BufRead, Stdin, Write, stdin, stdout},
    process::Command,
};
use sysinfo::{self, ProcessRefreshKind, RefreshKind, System, UpdateKind::OnlyIfNotSet, Users};

struct ExpectProcList {
    pid: u32,
    process: String,
    user: String,
}

fn main() {
    let system = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::nothing().with_user(OnlyIfNotSet)),
    );
    let mut users = Users::new();
    users.refresh();

    let proc_list = get_proc_list(system, users);
    for element in &proc_list {
        println!("{}, {}, {}", element.pid, element.process, element.user);
        stdout().flush().expect("FLUSH FAILED")
    }

    println!(" ");
    println!("Enter PID/Process name \n");

    let mut input = String::new();
    let stdin = &stdin();
    Stdin::lock(stdin)
        .read_line(&mut input)
        .expect("CANNOT READ INPUT");

    let input = input.trim().to_string();
    let match_process: Option<&ExpectProcList> = match input.parse::<u32>() {
        Ok(put_pid) => proc_list.iter().find(|list| put_pid == list.pid),
        Err(_) => proc_list.iter().find(|list| list.process == input),
    };

    println!(
        "You selected: {} {}\n",
        match_process.unwrap().process,
        match_process.unwrap().pid
    );

    exec_strace(match_process.unwrap().pid);
    exec_grep().expect("Err");
}

fn exec_strace(process: u32) {
    let strace_proc = Command::new("strace")
        .args([
            "-f",
            "-qqq",
            "-z",
            "-y",
            "-e",
            "trace=%file,connect,openat,open,sendto,read,write",
            "-o",
            "trace.log",
            "-p",
            &process.to_string(),
        ])
        .status()
        .expect("COMMAND FAILED");
    if strace_proc.success() {
        println!("Strace successfully exited!");
        println!("Formatting 'trace.log'");
    }
}

fn exec_grep() -> std::io::Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(r#"grep -oE '"/[^"]*"' trace.log | tr -d '"' | sort -u > paths.txt"#)
        .status()?;

    if !status.success() {
        eprintln!("pipeline failed: {status}");
    } else {
        println!("Results are sorted into 'paths.txt'.")
    }
    Ok(())
}

fn get_proc_list(system: System, users: Users) -> Vec<ExpectProcList> {
    let proc_result: Vec<ExpectProcList> = system
        .processes()
        .iter()
        .map(|(pid, process)| {
            let user = process
                .user_id()
                .and_then(|uid| users.get_user_by_id(uid))
                .map(|user_name| user_name.name())
                .unwrap_or("unknown");

            ExpectProcList {
                pid: pid.as_u32(),
                process: process.name().to_string_lossy().into_owned(),
                user: user.to_string(),
            }
        })
        .collect();
    proc_result
}
