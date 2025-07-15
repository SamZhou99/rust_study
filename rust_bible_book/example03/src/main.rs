fn main() {
    let a = 12;
    let b: i32 = 34;
    let c = add(a, b); //let mut c = add(a, b);
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    // 最后这里不要加";"就是return了
    i + j
}
