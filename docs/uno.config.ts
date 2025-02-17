import { defineConfig, presetUno, presetTypography } from "unocss";

export default defineConfig({
  presets: [presetUno(), presetTypography()],
  theme: {
    colors: {
      tombi: {
        primary: "rgb(0,0,102)", // メインカラー
        50: "rgb(230,230,255)",
        100: "rgb(204,204,255)",
        200: "rgb(153,153,255)",
        300: "rgb(102,102,255)",
        400: "rgb(51,51,255)",
        500: "rgb(0,0,255)",
        600: "rgb(0,0,204)",
        700: "rgb(0,0,153)",
        800: "rgb(0,0,102)",
        900: "rgb(0,0,102)", // メインカラー（濃い青）
        focus: "rgba(255,255,255, 0.8)", // 枠線用の半透明白
      },
    },
    animation: {
      keyframes: {
        "spin-fast":
          "{from{transform:rotate(0deg)}to{transform:rotate(360deg)}}",
        shake:
          "{0%,100%{transform:rotate(0deg)}25%{transform:rotate(-10deg)}75%{transform:rotate(10deg)}}",
      },
      durations: {
        "spin-fast": "0.5s",
        shake: "0.7s",
      },
      timingFns: {
        "spin-fast": "linear",
        shake: "ease-in-out",
      },
      counts: {
        "spin-fast": "infinite",
        shake: "infinite",
      },
    },
  },
  // カスタムルールやショートカットが必要な場合はここに追加
  shortcuts: {
    "btn-focus":
      "focus:outline-none focus-visible:ring-2 focus-visible:ring-tombi-focus transition-colors focus:rounded-lg",
  },
});
