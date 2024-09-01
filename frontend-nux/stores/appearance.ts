import type { ThemeDefinition, ThemeInstance } from 'vuetify'
import { useTheme } from 'vuetify'
import { createTheme } from 'vuetify/lib/composables/theme.mjs'

type DeepWriteable<T> = {
  -readonly [P in keyof T]: T[P] extends object ? DeepWriteable<T[P]> : T[P];
}

if (!import.meta.client) {
  throw new Error('SANITY CHECK: appearance store should only be used on the client')
}

export const useAppearanceStore = defineStore('appearance', {
  state: () => {
    return {
      themeDefinitions: ref({} as Record<string, ThemeDefinition>),
      theme: ref(useTheme() as DeepWriteable<ThemeInstance>),
    }
  },
  actions: {
    add(name: string, theme: ThemeDefinition) {
      this.themeDefinitions[name] = theme
      this.theme.themes[name] = createTheme({ themes: { theme } }).themes.value.theme
    },

    remove(name: string) {
      // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
      delete this.themeDefinitions[name]
      // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
      delete this.theme.themes[name]
    },

    set(name: string) {
      this.theme.global.name = name
    },

    list() {
      return Object.keys(this.theme.themes)
    },

    list_def() {
      return Object.keys(this.themeDefinitions)
    },

  },
  persist: {
    storage: persistedState.localStorage,
    key: 'appearance',
    paths: ['themeDefinitions', 'theme'],
  },
})
