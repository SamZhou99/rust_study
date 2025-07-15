#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
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
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home.kind);
    println!("{:?}", loopback.address);
}
