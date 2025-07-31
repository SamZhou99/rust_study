fn main() {
    // Rust主要有两种类型的字符串：&str和String
    // &str【不可变，没有所有权】
    // String【可变，有所有权】

    let s1: &str = "12345";
    let end = s1.len();
    let s2 = &s1[end / 2..end]; // 切片 "s1"必须"&"借用，因为局部变量必须一个静态的大小。
    println!("s1={}, s2={}", s1, s2);

    let s1 = " 1 Hello World ";
    // s1 = s1.trim().to_lowercase().replace("world", "rust"); // 这里不可以使用let mut s1="xxx";s1=s1;改变变量
    let s1 = s1.trim().to_lowercase().replace("world", "rust"); // 还要使用s1变量，就使用变量遮蔽 "let s1" Variable Shadowing
    println!("s1=\"{}\"", s1);

    let mut s1 = String::from(" 2 Hello World ");
    s1 = s1.trim().to_lowercase().replace("world", "rust"); // 这里可以使用s1改变变量
    println!("s1=\"{}\"", s1);
}
