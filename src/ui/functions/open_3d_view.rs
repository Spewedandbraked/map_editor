use std::sync::mpsc::Sender;

use crate::ui::Command;

pub fn open_3d_view(sender: &Sender<Command>) {
    sender.send(Command::AddViewport).unwrap();
}