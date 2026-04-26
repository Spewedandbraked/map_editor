use std::sync::mpsc::Sender;
use crate::ui::Command;

pub fn tools_menu(sender: &Sender<Command>) {
    sender.send(Command::ToggleTools).unwrap();
}