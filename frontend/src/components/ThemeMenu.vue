<template>
  <v-btn
    color="primary"
    text="Create Theme"
    @click="createNewTheme"
  />
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
  import { useAppearanceStore } from '@/stores/appearance'
  import { ThemeDefinition, useTheme } from 'vuetify'

  const theme = useTheme()

  const newTheme: ThemeDefinition = {
    dark: false,
    colors: {
      primary: '#ff0000',
      secondary: '#00ff00',
      surface: '#0000ff',
      error: '#ff0000',
      info: '#00ff00',
      success: '#0000ff',
      warning: '#ff0000',
    },
  }

  const store = useAppearanceStore()

  const createNewTheme = () => {
    store.add('new theme', newTheme)
  }

  const changeTheme = (themeName: string) => {
    theme.global.name.value = themeName
  }
</script>
