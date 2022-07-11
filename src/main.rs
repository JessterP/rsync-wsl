use std::env;
use std::process::{Command, Stdio};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args[0] = "rsync".to_string();

    for arg in args.iter_mut() {

        if arg.starts_with("C:\\") {
            let output = Command::new("wsl").arg("wslpath").arg("-a").arg(&arg.clone())
                .output().expect("failed to convert path");

            let stdout = output.stdout.as_slice();
            let new_path = std::str::from_utf8(stdout)
                .expect("Failed to path to utf8 string");

            arg.clear();
            arg.push_str(new_path);

        } else if arg.starts_with("/cygdrive/") {
            let (drive, path) = arg[10..].split_once("/").expect("failed to split cygdrive path");
            let (drive, path) = ( drive.to_lowercase(), path.to_string() );

            arg.clear();
            arg.push_str("/mnt/");
            arg.push_str(drive.as_str());
            arg.push_str("/");
            arg.push_str(path.as_str());
        }

    }

    let mut e = Command::new("wsl")
        .args(args).status().expect("failed to spawn wsl/rsync");
    std::process::exit(e.code().unwrap_or(0))
}
