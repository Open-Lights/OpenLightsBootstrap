use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

const MANAGER: &str = "open_lights_manager.exe";
const NEW: &str = "NEW-open_lights_manager.exe";
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
        let mut cmd = Command::new("./open_lights_manager.exe");
        let cmd_2 = CommandExt::creation_flags(&mut cmd, 0x00000008);
        cmd_2.spawn().unwrap();
    } else {
        println!("Failed to locate the program");
    }
}
