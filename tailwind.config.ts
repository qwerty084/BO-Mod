import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./frontend/**/*.{vue,js,ts}"],
  theme: {
    extend: {},
  },
  plugins: [],
} satisfies Config;
