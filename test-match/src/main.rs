use std::io::stdin;

fn main() {
    println!("请输入数字");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("输入错误");
    let buf = buf.trim();
    let in_number: i32 = match buf.parse() {
        Ok(num) => num,
        Err(e) => {
            println!("输入非数字：{}", e);
            -1
        }
    };
    println!("输入的是：{}, 结果：{}", buf, in_number);
}
