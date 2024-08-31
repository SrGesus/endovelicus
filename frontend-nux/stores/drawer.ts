export const useDrawerStore = defineStore('drawer', {
  state: () => ({
    drawer: true,
  }),
  actions: {
    toggleDrawer() {
      this.drawer = !this.drawer
    },
  },
  persist: true,
})
