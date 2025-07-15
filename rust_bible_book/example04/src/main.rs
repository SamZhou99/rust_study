fn main() {
    // 使用定义mut修改变量
    let mut x = 5;
    x = 6;
    // 使用下划线开头忽略未使用的变量
    let _x = 5;
    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    b = true;
    if b {
        println!("b:{}", b);
    }
    assert_eq!(a, b);
    // 变量遮蔽(shadowing)
    let y = 5;
    let y = y + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
    println!("spaces:{}", spaces);
}
