fn use_char() {
    let a = '中';
    println!("中占用字节 {}", size_of_val(&a));
}

fn use_for() {
    for i in 1..=5 {
        println!("number = {}", i);
    }
    for i in 'a'..='g' {
        println!("char = {}, {} 字节", i, size_of_val(&i));
    }
}

fn main() {
    use_char();
    use_for();
    // println!("Hello, world!");
}
