/*
所有权 (Ownership)
Rust 中的所有权是独特的内存管理机制，核心概念包括所有权 (ownership)、借用 (borrowing) 和引用 (reference)。

所有权规则:
Rust 中的每个值都有一个所有者。
每个值在任意时刻只能有一个所有者。
当所有者超出作用域时，值会被删除。
*/
fn example01() {
    // 需要注意的是 字符串 和 数字类型的不一样
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被转移给了 s2
    // let s2 = s1.clone(); 【需要 s2 = s1.clone() 才不会报错】
    // println!("s1={}, s2={}", s1, s2); // 此处编译会报错，因为 s1 已不再拥有该值
    println!("s2={}", s2);
}
fn example02() {
    let n1 = 5;
    let n2 = n1;
    assert_eq!(n1, n2);
    println!("n1={}, n2={}", n1, n2);

    // &str
    let s1 = "abc";
    let s2 = s1;
    assert_eq!("abc", s2);
    println!("s1={}, s2={}", s1, s2);

    // String
    let s1 = String::from("abc");
    let s2 = &s1;
    assert_eq!("abc", s2);
    println!("s1={}, s2={}", s1, s2);
}
fn example03() {
    // 引用与解引用
    let n1 = 5;
    let n2 = &n1; // &引用
    assert_eq!(n1, *n2); // *解引用   因为比较类型一致，不解引用，类型就不一致，就会报错。
}
fn example04() {
    // 可变引用
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("y: {}", y);

    let x = 5;
    let mut y = x;
    y += 1;
    println!("x: {}, y: {}", x, y);
}
fn example05() {
    // // Rust对引用的使用有严格的规则：
    // // 同一时间只能有一个可变引用，或多个不可变引用，但不能同时存在。
    // // 引用必须始终有效，引用的数据在其生命周期内不能被销毁。
    // let mut x = 5;
    // let r1 = &x; // 不可变引用
    // let r2 = &x; // 不可变引用
    // let r3 = &mut x; // 可变引用，错误！
    // println!("{}, {}, {}", r1, r2, r3);
    todo!("示例完成✅");
}
fn main() {
    example01();
    example02();
    example03();
    example04();
    example05();
}
