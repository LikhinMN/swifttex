import { defineConfig } from 'vitepress'

export default defineConfig({
  base: '/swifttex/',
  title: "SwiftTeX",
  description: "Fast, accurate LaTeX math rendering engine.",
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Docs', link: '/introduction' }
    ],
    sidebar: [
      {
        text: 'Overview',
        items: [
          { text: 'Introduction', link: '/introduction' },
          { text: 'Getting Started', link: '/getting-started' }
        ]
      },
      {
        text: 'Framework Guides',
        items: [
          { text: 'React', link: '/guides/react' },
          { text: 'Vue 3', link: '/guides/vue' },
          { text: 'Svelte', link: '/guides/svelte' },
          { text: 'Web Components', link: '/guides/web-components' }
        ]
      },
      {
        text: 'Reference',
        items: [
          { text: 'API Reference', link: '/api-reference' }
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/LikhinMN/swifttex' }
    ]
  }
})
