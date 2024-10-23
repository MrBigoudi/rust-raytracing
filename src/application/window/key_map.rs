use log::warn;
use winit::keyboard::{NamedKey, SmolStr};

pub fn winit_named_to_imgui_key(name: NamedKey) -> Option<imgui::Key> {
    match name {
        NamedKey::Tab => Some(imgui::Key::Tab),
        NamedKey::ArrowLeft => Some(imgui::Key::LeftArrow),
        NamedKey::ArrowRight => Some(imgui::Key::RightArrow), 
        NamedKey::ArrowUp => Some(imgui::Key::UpArrow),
        NamedKey::ArrowDown => Some(imgui::Key::DownArrow), 
        NamedKey::PageUp => Some(imgui::Key::PageUp),
        NamedKey::PageDown => Some(imgui::Key::PageDown),
        NamedKey::Home => Some(imgui::Key::Home),
        NamedKey::End => Some(imgui::Key::End),
        NamedKey::Insert => Some(imgui::Key::Insert), 
        NamedKey::Delete => Some(imgui::Key::Delete),
        NamedKey::Backspace => Some(imgui::Key::Backspace), 
        NamedKey::Space => Some(imgui::Key::Space),
        NamedKey::Enter => Some(imgui::Key::Enter),
        NamedKey::Escape => Some(imgui::Key::Escape), 
        NamedKey::Control => Some(imgui::Key::LeftCtrl), 
        NamedKey::Shift => Some(imgui::Key::LeftShift),
        NamedKey::Alt => Some(imgui::Key::LeftAlt),
        NamedKey::Super => Some(imgui::Key::LeftSuper),
        NamedKey::ContextMenu => Some(imgui::Key::Menu),
        NamedKey::Alphanumeric => Some(imgui::Key::Alpha0),
        NamedKey::F1 => Some(imgui::Key::F1),
        NamedKey::F2 => Some(imgui::Key::F2),
        NamedKey::F3 => Some(imgui::Key::F3),
        NamedKey::F4 => Some(imgui::Key::F4),
        NamedKey::F5 => Some(imgui::Key::F5),
        NamedKey::F6 => Some(imgui::Key::F6),
        NamedKey::F7 => Some(imgui::Key::F7),
        NamedKey::F8 => Some(imgui::Key::F8),
        NamedKey::F9 => Some(imgui::Key::F9),
        NamedKey::F10 => Some(imgui::Key::F10),
        NamedKey::F11 => Some(imgui::Key::F11),
        NamedKey::F12 => Some(imgui::Key::F12),
        NamedKey::CapsLock => Some(imgui::Key::CapsLock),
        NamedKey::ScrollLock => Some(imgui::Key::ScrollLock),
        NamedKey::NumLock => Some(imgui::Key::NumLock),
        NamedKey::PrintScreen => Some(imgui::Key::PrintScreen),
        NamedKey::Pause => Some(imgui::Key::Pause),
        _ => {
            warn!("The winit key `{:?}' is not yet supported for imgui...", name);
            None
        }
    }
}

pub fn winit_character_to_imgui_key(key: SmolStr) -> Option<imgui::Key> {
    if key == "A"{
        return Some(imgui::Key::A);
    } else if key == "B" {
        return Some(imgui::Key::B);
    } else if key == "C" {
        return Some(imgui::Key::C);
    } else if key == "D" {
        return Some(imgui::Key::D);
    } else if key == "E" {
        return Some(imgui::Key::E);
    } else if key == "F" {
        return Some(imgui::Key::F);
    } else if key == "G" {
        return Some(imgui::Key::G);
    } else if key == "H" {
        return Some(imgui::Key::H);
    } else if key == "I" {
        return Some(imgui::Key::I);
    } else if key == "J" {
        return Some(imgui::Key::J);
    } else if key == "K" {
        return Some(imgui::Key::K);
    } else if key == "L" {
        return Some(imgui::Key::L);
    } else if key == "M" {
        return Some(imgui::Key::M);
    } else if key == "N" {
        return Some(imgui::Key::N);
    } else if key == "O" {
        return Some(imgui::Key::O);
    } else if key == "P" {
        return Some(imgui::Key::P);
    } else if key == "Q" {
        return Some(imgui::Key::Q);
    } else if key == "R" {
        return Some(imgui::Key::R);
    } else if key == "S" {
        return Some(imgui::Key::S);
    } else if key == "T" {
        return Some(imgui::Key::T);
    } else if key == "U" {
        return Some(imgui::Key::U);
    } else if key == "V" {
        return Some(imgui::Key::V);
    } else if key == "W" {
        return Some(imgui::Key::W);
    } else if key == "X" {
        return Some(imgui::Key::X);
    } else if key == "Y" {
        return Some(imgui::Key::Y);
    } else if key == "Z" {
        return Some(imgui::Key::Z);
    } else if key == "'" {
        return Some(imgui::Key::Apostrophe);
    } else if key == "," {
        return Some(imgui::Key::Comma);
    } else if key == "-" {
        return Some(imgui::Key::Minus);
    } else if key == "." {
        return Some(imgui::Key::Period);
    } else if key == "/" {
        return Some(imgui::Key::Slash);
    } else if key == ";" {
        return Some(imgui::Key::Semicolon);
    } else if key == "=" {
        return Some(imgui::Key::Equal);
    } else if key == "[" {
        return Some(imgui::Key::LeftBracket);
    } else if key == "\\" {
        return Some(imgui::Key::Backslash);
    } else if key == "]" {
        return Some(imgui::Key::RightBracket);
    } else if key == "`" {
        return Some(imgui::Key::GraveAccent);
    } else {
        warn!("The winit key `{:?}' is not yet supported for imgui...", key);
        return None;
    }
}