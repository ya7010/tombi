import { defineConfig, presetUno, presetTypography } from "unocss";

export default defineConfig({
  presets: [presetUno(), presetTypography()],
  theme: {
    colors: {
      tombi: {
        primary: "rgb(0,0,102)", // メインカラー
        50: "#e6e6ff",
        100: "#ccccff",
        200: "#9999ff",
        300: "#6666ff",
        400: "#3333ff",
        500: "#0000ff",
        600: "#0000cc",
        700: "#000099",
        800: "#000066",
        900: "rgb(0,0,102)", // メインカラー（濃い青）
      },
    },
  },
  // カスタムルールやショートカットが必要な場合はここに追加
  shortcuts: {
    "nav-link": "px-4 py-2 hover:text-gray-600 transition-colors",
  },
});
