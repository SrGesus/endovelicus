<template>
  <v-navigation-drawer
    v-model="drawerStore.drawer"
    prominent
    border="primary"
  >
    <v-list>
      <v-list-item
        v-for="(item, index) in navBarItems"
        :key="index"
        :to="item.path"
        class="font-weight-bold align-middle"
      >
        <template #prepend>
          <v-icon>{{ item.icon }}</v-icon>
        </template>
        <v-list-item-title class="font-weight-medium">
          {{ item.name }}
        </v-list-item-title>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
const drawerStore = useDrawerStore()

const navBarItems = [
  {
    name: 'Home',
    icon: 'mdi-home',
    path: '/home',
  },
  {
    name: 'Currencies',
    icon: 'mdi-currency-eur',
    path: '/currency',
  },
  {
    name: 'Settings',
    icon: 'mdi-cog',
    path: '/settings',
  },
]
</script>
