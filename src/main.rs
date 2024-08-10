use std::fs;
#[cfg(not(target_os = "windows"))]
use std::os::unix::process::CommandExt;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
const MANAGER: &str = "open_lights_manager.exe";
#[cfg(not(target_os = "windows"))]
const MANAGER: &str = "open_lights_manager";
#[cfg(target_os = "windows")]
const NEW: &str = "NEW-open_lights_manager.exe";
#[cfg(not(target_os = "windows"))]
const NEW: &str = "NEW-open_lights_manager";
fn main() {
    let new_manager_path = Path::new(NEW);
    let old_manager_path = Path::new(MANAGER);

    if new_manager_path.exists() {
        println!("Found a new installation");
        if old_manager_path.exists() {
            println!("Removing");
            fs::remove_file(old_manager_path).unwrap();
        }
        println!("Renaming");
        fs::rename(new_manager_path, old_manager_path).unwrap();
    }

    if old_manager_path.exists() {
        println!("Launching the program");
        #[cfg(target_os = "windows")]
        launch_windows();
        #[cfg(not(target_os = "windows"))]
        launch_unix();
    } else {
        println!("Failed to locate the program");
    }
}

#[cfg(target_os = "windows")]
fn launch_windows() {
    let mut cmd = Command::new("./open_lights_manager.exe");
    let cmd_2 = CommandExt::creation_flags(&mut cmd, 0x00000008);
    cmd_2.spawn().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn launch_unix() {
    let mut cmd = Command::new("./open_lights_manager.exe");
    cmd.exec();
}
