use std::sync::mpsc::Sender;
use crate::editor::Command;

pub fn toggle_assets(sender: &Sender<Command>) {
    sender.send(Command::ToggleAssets).unwrap();
}