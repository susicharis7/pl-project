// Run System commands 
use std::process::Command;


pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        // `-` - I am aware that it is returning something, but I don't care, just ignore it and dont write it so that I have clear screen 
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("clear").status();
    }
}
