// Utilities
import { defineStore } from 'pinia'
import { ThemeDefinition, useTheme } from 'vuetify'
import { createTheme } from 'vuetify/lib/composables/theme'

export const useAppearanceStore = defineStore('app', {
  state: () => {
    const theme = ref(
      useTheme()
    )
    return { theme }
  },
  actions: {
    add (name: string, theme: ThemeDefinition) {
      this.theme.themes[name] = createTheme({ themes: { theme } }).themes.value.theme
    },

  },
  persist: true,
})
