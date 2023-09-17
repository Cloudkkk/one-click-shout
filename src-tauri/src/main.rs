// 防止在Windows的发布版本中出现额外的控制台窗口，不要删除这行代码！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 引入device_query库，用于监控键盘事件
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, KeyboardControllable, Key};
use tauri::Manager;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
// 引入clipboard库
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

const _USER_KEY_CHANNEL: &str = "user_key_channel";
const KEY_PRESS_CHANNEL: &str = "key_press_channel";
const SWITCH_CHANNEL: &str ="switch_channel";
// 创建一个全局变量来存储is_listening的状态
static IS_LISTENING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

#[tauri::command]
fn switch_command(switch_value: bool)-> String {    
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
            if *is_listening && keys.contains(&Keycode::O) {
                let window = window_clone.lock().unwrap();
                enigo.key_down(Key::Shift);
                enigo.key_click(Key::Return);
                enigo.key_up(Key::Shift);
                // 创建一个剪切板上下文
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                // 将字符串复制到剪切板
                ctx.set_contents("你要复制的字符串".to_owned()).unwrap();
                // 模拟粘贴热键
                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout('v'));
                enigo.key_up(Key::Control);
                window.emit(KEY_PRESS_CHANNEL, None::<()>).unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        });
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }