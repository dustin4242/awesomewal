use std::{env::args, path::Path, process::Command};
fn main() -> std::io::Result<()> {
    let all_args: Vec<String> = args().collect();
    if all_args.len() >= 2 {
        if Path::new(&all_args[1]).exists() {
            if !all_args[1].ends_with(".png") != true {
                let _walling = Command::new("wal")
                    .args(["-i", &all_args[1], "--saturate", "1"])
                    .output();
                let _set_all_args = Command::new("cp")
                    .args([
                        &all_args[1],
                        "/home/dudebro/.config/awesome/themes/powerarrow-dark/wall.png",
                    ])
                    .output();
                let _restart_client = Command::new("awesome-client")
                    .arg("awesome.restart()")
                    .output();
            } else {
                error_print("Not a valid .png file")
            }
        } else {
            error_print("Not a valid path")
        }
    } else {
        match all_args.len() {
            1 => println!("Usage: awesomewal [path to all_args]"),
            _ => error_print("How"),
        }
    }
    Ok(())
}
fn error_print(error_label: &'static str) {
    println!("Error: {}", error_label)
}
