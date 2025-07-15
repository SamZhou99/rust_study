mod fruit {
    pub mod apple {
        pub fn all() {
            // 包的绝对路径
            crate::fruit::apple::red();
            crate::fruit::apple::green();
            // 包的相对路径
            super::banana::big_banana();
            super::banana::small_banana();
        }
        // 默认都是 私有private 方法
        pub fn red() {
            println!("红苹果");
        }
        fn green() {
            println!("青苹果");
        }
    }
    mod banana {
        pub fn big_banana() {
            println!("大香蕉");
        }
        pub fn small_banana() {
            println!("小香蕉");
        }
    }
}

mod food {
    pub fn all() {
        println!("------ crate ------");
        crate::fruit::apple::all();
        println!("------ super ------");
        super::fruit::apple::red();
        println!("------ vegetable ------");
        test_mod::vegetable::tomato();
        test_mod::vegetable::potato::all();
        println!("------ utils ------");
        test_mod::utils::config::get_config();
    }
}

fn main() {
    food::all();
}
