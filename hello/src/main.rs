fn main() {
    let a = 1;
    let b = 2;
    let c = a + b;
    let c = c * 2;
    println!("Hello, world! {}", c);
    println!("a:{}, b:{}, c:{}, ", a, b, c);
    // 中英文长度不一样
    println!("Eng.len()：{}", String::from("Eng").len());
    println!("中文.len()：{}", String::from("中文").len());

    // 可变 租用 权限(s租给s2并可变,其他不可变)
    let mut s: String = String::from("Hello");
    let s2 = &mut s;
    // 改变字符串：删除或清除原来的 再插入
    s2.remove(0);
    s2.clear();
    s2.push_str("ABC");
    s2.insert_str(2, ",,");
    println!("{}", s2);
}
