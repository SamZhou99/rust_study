fn iter_map() {
    // 迭代处理数据
    let vec = vec![1, 2, 3, 4, 5];
    let squared_vec: Vec<i32> = vec.iter().map(|x| x + x).collect();
    println!("iter_map {:?}", squared_vec);
}

fn iter_filter() {
    // 过滤数据
    let vec = vec![1, 2, 3, 4, 5];
    let filtered_vec: Vec<i32> = vec.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("iter_filter {:?}", filtered_vec);
}

fn iter_for() {
    // for in
    let vec = vec![1, 2, 3, 4, 5];
    for &num in vec.iter() {
        print!("for={}, ", num);
    }
    println!("{}", "");
}

fn iter_sum() {
    // 求和
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    println!("sum={:?}, 和：{}", v, sum);
}

fn iter_next() {
    // 返回迭代器中的下一个元素。
    let mut iter = (1..5).into_iter();
    while let Some(val) = iter.next() {
        print!("next={}, ", val);
    }
    println!("{}", "");
}

fn iter_size_hint() {
    // 返回迭代器中剩余元素数量的下界和上界。
    let iter = ('a'..='z').into_iter();
    println!("size_hint={:?}", iter.size_hint());
}

fn iter_count() {
    // 计算迭代器中的元素数量。
    let count = (1..5).into_iter().count();
    println!("count={}", count);
}

fn iter_nth() {
    // 返回迭代器中第 n 个元素。
    let third = ('a'..='z').into_iter().nth(2);
    println!("nth={:?}", third);
}

fn iter_last() {
    // 返回迭代器中的最后一个元素。
    let last = ('a'..='z').into_iter().last();
    println!("last={:?}", last);
}

fn iter_all() {
    // 如果迭代器中的所有元素都满足某个条件，返回 true。
    let all_positive = (1..=5).into_iter().all(|x| x > 2);
    println!("iter_all={}", all_positive);
}

fn iter_any() {
    // 如果迭代器中的至少一个元素满足某个条件，返回 true。
    let any_negative = (1..5).into_iter().any(|x| x == 2);
    println!("iter_any={}", any_negative);
}

fn iter_find() {
    // 返回迭代器中第一个满足某个条件的元素。
    let first_even = ('a'..'z').into_iter().find(|&x| x == 'b');
    println!("iter_find={:?}", first_even);
}

fn main() {
    iter_map();
    iter_filter();
    iter_for();
    iter_sum();
    iter_next();
    iter_size_hint();
    iter_count();
    iter_nth();
    iter_last();
    iter_all();
    iter_any();
    iter_find();
}

/*

find_map()对迭代器中的元素应用一个函数，返回第一个返回 Some 的结果。
let first_letter = "hello".chars().find_map(|c| if c.is_alphabetic() { Some(c) } else { None });

map()对迭代器中的每个元素应用一个函数。
let squares: Vec<i32> = (1..5).into_iter().map(|x| x * x).collect();

filter()保留迭代器中满足某个条件的元素。
let evens: Vec<i32> = (1..10).into_iter().filter(|x| x % 2 == 0).collect();

filter_map()对迭代器中的元素应用一个函数，如果函数返回 Some，则保留结果。
let chars: Vec<char> = "hello".chars().filter_map(|c| if c.is_alphabetic() { Some(c.to_ascii_uppercase()) } else { None }).collect();

map_while()对迭代器中的元素应用一个函数，直到函数返回 None。
let first_three = (1..).into_iter().map_while(|x| if x <= 3 { Some(x) } else { None });

take_while()从迭代器中取出满足某个条件的元素，直到不满足为止。
let first_five = (1..10).into_iter().take_while(|x| x <= 5).collect::<Vec<_>>()

skip_while()跳过迭代器中满足某个条件的元素，直到不满足为止。
let odds: Vec<i32> = (1..10).into_iter().skip_while(|x| x % 2 == 0).collect();

for_each()对迭代器中的每个元素执行某种操作。
let mut counter = 0; (1..5).into_iter().for_each(|x| counter += x);

fold()对迭代器中的元素进行折叠，使用一个累加器。
let sum: i32 = (1..5).into_iter().fold(0, |acc, x| acc + x);

try_fold()对迭代器中的元素进行折叠，可能在遇到错误时提前返回。
let result: Result = (1..5).into_iter().try_fold(0, |acc, x| if x == 3 { Err("Found the number 3") } else { Ok(acc + x) });

scan()对迭代器中的元素进行状态化的折叠。
let sum: Vec<i32> = (1..5).into_iter().scan(0, |acc, x| { *acc += x; Some(*acc) }).collect();

take()从迭代器中取出最多 n 个元素。
let first_five = (1..10).into_iter().take(5).collect::<Vec<_>>()

skip()跳过迭代器中的前 n 个元素。
let after_five = (1..10).into_iter().skip(5).collect::<Vec<_>>()

zip()将两个迭代器中的元素打包成元组。
let zipped = (1..3).zip(&['a', 'b', 'c']).collect::<Vec<_>>()

cycle()重复迭代器中的元素，直到无穷。
let repeated = (1..3).into_iter().cycle().take(7).collect::<Vec<_>>()

chain()连接多个迭代器。
let combined = (1..3).chain(4..6).collect::<Vec<_>>()

rev()反转迭代器中的元素顺序。
let reversed = (1..4).into_iter().rev().collect::<Vec<_>>()

enumerate()为迭代器中的每个元素添加索引。
let enumerated = (1..4).into_iter().enumerate().collect::<Vec<_>>()

peeking_take_while()取出满足条件的元素，同时保留迭代器的状态，可以继续取出后续元素。
let (first, rest) = (1..10).into_iter().peeking_take_while(|&x| x < 5);

step_by()按照指定的步长返回迭代器中的元素。
let even_numbers = (0..10).into_iter().step_by(2).collect::<Vec<_>>()

fuse()创建一个额外的迭代器，它在迭代器耗尽后仍然可以调用 next() 方法。
let mut iter = (1..5).into_iter().fuse(); while iter.next().is_some() {}

inspect()在取出每个元素时执行一个闭包，但不改变元素。
let mut counter = 0; (1..5).into_iter().inspect(|x| println!("Inspecting: {}", x)).for_each(|x| println!("Processing: {}", x));

same_items()比较两个迭代器是否产生相同的元素序列。
let equal = (1..5).into_iter().same_items((1..5).into_iter());
*/
