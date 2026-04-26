use std::sync::mpsc::Sender;
use crate::editor::Command;

pub fn new_project(sender: &Sender<Command>) {
    sender.send(Command::NewProject).unwrap();
}