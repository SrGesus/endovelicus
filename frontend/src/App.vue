<template>
  <v-app>
    <v-main>
      <NavBar />
      <SideBar />
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
  import NavBar from '@/components/NavBar.vue'
  import SideBar from '@/components/SideBar.vue'
    //
</script>
