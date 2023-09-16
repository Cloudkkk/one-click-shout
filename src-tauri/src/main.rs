// 防止在Windows的发布版本中出现额外的控制台窗口，不要删除这行代码！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 引入device_query库，用于监控键盘事件
use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::Manager;
use std::sync::{Arc, Mutex};
//主函数
fn main() {
  let device_state = DeviceState::new();
  // 使用默认的tauri构建器
  tauri::Builder::default()
  .setup(move |app| {
        let window = app.get_window("main").unwrap();
        let window = Arc::new(Mutex::new(window));
        let window_clone = Arc::clone(&window);
        std::thread::spawn(move || loop {
            let keys = device_state.get_keys();
            if keys.contains(&Keycode::A) {
                let window = window_clone.lock().unwrap();
                window.emit("f1_pressed", None::<()>).unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        });
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
