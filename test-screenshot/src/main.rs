use screenshots::Screen;
use std::{fs, time::Instant};

fn main() {
    let path = String::from("./");
    // let img_name = String::from("{}.png");
    let mut start;
    let screens = Screen::all().unwrap();
    fs::create_dir_all(&path).unwrap();
    for screen in screens {
        if screen.display_info.is_primary {
            start = Instant::now();
            println!("屏幕： {screen:?}");
            let image = screen.capture().unwrap();
            let buffer = image.buffer();
            fs::write(format!("./{}.png", screen.display_info.id), buffer).unwrap();
            println!("运行耗时: {:?}", start.elapsed());
        }
    }

    start = Instant::now();
    // 获取点所在屏幕
    let screen = Screen::from_point(100, 100).unwrap();
    println!("点所在屏幕： {screen:?}");
    let image = screen.capture_area(0, 0, 1024, 768).unwrap();
    let buffer = image.buffer();
    fs::write("./capture_display_with_point.png", buffer).unwrap();
    println!("运行耗时: {:?}", start.elapsed());
}
