// use std::io::{self, Write};
use winreg::RegKey;
use winreg::enums::HKEY_CLASSES_ROOT;

const KEY_NAME: &str = "._aaa_001";

fn main() {
    let key_name = KEY_NAME;
    if is_has_key(key_name) {
        println!("has key : {}", key_name);
        if delete_key(key_name) {
            println!("delete key success");
        } else {
            println!("delete key fail");
        }
    } else {
        println!("no key : {}", key_name);
        let (_, res) = create_key(key_name);
        if res {
            println!("create key success");
        } else {
            println!("create key fail");
        }
    }
}

fn is_has_key(key_name: &str) -> bool {
    RegKey::predef(HKEY_CLASSES_ROOT)
        .open_subkey(key_name)
        .is_ok()
}

fn create_key(key_name: &str) -> (RegKey, bool) {
    println!("create key : {}", key_name);
    let reg_key = RegKey::predef(HKEY_CLASSES_ROOT);
    let res = reg_key.create_subkey(key_name).is_ok();
    (reg_key, res)

    // // 创建新的注册表项
    // let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    // let (aaaaaa, _) = hkcr.create_subkey(&format!(".aaaaaa")).unwrap();
    // // 设置注册表项的值
    // aaaaaa.set_value("", &format!("aaaaaa_file")).unwrap();
    // // 创建子项并设置值
    // let (aaaaaa_file, _) = hkcr.create_subkey(&format!("aaaaaa_file")).unwrap();
    // let (aaaaaa_file_defaulticon, _) = aaaaaa_file.create_subkey(&format!("DefaultIcon")).unwrap();
    // aaaaaa_file_defaulticon
    //     .set_value(
    //         "",
    //         &format!(r#"D:\worker\wiring - diagram - editor - ui\src - tauri\icons\128x128.png"#),
    //     )
    //     .unwrap();
    // let (aaaaaa_file_shell_open_command, _) = aaaaaa_file
    //     .create_subkey(&format!("shell\\open\\command"))
    //     .unwrap();
    // aaaaaa_file_shell_open_command
    //     .set_value(
    //         "",
    //         &format!(
    //             r#"D:\worker\wiring - diagram - editor - ui\src - tauri\target\debug\云图.exe "%1""#
    //         ),
    //     )
    //     .unwrap();
}

fn delete_key(key_name: &str) -> bool {
    println!("delete key : {}", key_name);
    RegKey::predef(HKEY_CLASSES_ROOT)
        .delete_subkey(key_name)
        .is_ok()
}
