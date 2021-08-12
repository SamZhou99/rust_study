use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::{thread, time};

fn input_number() -> i32 {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("输入错误");
        let in_number: i32 = match buf.trim().parse() {
            Ok(num) => {
                println!("\n输入的是：{}", num);
                num
            }
            Err(e) => {
                println!("\n异常：{}\n请输入一个整数!", e);
                continue;
            }
        };
        return in_number;
    }
}

fn number_cmp(random: i32, in_num: i32) -> Ordering {
    match in_num.cmp(&random) {
        Ordering::Less => {
            println!("少了");
            Ordering::Less
        }
        Ordering::Greater => {
            println!("多了");
            Ordering::Greater
        }
        Ordering::Equal => {
            println!("相等");
            Ordering::Equal
        }
    }
}

fn delay(second: u64) {
    let millis = time::Duration::from_millis(1000 * second);
    thread::sleep(millis);
}

fn main() {
    println!("开始猜数游戏，从100-999之间的数");
    let random_number = rand::thread_rng().gen_range(100..999);

    loop {
        let in_number: i32 = input_number();
        let result: Ordering = number_cmp(random_number, in_number);
        if Ordering::Equal == result {
            println!("=== 恭喜猜出正确答案！===");
            break;
        }
    }

    delay(2);

    // // Shadowing 重影
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("重影变量 x : {}", x);

    // // 变量所有权
    // let s1 = String::from("hello"); // 对象变量产生在堆中
    // let s2 = s1; // 对象移动到 s2  如果使用copy就不是移动了。 s1.clone()
    // println!("{}, world!", s2); // 这里不能打印 s1. 错误！s1 已经失效
}
