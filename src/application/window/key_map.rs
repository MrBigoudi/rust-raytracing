// use log::warn;
use winit::keyboard::SmolStr;

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
