declare module 'vuetify/lib/composables/theme' {
  function createTheme(options?: ThemeOptions): ThemeInstance & { install: (app: App) => void }
  export { createTheme }
}
