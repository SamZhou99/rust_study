use std::{thread, time};

fn main() {
    let mut start: u128 = 0;
    let mut end: u128 = 1;
    let mut result: u128;
    let mut i: u32 = 0;

    let mut str_buf = String::new();
    println!("请输入数字：");
    std::io::stdin().read_line(&mut str_buf).expect("输入错误");
    let mut num = str_buf.trim().parse().expect("请输入数字");
    if num > 150 {
        num = 150;
    }

    while i < num {
        i += 1;
        result = start + end;
        println!("{}: {} + {} = {}", i, start, end, result);
        start = end;
        end = result;
    }

    let ten_millis = time::Duration::from_millis(1000 * 2);
    thread::sleep(ten_millis);
}
