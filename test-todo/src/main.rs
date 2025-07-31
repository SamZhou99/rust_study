struct MyStruct;

trait Foo {
    fn bar(&self);
    fn baz(&self);
}

impl Foo for MyStruct {
    fn bar(&self) {
        // 实现在这里
        println!("这里实现");
    }

    fn baz(&self) {
        // 让我们现在不必担心实现 baz()
        todo!();
    }
}

// todo!() 表示未完成的代码
fn main() {
    let s = MyStruct;
    s.bar();

    // 我们甚至没有使用 baz()，所以很好。
}
