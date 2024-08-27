<template>
  <v-navigation-drawer>
    <v-list>
      <v-list-item
        v-for="(item, index) in navBarItems"
        :key="index"
        :to="item.path"
      >
        <v-icon>{{ item.icon }}</v-icon>
        {{  item.name }}
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
const navBarItems = [
  {
    name: 'Home',
    icon: 'mdi-home',
    path: '/home'
  },
  {
    name: 'Currencies',
    icon: 'mdi-currency-eur',
    path: '/currency'
  }
]
</script>