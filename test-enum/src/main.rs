#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
    V9(String),
}

// // 检枚举定义方法
// impl IpAddrKind {
//     fn Call(&self) {}
// }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // println!("{:?}", home);
    // println!("{:?}", loopback);
}
