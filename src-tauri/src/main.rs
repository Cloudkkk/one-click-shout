// 防止在Windows的发布版本中出现额外的控制台窗口，不要删除这行代码！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 引入device_query库，用于监控键盘事件
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, KeyboardControllable, Key};
use tauri::Manager;
use std::sync::{Arc, Mutex};

const USER_KEY_CHANNEL: &str = "user_key";
const PRESS_CHANNEL: &str = "key_pressed";
const SWITCH_CHANNEL: &str ="switch_channel"

//主函数
fn main() {
  let device_state = DeviceState::new();

  // 使用默认的tauri构建器
  tauri::Builder::default()
  .setup(move |app| {
        let window = app.get_window("main").unwrap();
        let window = Arc::new(Mutex::new(window));
        let window_clone = Arc::clone(&window);
        // 克隆标志位的引用，以便在闭包中使用
        let switch_flag_clone = Arc::clone(&switch_flag);

        // 监听'switch_channel'事件
        app.listen(SWITCH_CHANNEL, move |event| {
            // 获取前端发送的开关状态
            let switch_state = event.payload().unwrap_or(&"true".to_string()).parse::<bool>().unwrap_or(true);
            // 更新标志位
            *switch_flag_clone.lock().unwrap() = switch_state;
            Ok(())
        });
        // 在后端监听前端发送的事件
        app.listen(USER_KEY_CHANNEL, move |event| {
            let key_from_frontend = event.payload().unwrap_or("A").to_string();
            let user_key = match key_from_frontend.as_str() {
                "A" => Keycode::A,
                "B" => Keycode::B,
                // 你可以根据需要添加更多的匹配项
                _ => Keycode::A, // 默认值
            };
            std::thread::spawn(move || loop {
                // 检查标志位，如果为false则跳过本次循环
                if !*switch_flag.lock().unwrap() {
                    continue;
                }
                let mut enigo = Enigo::new();
                let keys = device_state.get_keys();
                if keys.contains(user_key) {
                    let window = window_clone.lock().unwrap();
                    enigo.key_down(Key::Shift);
                    enigo.key_click(Key::Return);
                    enigo.key_up(Key::Shift);
                    window.emit(PRESS_CHANNEL, None::<()>).unwrap();
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            });
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
