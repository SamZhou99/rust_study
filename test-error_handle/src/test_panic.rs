// 可恢复错误用 Result<T, E> 类来处理
// 不可恢复错误使用 panic! 宏来处理。

use std::io;
use std::{
    fs::File,
    io::{BufReader, Read},
};

// 可恢复错误
pub fn demo_result(file_name: &str) -> Result<String, io::Error> {
    let f = File::open(file_name);
    match f {
        Ok(_file) => {
            println!("✔ 打开文件成功 {}", file_name);
            let mut buf = String::new();
            let mut reader = BufReader::new(_file);
            let _ = reader.read_to_string(&mut buf);
            Ok(buf)
        }
        Err(_err) => {
            println!("✘ 打开文件失败 {}\n{}", file_name, _err);
            Err(_err)
        }
    }
}

// 不可以恢复错误
pub fn demo_panic() {
    panic!("demo_panic : 不可以恢复错误");
}
