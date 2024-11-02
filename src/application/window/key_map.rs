use log::warn;
// use log::warn;
use winit::{event::ElementState, keyboard::{KeyCode, SmolStr}};

pub fn winit_character_to_imgui_key(key: SmolStr) -> Option<imgui::Key> {
    if key == "A" {
        Some(imgui::Key::A)
    } else if key == "B" {
        Some(imgui::Key::B)
    } else if key == "C" {
        Some(imgui::Key::C)
    } else if key == "D" {
        Some(imgui::Key::D)
    } else if key == "E" {
        Some(imgui::Key::E)
    } else if key == "F" {
        Some(imgui::Key::F)
    } else if key == "G" {
        Some(imgui::Key::G)
    } else if key == "H" {
        Some(imgui::Key::H)
    } else if key == "I" {
        Some(imgui::Key::I)
    } else if key == "J" {
        Some(imgui::Key::J)
    } else if key == "K" {
        Some(imgui::Key::K)
    } else if key == "L" {
        Some(imgui::Key::L)
    } else if key == "M" {
        Some(imgui::Key::M)
    } else if key == "N" {
        Some(imgui::Key::N)
    } else if key == "O" {
        Some(imgui::Key::O)
    } else if key == "P" {
        Some(imgui::Key::P)
    } else if key == "Q" {
        Some(imgui::Key::Q)
    } else if key == "R" {
        Some(imgui::Key::R)
    } else if key == "S" {
        Some(imgui::Key::S)
    } else if key == "T" {
        Some(imgui::Key::T)
    } else if key == "U" {
        Some(imgui::Key::U)
    } else if key == "V" {
        Some(imgui::Key::V)
    } else if key == "W" {
        Some(imgui::Key::W)
    } else if key == "X" {
        Some(imgui::Key::X)
    } else if key == "Y" {
        Some(imgui::Key::Y)
    } else if key == "Z" {
        Some(imgui::Key::Z)
    } else if key == "'" {
        Some(imgui::Key::Apostrophe)
    } else if key == "," {
        Some(imgui::Key::Comma)
    } else if key == "-" {
        Some(imgui::Key::Minus)
    } else if key == "." {
        Some(imgui::Key::Period)
    } else if key == "/" {
        Some(imgui::Key::Slash)
    } else if key == ";" {
        Some(imgui::Key::Semicolon)
    } else if key == "=" {
        Some(imgui::Key::Equal)
    } else if key == "[" {
        Some(imgui::Key::LeftBracket)
    } else if key == "\\" {
        Some(imgui::Key::Backslash)
    } else if key == "]" {
        Some(imgui::Key::RightBracket)
    } else if key == "`" {
        Some(imgui::Key::GraveAccent)
    } else {
        // warn!(
        //     "The winit key `{:?}' is not yet supported for imgui...",
        //     key
        // );
        None
    }
}

#[derive(PartialEq, Eq)]
pub enum KeyState{
    Pressed,
    Released,
}

impl Default for KeyState{
    fn default() -> Self {
        KeyState::Released
    }
}

impl KeyState{
    pub fn from_winit(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Key{
    A,B,C,D,E,F,G,H,I,
    J,K,L,M,N,O,P,Q,R,
    S,T,U,V,W,X,Y,Z,
    Left, Right,
    Up, Down,
    Enter, Delete, Escape,
}

impl Key {
    pub fn from_winit(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::KeyA => Some(Key::A),
            KeyCode::KeyB => Some(Key::B),
            KeyCode::KeyC => Some(Key::C),
            KeyCode::KeyD => Some(Key::D),
            KeyCode::KeyE => Some(Key::E),
            KeyCode::KeyF => Some(Key::F),
            KeyCode::KeyG => Some(Key::G),
            KeyCode::KeyH => Some(Key::H),
            KeyCode::KeyI => Some(Key::I),
            KeyCode::KeyJ => Some(Key::J),
            KeyCode::KeyK => Some(Key::K),
            KeyCode::KeyL => Some(Key::L),
            KeyCode::KeyM => Some(Key::M),
            KeyCode::KeyN => Some(Key::N),
            KeyCode::KeyO => Some(Key::O),
            KeyCode::KeyP => Some(Key::P),
            KeyCode::KeyQ => Some(Key::Q),
            KeyCode::KeyR => Some(Key::R),
            KeyCode::KeyS => Some(Key::S),
            KeyCode::KeyT => Some(Key::T),
            KeyCode::KeyU => Some(Key::U),
            KeyCode::KeyV => Some(Key::V),
            KeyCode::KeyW => Some(Key::W),
            KeyCode::KeyX => Some(Key::X),
            KeyCode::KeyY => Some(Key::Y),
            KeyCode::KeyZ => Some(Key::Z),
            KeyCode::ArrowLeft => Some(Key::Left),
            KeyCode::ArrowRight => Some(Key::Right),
            KeyCode::ArrowUp => Some(Key::Up),
            KeyCode::ArrowDown => Some(Key::Down),
            KeyCode::Enter => Some(Key::Enter),
            KeyCode::Delete => Some(Key::Delete),
            KeyCode::Escape => Some(Key::Escape),
            _ => {
                warn!{"Winit Key: {:?} is not yet supported...", key_code};
                None
            },
        }
    }
}