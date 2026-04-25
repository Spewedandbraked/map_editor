use fltk::dialog;

pub fn new_project() {
    println!("нажата функция New Project");
    dialog::message_default("Новый проект создан");
}

pub fn open_project() {
    println!("нажата функция Open Project");
    dialog::message_default("Проект открыт");
}

pub fn save_project() {
    println!("нажата функция Save Project");
    dialog::message_default("Проект сохранён");
}

pub fn export_project() {
    println!("нажата функция Export Project");
    dialog::message_default("Проект экспортирован");
}