pub mod req;

use std::process::Command;

pub fn lock_screen() {
    // 根据不同操作系统执行相应的锁屏命令
    match std::env::consts::OS {
        "windows" => crate::lock_windows(),
        "macos" => crate::lock_macos(),
        "linux" => crate::lock_linux(),
        os => eprintln!("不支持的操作系统: {}", os),
    }

    // lock_windows();
}

// Windows系统锁屏
pub fn lock_windows() {
    // 使用rundll32.exe调用user32.dll的LockWorkStation函数
    if let Err(e) = Command::new("rundll32.exe")
        .arg("user32.dll,LockWorkStation")
        .status()
    {
        eprintln!("Windows锁屏失败: {}", e);
    } else {
        println!("Windows系统已锁屏");
    }
}

// macOS系统锁屏
pub fn lock_macos() {
    // 使用osascript执行AppleScript命令锁屏
    if let Err(e) = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to keystroke \"q\" using {command down, control down}")
        .status()
    {
        eprintln!("macOS锁屏失败: {}", e);
    } else {
        println!("macOS系统已锁屏");
    }
}

// Linux系统锁屏
pub fn lock_linux() {
    println!("Linux系统锁屏");
    // Linux有多种桌面环境，尝试常见的锁屏命令
    let commands = [
        // GNOME桌面环境
        ("gnome-screensaver-command", &["--lock"][..]),
        // KDE桌面环境
        (
            "qdbus",
            &["org.freedesktop.ScreenSaver", "/ScreenSaver", "Lock"][..],
        ),
        // 通用命令
        ("xscreensaver-command", &["-lock"][..]),
        ("i3lock", &[]),
        ("slock", &[]),
    ];

    // 尝试每个命令，直到成功
    for (cmd, args) in commands.iter() {
        // println!("尝试命令: {} {:?}", cmd, args);
        match Command::new(cmd).args(*args).status() {
            Ok(status) if status.success() => {
                println!("Linux系统已锁屏 (使用命令: {})", cmd);
                return;
            }
            _ => continue,
        }
    }

    eprintln!("Linux锁屏失败: 未找到可用的锁屏命令，请确保已安装锁屏工具");
}
