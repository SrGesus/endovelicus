// Utilities
import { defineStore } from 'pinia'
import { ThemeDefinition, ThemeInstance, useTheme } from 'vuetify'
import { createTheme } from 'vuetify/lib/composables/theme'

// TODO:
// I really hate this, find another way to make this.theme.global.name = name work
type DeepWriteable<T> = {
  -readonly [P in keyof T]: T[P] extends object ? DeepWriteable<T[P]> : T[P];
};

export const useAppearanceStore = defineStore('appearance', {
  state: () => {
    return {
      themeDefinitions: ref({} as Record<string, ThemeDefinition>),
      theme: ref(useTheme() as DeepWriteable<ThemeInstance>),
    }
  },
  actions: {
    add (name: string, theme: ThemeDefinition) {
      this.themeDefinitions[name] = theme
      this.theme.themes[name] = createTheme({ themes: { theme } }).themes.value.theme
    },

    remove (name: string) {
      delete this.themeDefinitions[name]
      delete this.theme.themes[name]
    },

    set (name: string) {
      this.theme.global.name = name
    },

  },
  persist: {
    storage: localStorage,
    key: 'appearance',
    paths: ['themeDefinitions', 'theme'],
  },
})
