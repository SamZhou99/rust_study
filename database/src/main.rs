// use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*; // 用来处理日期

fn main() {
    println!("链接数据库");

    let url = "mysql://root:root@127.0.0.1:3306/test";

    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // 第一种方法
    // conn.query_iter("SELECT * FROM list")
    //     .unwrap()
    //     .for_each(|row| {
    //         let r: (i32, String, String) = from_row(row.unwrap());
    //         println!("{}, {}, {:?}", r.0, r.1, r.2);
    //     });

    // 第二种方法
    let row: Vec<(i32, String, String)> = conn
        .query("SELECT * FROM list ORDER BY id DESC LIMIT 100")
        .unwrap();
    for r in row {
        println!("id={},key={},value={}", r.0, r.1, r.2);
    }

    println!("结束");
}
