module.exports = {
  base: '/',
  title: 'Glamour',
  themeConfig: {
    nav: [
      { text: 'Technical Report', link: '/tech/' },
      { text: 'Development Notes', link: '/dev/' },
      { text: 'API', link: '/api/' },
    ],
    repo: 'smeagolem/glamour',
    lastUpdated: true,
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
