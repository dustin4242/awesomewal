use std::{process::Command, env::args, path::Path};
fn main() -> std::io::Result<()> {
    let wallpaper: Vec<String> = args().collect();
    if wallpaper.len() >= 2 {
        if Path::new(&wallpaper[1]).exists() {
            if !wallpaper[1].ends_with(".png") != true {
                let _walling = Command::new("wal").args([
                    "-i",
                    &wallpaper[1],
                    "--saturate",
                    "1"
                ]).output();
                let _set_wallpaper = Command::new("cp").args([
                    &wallpaper[1], 
                    "/home/dudebro/.config/awesome/themes/powerarrow-dark/wall.png"
                ]).output();
                let _restart_client = Command::new("awesome-client").arg("awesome.restart()").output();
            }
            else { error_print("Not a valid .png file") }
        }
        else { error_print("Not a valid path") }
    }
    else { match wallpaper.len() {
        1 => error_print("Missing Wallpaper Argument"),
        _ => error_print("How"),
    }}
    Ok(())
}
fn error_print(error_label: &'static str){ println!("Error: {}", error_label) }
