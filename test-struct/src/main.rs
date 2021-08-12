#[derive(Debug)]

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段（field）的结构体
struct Rectangle {
    width: u32,
    height: u32,
}
// 知识点
// struct 方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// impl 可以定义多个，跟在一个里面的作用一样
impl Rectangle {
    fn square_area(size: u32) -> u32 {
        let r = Rectangle {
            width: size,
            height: size,
        };
        r.width * r.height
    }
}

fn main() {
    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let result = rect.area();
    println!(
        "一个长方形的宽是{}，高是{}，求出面积。\n结果是：{}!", rect.width, rect.height, result
    );

    // 知识点
    // 首行添加 #[derive(Debug)] 方法打印 {:?}, {:#?} 加#号后格式会换行
    // println!("{:#?}", rect);

    let square_result = Rectangle::square_area(30);
    println!("正方形的面积是：{}", square_result);
}
