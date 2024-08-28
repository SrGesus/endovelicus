<template>
  <v-btn
    append-icon="mdi-chevron-down"
    color="primary"
  >
    Theme
    <v-menu activator="parent">
      <v-list>
        <v-list-item
          v-for="(item, index) in Object.keys(theme.themes.value).sort()"
          :key="index"
          :value="index"
          @click="changeTheme(item)"
        >
          <v-list-item-title class="text-capitalize">
            {{ item }}
          </v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-btn>
</template>

<script lang="ts" setup>
  import { useTheme } from 'vuetify'

  const theme = useTheme()

  const changeTheme = (themeName: string) => {
    theme.global.name.value = themeName
  }
</script>
