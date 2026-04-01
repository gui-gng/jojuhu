# Jojuhu Website

Website institucional da rede social Jojuhu.

## 🚀 Tecnologias

- **Astro 4.x** - Framework web moderno
- **Tailwind CSS 3.x** - Framework CSS utility-first
- **TypeScript** - Tipagem estática
- **pnpm** - Package manager rápido e eficiente

## 📁 Estrutura

```
website/
├── src/
│   ├── components/     # Componentes reutilizáveis
│   ├── layouts/        # Layouts de página
│   ├── pages/          # Páginas do site
│   ├── i18n/           # Traduções
│   └── styles/         # Estilos globais
├── public/             # Assets estáticos
│   └── images/
│       ├── logo.svg       # Logo completo
│       └── logo-icon.svg  # Logo ícone (favicon)
└── package.json
```

## 🎨 Cores

- **Sol**: #FF6B35 (Rio Solimões)
- **Lua**: #004E89 (Rio Negro)
- **Encontro**: #9B59B6 (Onde se encontram)

## 📝 Páginas

1. **Home** (`/`) - Landing page principal
2. **Funcionalidades** (`/funcionalidades`) - Features do produto
3. **Sobre** (`/sobre`) - História e valores
4. **Baixar** (`/baixar`) - Download do app
5. **Privacidade** (`/privacidade`) - Política de privacidade
6. **Termos** (`/termos`) - Termos de uso
7. **404** (`/404`) - Página não encontrada

## 🛠️ Desenvolvimento com pnpm

### Pré-requisitos

- Node.js >= 18.0.0
- pnpm >= 8.0.0

```bash
# Instalar pnpm (se não tiver)
npm install -g pnpm

# Ou via Homebrew (macOS)
brew install pnpm
```

### Comandos

```bash
# Clonar/entrar no projeto
cd website

# Instalar dependências
pnpm install

# Iniciar servidor de desenvolvimento
pnpm dev

# Build de produção
pnpm build

# Preview do build
pnpm preview

# Limpar build e node_modules
pnpm clean
```

## 🌐 Internacionalização

- Idioma padrão: **PT-BR**
- Estrutura preparada para: **EN** (em breve)

## 🎭 Temas

- Light mode (padrão)
- Dark mode (toggle no header)

## 📱 Responsividade

- Mobile-first design
- Breakpoints: sm (640px), md (768px), lg (1024px), xl (1280px)

## 🚢 Deploy

O site é estático e pode ser hospedado em qualquer serviço de hosting estático:

### Locaweb (recomendado)

```bash
# Build do projeto
pnpm build

# Upload da pasta dist/ via FTP
# Configurar domínio: jojuhu.com
# Ativar SSL
```

### Outras opções

- Netlify
- Vercel
- GitHub Pages
- Cloudflare Pages

## 📄 Licença

MIT License - Veja LICENSE para detalhes.

## 🎨 Logo

O logo do Jojuhu representa:
- **"J" caligráfico/manual** - Escrito como água fluindo
- **Dois rios** - Rio Solimões (laranja) e Rio Negro (azul)
- **Inspiração Ying Yang** - Equilíbrio entre forças opostas
- **Encontro das Águas** - Onde os rios se encontram mas não se misturam imediatamente

---

**Jojuhu** - Onde correntes distintas criam algo maior.
