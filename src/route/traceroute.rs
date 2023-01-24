use actix_web::{web, HttpResponse, Responder};
use rust_shell::{cmd, ShellError};
use serde_derive::Deserialize;
use sys_info::os_type;
#[derive(Deserialize)]
pub struct Param {
    q: String,
    ipv4_source: Option<String>,
}

pub fn traceroute(param: web::Query<Param>, mode: i8) -> impl Responder {
    let mut traceroute: Vec<&str> = Vec::new();
    match Some(os_type().unwrap().to_ascii_lowercase().as_str()) {
        Some(platform) if platform.contains("bsd")=> {
            match mode {
                6 => {
                    traceroute.append(&mut vec!["traceroute6"]);
                }
                _ => {
                    traceroute.append(&mut vec!["traceroute"]);
                }
            };
            traceroute.append(&mut vec![
                match platform {
                    "openbsd" => "-A",
                    _ => "-a",
                },
                "-q1",
                "-w1",
                "-m15",
            ]);
        }

        _ => {
            match mode {
                6 => {
                    traceroute.append(&mut vec!["traceroute6", "-6"]);
                }
                _ => {
                    traceroute.append(&mut vec!["traceroute", "-4"]);
                }
            }
            traceroute.append(&mut vec!["-A", "-q1", "-N32", "-w1", "-m15"])
        }
    };
    match param.ipv4_source.as_deref() {
        Some(ref source) => traceroute.append(&mut vec!["-s", source]),
        None => {}
    };
    HttpResponse::Ok().body(
        match cmd!(&format!("{} {}", traceroute.join(" "), &param.q)).stdout_utf8() {
            Ok(stdout) => stdout,
            Err(e) => match e {
                ShellError::Status(string, exitstatus) => format!("{}: {}", exitstatus, string),
                ShellError::IoError(_) => "IoError".to_string(),
                ShellError::VarError(_) => "VarError".to_string(),
                ShellError::NoSuchProcess => "NoSuchProcess".to_string(),
                ShellError::Errno(string, errno) => format!("{}: {}", errno, string),
            },
        },
    )
}
pub async fn traceroute4(_param: web::Query<Param>) -> impl Responder {
    traceroute(_param, 4)
}
pub async fn traceroute6(_param: web::Query<Param>) -> impl Responder {
    traceroute(_param, 6)
}
