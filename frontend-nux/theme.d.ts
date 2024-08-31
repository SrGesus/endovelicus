declare module 'vuetify/lib/composables/theme.mjs' {
  function createTheme(options?: ThemeOptions): ThemeInstance & { install: (app: App) => void }
  export { createTheme }
}
