<template>
  <v-app-bar class="pr-4 flex text-primary" prominent>
    <v-btn to="home" size="x-large">
      <Logo />
    </v-btn>
    <v-spacer></v-spacer>
    <div class="text-capitalize text-h5 font-weight-bold">
      {{  router.currentRoute.value.name }}
    </div>
    <v-spacer></v-spacer>
    <ThemeMenu />
    <v-btn target="_blank" href="https://github.com/SrGesus/endovelicus" icon class="h-full">
      <v-icon
        size="3rem"
      >mdi-github</v-icon>
    </v-btn>
  </v-app-bar>
</template>

<script lang="ts" setup>
import router from '@/router';
import Logo from '@/components/Logo.vue';
import ThemeMenu from '@/components/ThemeMenu.vue';

console.log(router);
console.log(router.currentRoute.value.path);
</script>