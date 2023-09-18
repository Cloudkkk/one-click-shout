import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  // 禁止 Vite 在运行时清除屏幕，以防止它隐藏 Rust 错误信息
  clearScreen: false,
   // 严格模式，确保指定的端口可用，否则会报错
  server: {
    port: 3001,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Windows 使用 Chromium | macOS & Linux 使用 WebKit
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // 调试构建时不启用代码压缩
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // 为调试构建生成源代码映射 (sourcemap)
    sourcemap: !!process.env.TAURI_DEBUG,
  }
})
