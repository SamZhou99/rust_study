use std::io::stdin;

fn main() {
    demo01();
    demo02();
}

fn demo01() {
    let a = [9, 8, 7, 6, 5, 4, 3, 2, 1];

    println!("请输入一个数组索引 （0-{}）", a.len() - 1);

    let mut index = String::new();

    stdin().read_line(&mut index).expect("读取行失败~~~!");

    let index: usize = index.trim().parse().expect("输入的索引不是数字~~~!");
    if index >= a.len() {
        println!("索引 {} 超出数组范围", index);
        return;
    }

    let element = a[index];
    println!("索引 {} 处元素的值为：{}", index, element);
}

fn demo02() {
    // 交换数组第一个和最后一个元素
    let mut a2 = [1, 2, 3, 4, 5];
    if a2.len() < 2 {
        println!("数组元素不足两个，无法交换");
        return;
    }
    let a3 = a2;
    let temp = (a2[0], a2[1]);
    (a2[0], a2[1]) = (a2[a2.len() - 1], a2[a2.len() - 2]);
    (a2[a2.len() - 1], a2[a2.len() - 2]) = temp;
    println!(
        "交换数组第一个和最后一个元素\n原数组 {:?}\n交换后数组 {:?}",
        a3, a2
    );
}
