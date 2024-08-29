<template>
  <v-app-bar border class="pr-4 flex text-primary" prominent>
    <v-btn v-if="$vuetify.display.mdAndDown" icon="mdi-menu" @click="drawer = !drawer" />
    <v-btn v-if="$vuetify.display.smAndUp" size="x-large" to="/">
      <Logo height="1.75rem" />
    </v-btn>
    <v-spacer />
    <div class="text-capitalize text-h5 font-weight-bold">
      {{ router.currentRoute.value.name }}
    </div>
    <v-spacer />
    <ThemeMenu />
    <v-btn class="h-full" href="https://github.com/SrGesus/endovelicus" icon target="_blank">
      <v-icon
        size="3rem"
      >mdi-github</v-icon>
    </v-btn>
  </v-app-bar>
</template>

<script lang="ts" setup>
  import router from '@/router'
  import Logo from '@/components/Logo.vue'
  import ThemeMenu from '@/components/ThemeMenu.vue'
  import { drawer } from './sidebar'

  console.log(router)
  console.log(router.currentRoute.value.path)
</script>
