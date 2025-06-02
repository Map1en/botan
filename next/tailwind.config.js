/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      spacing: {
        18: '4.5rem', // 72px - 匹配 MUI 的间距
      },
    },
  },
  corePlugins: {
    preflight: false, // 防止与 MUI 样式冲突
  },
  plugins: [],
};
