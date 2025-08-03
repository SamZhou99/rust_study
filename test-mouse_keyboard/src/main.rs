use enigo::Button;
use enigo::Coordinate::Abs;
use enigo::Direction::Click;
use enigo::Enigo;
use enigo::Keyboard;
use enigo::Mouse;
use enigo::Settings;
use rand::{self, random_range};
use screenshots::Screen;
use std::{thread, time};

fn main() {
    demo02();
    demo01();
}

fn demo01() {
    let ten_millis = time::Duration::from_millis(100);

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (sw, sh) = get_screen_size();
    let sw = (sw / 2).try_into().unwrap();
    let sh = (sh - 100).try_into().unwrap();

    enigo.move_mouse(100, 100, Abs).unwrap();
    thread::sleep(ten_millis);
    enigo.button(Button::Left, Click).unwrap();
    thread::sleep(ten_millis);

    for i in 1..=20 {
        let sw = random_range(sw..sw + 10);
        let sh = random_range(sh..sh + 10);
        // println!("{} {}", sw, sh);
        enigo.move_mouse(sw, sh, Abs).unwrap();
        thread::sleep(ten_millis);
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(ten_millis);

        let (text, text_len) = text_info(i);
        enigo.text(&text).unwrap();
        temp_move_mouse(&mut enigo, text_len + 10);
        thread::sleep(ten_millis);
    }
    enigo.text("------结束------\n").unwrap();
    temp_move_mouse(&mut enigo, 20);
}

fn demo02() {
    let ten_millis = time::Duration::from_millis(100);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    for i in 1..=100 {
        let sw = random_range(100..110);
        let sh = random_range(100..110);
        enigo.move_mouse(sw, sh, Abs).unwrap();
        println!("{} {} {}", i, sw, sh);
        thread::sleep(ten_millis);
    }
}
fn get_screen_size() -> (usize, usize) {
    let screens = Screen::all().unwrap();
    for screen in screens {
        return (
            screen.display_info.width.try_into().unwrap(),
            screen.display_info.height.try_into().unwrap(),
        );
    }
    return (0, 0);
}

fn text_info(i: usize) -> (String, usize) {
    let text = "// Hello World! 博楷仔仔 好好学习 天天向上！......❤️\n";
    let text = format!("{} {}", num_to_str(i), text);
    (text.clone(), text.len())
}

fn num_to_str(i: usize) -> String {
    let num_str = if i < 10 {
        format!("0{}", i)
    } else {
        i.to_string()
    };
    num_str
}

fn temp_move_mouse(enigo: &mut Enigo, text_len: usize) {
    let (sw, sh) = get_screen_size();
    let sw = (sw / 2).try_into().unwrap();
    let sh = (sh / 2).try_into().unwrap();
    let ten_millis = time::Duration::from_millis(20);
    for _ in 0..text_len {
        let sw = random_range(sw..sw + 4);
        let sh = random_range(sh..sh + 2);
        enigo.move_mouse(sw, sh, Abs).unwrap();
        thread::sleep(ten_millis);
    }
}
