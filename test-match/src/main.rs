use std::io::stdin;

fn check_input_num(check_num: u8) -> i32 {
    println!("请输入一个整数：");
    for i in 0..check_num {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("输入错误");
        let buf = buf.trim();
        let _in_number: i32 = match buf.parse() {
            Ok(num) => {
                return num;
            }
            Err(e) => {
                if i < check_num - 1 {
                    print!("输入非数字：\"{}\", ", buf);
                    println!("异常：{}", e);
                    println!("请再次输入一个整数：");
                }
                continue;
            }
        };
    }
    return -1;
}

fn main() {
    let in_number: i32 = check_input_num(3);
    if in_number > 0 {
        println!("输入的数字是：{}", in_number);
    } else {
        println!("输入的数字不是整数。");
    }
}
