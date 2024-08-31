<template>
  <v-app-bar
    class="px-4 text-primary"
    prominent
  >
    <v-btn
      v-if="$vuetify.display.mdAndDown"
      icon="mdi-menu hidden md:display"
      @click="drawerStore.toggleDrawer"
    />
    <v-btn
      v-else
      size="x-large"
      to="/"
    >
      <svg-logo height="1.75rem" />
    </v-btn>
    <v-spacer />
    <div class="text-capitalize text-h5 font-weight-bold select-none">
      {{ router.currentRoute.value.name }}
    </div>
    <v-spacer />
    <ThemeMenu />
    <v-btn
      class="h-full"
      href="https://github.com/SrGesus/endovelicus"
      icon
      target="_blank"
    >
      <v-icon
        size="3rem"
      >
        mdi-github
      </v-icon>
    </v-btn>
  </v-app-bar>
</template>

<script lang="ts" setup>
import { useRouter } from '#app'

const router = useRouter()

const drawerStore = useDrawerStore()
</script>
