import { defineConfig, presetUno, presetTypography } from "unocss";

export default defineConfig({
  presets: [presetUno(), presetTypography()],
  // カスタムルールやショートカットが必要な場合はここに追加
  shortcuts: {
    "nav-link": "px-4 py-2 hover:text-gray-600 transition-colors",
  },
});
