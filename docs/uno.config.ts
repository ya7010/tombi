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
        border: "rgba(255,255,255,0.2)", // 枠線用の半透明白
        "border-focus": "rgba(255,255,255,0.3)", // フォーカス時の枠線用
      },
    },
  },
  // カスタムルールやショートカットが必要な場合はここに追加
  shortcuts: {
    "nav-link": "px-4 py-2 hover:text-gray-600 transition-colors",
    "input-focus":
      "ring-1 ring-tombi-border focus:ring-tombi-border-focus focus:outline-none transition-colors",
    "btn-focus":
      "p-2 focus:outline-none focus-visible:ring-2 focus-visible:ring-tombi-border-focus transition-colors rounded-lg",
  },
  rules: [
    [
      "material-symbols-rounded",
      {
        "font-family": '"Material Symbols Rounded"',
        "font-weight": "400",
        "font-style": "normal",
        display: "inline-block",
        "line-height": "1",
        "text-transform": "none",
        "letter-spacing": "normal",
        "word-wrap": "normal",
        "white-space": "nowrap",
        direction: "ltr",
        "-webkit-font-smoothing": "antialiased",
        "font-variation-settings": "'FILL' 1",
      },
    ],
  ],
});
