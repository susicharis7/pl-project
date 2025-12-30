use std::process::Command;

pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("clear").status();
    }
}
