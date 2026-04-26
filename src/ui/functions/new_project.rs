use std::sync::mpsc::Sender;
use crate::ui::Command;

pub fn new_project(sender: &Sender<Command>) {
    sender.send(Command::NewProject).unwrap();
}