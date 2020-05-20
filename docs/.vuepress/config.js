module.exports = {
  base: '/',
  title: 'Glamour',
  themeConfig: {
    nav: [
      { text: 'Technical Report', link: '/tech/' },
      { text: 'Development Notes', link: '/dev/' },
      { text: 'API', link: '/doc/glamour/index.html', target: '_blank', rel: 'noopener noreferrer' },
    ],
    repo: 'smeagolem/glamour',
    lastUpdated: true,
    sidebar: {
      '/tech/': [
        {
          title: 'Technical Report',
          collapsable: false,
          sidebarDepth: 2,
          children: [
            ['', 'Introduction 🚧'],
            'technology',
            'foundation',
            'architecture',
            'main-loop',
            'renderer',
            'assets',
            'data-collection',
            'tests-docs',
            'proposal-changes',
            'future-work',
          ],
        },
      ],
      '/dev/': [
        {
          title: 'Development Notes',
          collapsable: false,
          sidebarDepth: 2,
          children: [
            ['', 'Table of Contents'],
            '2020-04-23-basic-lighting',
            '2020-04-20-the-crit-path-to-deferred-shading',
            '2020-04-16-finding-problems',
          ],
        },
      ],
    },
  },
  evergreen: true,
  plugins: {
    '@vuepress/medium-zoom': {
      options: {
        margin: 16
      }
    }
  }
}
