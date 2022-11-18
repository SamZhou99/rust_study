// use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*; // 用来处理日期

fn main() {
    println!("链接数据库");

    let url = "mysql://root:root@127.0.0.1:3306/test";

    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_iter("SELECT * FROM list")
        .unwrap()
        .for_each(|row| {
            let r: (i32, String, String) = from_row(row.unwrap());
            println!("{}, {}, {:?}", r.0, r.1, r.2);
        });

    // for user in res {
    //     println!("id={},key={},value={}", user.id, user.key, user.value);
    // }

    println!("结束");
}
