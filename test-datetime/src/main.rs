use chrono::Local;

fn main() {
    let now = Local::now();
    println!("{}", now.format("%Y年%m月%d日 %H:%M:%S")); // 输出: 2024年05月12日 14:30:45
}
