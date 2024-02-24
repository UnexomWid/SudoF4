use active_win_pos_rs::get_active_window;
use std::process::Command;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::F4, &[ModKey::Win], || {
        let win = get_active_window();

        if win.is_err() {
            println!("Failed to get the current active window");
            return;
        }

        let win = win.unwrap();

        let output = Command::new("taskkill")
            .args(["/F", "/PID", &win.process_id.to_string()])
            .output();

        match output {
            Err(e) => println!(
                "Failed to terminate process with PID {}: {}",
                win.process_id, e
            ),
            Ok(_) => println!("Killed process with PID {}: {}", win.process_id, win.title),
        }
    })
    .unwrap();

    println!("[SudoF4] Listening for Win + F4...");

    hkm.event_loop();
}
