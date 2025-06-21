use std::env;

use crate::debug_println;

pub enum DisplayServer {
    Wayland,
    X11,
    Unknown(String),
    None,
}

impl DisplayServer {
    fn detect() -> DisplayServer {
        if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
            debug_println!("found XDG_SESSION: {}", session_type);
            match session_type.to_lowercase().as_str() {
                "wayland" => DisplayServer::Wayland,
                "x11" => DisplayServer::X11,
                _ => DisplayServer::Unknown(session_type),
            }
        } else if env::var("WAYLAND_DISPLAY").is_ok() {
            debug_println!("WAYLAND_DISPLAY is found");
            DisplayServer::Wayland
        } else if env::var("DISPLAY").is_ok() {
            debug_println!("DISPLAY (X11) is found");
            DisplayServer::X11
        } else {
            DisplayServer::None
        }
    }

    pub fn should_use_daemon() -> bool {
        match Self::detect() {
            DisplayServer::Wayland => false,
            DisplayServer::X11 => true,
            // Iam not sure about these values, let's assume they do not require a daemon
            DisplayServer::Unknown(srv) => {
                debug_println!(
                    "Unknown display server detected, assuming no daemon is needed, please report your display server's name in issue tracker. {}",
                    srv
                );
                false
            }
            DisplayServer::None => false,
        }
    }
}
