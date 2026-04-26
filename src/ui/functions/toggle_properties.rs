use std::sync::mpsc::Sender;
use crate::editor::Command;

pub fn toggle_properties(sender: &Sender<Command>) {
    sender.send(Command::ToggleProperties).unwrap();
}