fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) } else { Err(false) }
}

fn g(i: i32) -> Result<i32, bool> {
    /*
     * ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
     * 所以，? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致。
     */
    let t = f(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

pub fn demo_result() {
    let r = g(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}
