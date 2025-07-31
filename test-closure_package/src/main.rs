fn main() {
    // let x = 10;
    // // 示例：js=(num){return num+x;}
    // let add_num = |num| num + x;
    // println!("{}", add_num(x)); // 输出 15
    // println!("{}", x); // 仍然可以使用 x
    // println!("{}", x); // 仍然可以使用 x

    // let s = String::from("hello");
    // let print_s = move || println!("{}", s); // move 's'变量的 所有权转移到 闭包函数了。
    // print_s(); // 输出 "hello"
    //            // println!("{}", s); // 这行代码将会报错，因为 s 的所有权已经被转移给了闭包
    test_fn_i8();
}

fn test_fn_i8() {
    let mut i: i8 = 3;
    let f = || i + 2;

    // f闭包对i是immutable borrowed，是Fn trait
    let v = f();

    // f闭包中只是immutable borrowed，此处可以再做borrowed。
    dbg!(i);

    // f可以调用多次
    let v2 = f();

    // 此时，f闭包生命周期已经结束，i已经没有borrowed了，所以此处可以mutable borrowed。
    // i += 10;
    let mut j = 100;
    j += i;

    let v3 = f();
    i += j;

    println!("v={}, v2={}, i={}, v3={}, j={}", v, v2, i, v3, j);

    // [src/main.rs:23:5] &i = 1
    // v=2,v2=2,i=11
}
