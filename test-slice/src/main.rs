fn main() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    // let mut s = String::from("runoob");
    // let slice = &s[0..3];
    // // 被切片引用的字符串禁止更改其值：
    // s.push_str("yes!"); // 错误
    // println!("slice = {}", slice);
}
