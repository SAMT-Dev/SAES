use std::{env, fs, path::Path, process::Command};

pub fn main() {
    init_ffmpeg();
    init_error_dir();
}

fn init_dir() {
    let dir = Path::new("exec");
    if !dir.exists() {
        fs::create_dir(dir).expect("Exec dir létrehozása sikertelen")
    }
}

fn init_ffmpeg() {
    if env::consts::OS == "windows" {
        init_dir();
        let exec = Path::new("exec/ffmpeg.exe");
        if !exec.exists() {
            panic!("ffmpeg.exe nem található")
        }
    } else if Command::new("ffmpeg").spawn().is_err() {
        panic!("ffmpeg nincs telepítve")
    }
}

fn init_error_dir() {
    let error_dir = Path::new("error");
    if !error_dir.exists() {
        fs::create_dir(error_dir).expect("error dir létrehozása sikertelen")
    }
}
