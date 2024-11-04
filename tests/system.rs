use std::env::home_dir;
use std::path::PathBuf;

pub fn init_system() {
    let does_exist = does_path_exist();

    if does_exist {
        does_admin_data_exist();
    } else {
        create_wade_dir();
    }
}

pub fn does_path_exist() -> bool {
    let mut path = PathBuf::new();
    let home_dir = home_dir().unwrap();

    path.push(&home_dir);
    path.push("Wade");

    let mut admin_key_path = PathBuf::new();
    admin_key_path.push(&path);

    println!("Path: {:#?}", path);

    if path.exists() {
        println!("Exists");
        return true;
    } else {
        println!("Does Not Exist, ceating now");
        return false;
    }
}

pub fn create_wade_dir() {
    let mut path = PathBuf::new();
    let home_dir = home_dir().unwrap();

    path.push(&home_dir);
    path.push("Wade");

    std::fs::create_dir(path);
}

pub fn does_admin_data_exist() {
    
}