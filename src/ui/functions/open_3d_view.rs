use std::sync::mpsc::Sender;
use crate::editor::Command;

pub fn open_3d_view(sender: &Sender<Command>) {
    sender.send(Command::AddViewport).unwrap();
}