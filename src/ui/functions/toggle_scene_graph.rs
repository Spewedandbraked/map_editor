use std::sync::mpsc::Sender;
use crate::editor::Command;

pub fn toggle_scene_graph(sender: &Sender<Command>) {
    sender.send(Command::ToggleSceneGraph).unwrap();
}