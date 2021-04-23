#[macro_use]
extern crate rocket;
extern crate lazy_static;

use std::thread;
use std::net::Ipv4Addr;
use rocket::figment::{Figment};
use anyhow::{bail, Result};
use libbpf_rs::PerfBufferBuilder;
use plain::Plain;
use std::time::Duration;
use nix::errno::*;
use nix::unistd::*;
use std::sync::Mutex;

use lazy_static::lazy_static;

mod bpf;
use bpf::*;

lazy_static! {
    static ref EXEC_LIST: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[get("/")]
fn index() -> String {
    let mut x = EXEC_LIST.lock().unwrap();

    let mut buf = [0u8; 64];
    let hostname_cstr = gethostname(&mut buf).expect("failed to get hostname");
    let hostname = hostname_cstr.to_str().unwrap_or("localhost");

    let mut ret = String::new();
    ret.push_str(hostname);
    ret.push('\n');
    while x.len() > 0 {
        let s = x.pop().unwrap();
        ret.push_str(&s);
    }

    ret
}

#[rocket::main]
async fn fly_rocket() -> Result<(), rocket::Error> {
    let mut config = rocket::Config::default();
    config.address = Ipv4Addr::new(0,0,0,0).into();
    config.port = 80u16;

    let figment = Figment::from(config);
    rocket::custom(figment).mount("/", routes![index]).ignite().await?.launch().await
}

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    unsafe {
        let rc = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit);
        if rc != 0 {
            bail!("{} Failed to increase rlimit", errno());
        }
    }

    Ok(())
}

fn lost_exec(_cpu: i32, _count: u64) {

}

unsafe impl Plain for exec_bss_types::exec_data_t {}

fn handle_exec(_cpu: i32, data: &[u8]) {
    let mut exec_data = exec_bss_types::exec_data_t::default();
    plain::copy_from_bytes(&mut exec_data, data).expect("Data Buffer too short");

    let fname = std::str::from_utf8(&exec_data.fname).unwrap_or("UnknownFile");
    let s = format!("{} {}\n", exec_data.pid, fname.trim_end_matches('\0'));
    let mut x = EXEC_LIST.lock().unwrap();
    x.push(s);
}

fn run_bpf() -> Result<()> {
    match bump_memlock_rlimit() {
        Ok(_) => {
            let builder = ExecSkelBuilder::default();
            let mut exec = builder.open()?.load()?;

            exec.attach()?;

            let perf = PerfBufferBuilder::new(exec.maps().events())
                .sample_cb(handle_exec)
                .lost_cb(lost_exec)
                .build()?;

            loop {
                perf.poll(Duration::from_millis(100))?;
            }
        },
        Err(e) => {
            eprintln!("Error {}", e);
            return Err(e);
        }
    }
}

fn main() -> Result<()> {
    let h = thread::spawn(move || {
        fly_rocket().unwrap();
    });

    match run_bpf() {
        _ => {},
    }

    h.join().unwrap();
    Ok(())
}
