/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        sol: '#FF6B35',
        lua: '#004E89',
        encontro: '#9B59B6',
        aurora: '#FFB347',
        crepusculo: '#2C3E50',
        espuma: '#F5F5F0',
        fundo: '#F5F5F0',
        'fundo-dark': '#0A0A0A',
        surface: '#FFFFFF',
        'surface-dark': '#1A1A1A',
        texto: '#1A1A1A',
        'texto-dark': '#F5F5F0',
        'texto-secundario': '#666666',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        display: ['Playfair Display', 'Georgia', 'serif'],
      },
      animation: {
        'water-flow': 'waterFlow 15s ease infinite',
        'float': 'float 6s ease-in-out infinite',
        'fade-in-up': 'fadeInUp 0.8s ease-out forwards',
      },
      keyframes: {
        waterFlow: {
          '0%, 100%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
        fadeInUp: {
          '0%': { opacity: '0', transform: 'translateY(30px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
      },
    },
  },
  plugins: [],
  darkMode: 'class',
};
