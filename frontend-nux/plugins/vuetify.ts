// import this after install `@mdi/font` package
import '@mdi/font/css/materialdesignicons.css'

import 'vuetify/styles'
import { createVuetify, type ThemeDefinition } from 'vuetify'

const autumn: ThemeDefinition = {
  dark: false,
  colors: {
    background: '#F1F1F1',
    surface: '#DBDBDB',
    primary: '#8C0327',
    secondary: '#D85251',
    error: '#B00020',
    info: '#2196F3',
    success: '#4CAF50',
    warning: '#FB8C00',
  },
}

export default defineNuxtPlugin((app) => {
  const vuetify = createVuetify({
    theme: {
      defaultTheme: 'autumn',
      themes: {
        autumn,
      },
    },
  })
  app.vueApp.use(vuetify)
})
