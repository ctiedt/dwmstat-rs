//use std::env::args;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};

use chrono::offset::Local;
use chrono::DateTime;
use sysinfo::{NetworkExt, Processor, ProcessorExt, System, SystemExt};

fn get_wifi() -> Result<String, Box<dyn std::error::Error>> {
    let ssid_command = String::from_utf8(Command::new("iwgetid").output()?.stdout)?;
    let ssid: Vec<&str> = ssid_command.split('"').collect();
    let ip_route_command = String::from_utf8(
        Command::new("ip")
            .args(&["route", "get", "1"])
            .output()?
            .stdout,
    )?;
    let ip_route: Vec<&str> = ip_route_command.split_whitespace().collect();
    Ok(format!("直 {} : {}", ssid[1], ip_route[6]))
}

fn get_cpu(system: &System) -> String {
    let proc: &Processor = system.get_global_processor_info();
    let cpu_perc = proc.get_cpu_usage();
    format!(" {:.2}%", cpu_perc)
}

fn get_mem(system: &System) -> String {
    let mb_factor = 1024u64.pow(2) as f64;
    let mem = system.get_used_memory() as f64 / mb_factor;
    let total_mem = system.get_total_memory() as f64 / mb_factor;
    format!(" {:.2}Gb/{:.2}Gb", mem, total_mem)
}

fn get_time() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    format!("{}", datetime.format(" %d/%m/%y  %T"))
}

fn update_status(status: &String) {
    let _ = Command::new("xsetroot").arg("-name").arg(status).output();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut status: String;
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    loop {
        system.refresh_all();
        let values = [get_wifi()?, get_cpu(&system), get_mem(&system), get_time()];
        status = values.join(" ⟪ ");
        update_status(&status);
        thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}
