use std::sync::mpsc::Sender;
use crate::editor::Command;

pub fn tools_menu(sender: &Sender<Command>) {
    sender.send(Command::ToggleTools).unwrap();
}