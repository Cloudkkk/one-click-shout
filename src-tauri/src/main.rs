// 防止在Windows的发布版本中出现额外的控制台窗口，不要删除这行代码！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 引入device_query库，用于监控键盘事件
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, Key, KeyboardControllable};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use tauri::Manager;
// 引入clipboard库
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

const _USER_KEY_CHANNEL: &str = "user_key_channel";
const KEY_PRESS_CHANNEL: &str = "key_press_channel";
// 创建一个全局变量来存储is_listening的状态
static IS_LISTENING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

#[tauri::command]
fn switch_command(switch_value: bool) -> String {
    let mut is_listening = IS_LISTENING.lock().unwrap();
    *is_listening = switch_value;
    println!("前端传来的值是: {}", switch_value);
    if switch_value {
        "Switch Opened!".into()
    } else {
        "Switch Closed!".into()
    }
}

//主函数
fn main() {
    let device_state = DeviceState::new();

    // 使用默认的tauri构建器
    tauri::Builder::default()
        .manage(IS_LISTENING.clone())
        .invoke_handler(tauri::generate_handler![switch_command])
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            let window = Arc::new(Mutex::new(window));
            let window_clone = Arc::clone(&window);
            std::thread::spawn(move || loop {
                let mut enigo = Enigo::new();
                let keys = device_state.get_keys();
                let is_listening = IS_LISTENING.lock().unwrap();
                // if *is_listening && keys.contains(&Keycode::O) {
                if keys.contains(&Keycode::O) {
                    let window = window_clone.lock().unwrap();
                    enigo.key_down(Key::Shift);
                    enigo.key_click(Key::Return);
                    enigo.key_up(Key::Shift);
                    // 创建一个剪切板上下文
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    // 将字符串复制到剪切板
                    ctx.set_contents("姐姐晚上一个人睡吗OvO".to_owned())
                        .unwrap();
                    // 模拟粘贴热键
                    #[cfg(target_os = "macos")]
                    let control_key = Key::Meta;
                    #[cfg(target_os = "windows")]
                    let control_key = Key::Control;
                    enigo.key_down(control_key); // 使用 Meta 键代替 Control 键
                    enigo.key_click(Key::Layout('v'));
                    enigo.key_up(control_key);
                    window.emit(KEY_PRESS_CHANNEL, None::<()>).unwrap();
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
