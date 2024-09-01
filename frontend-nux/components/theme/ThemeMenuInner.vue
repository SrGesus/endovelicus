<!-- IMPORTANT: This component must be wrapped in a ClientOnly component -->
<template>
  <v-btn
    append-icon="mdi-chevron-down"
    color="primary"
  >
    Theme
    <v-menu
      activator="parent"
      :close-on-content-click="false"
    >
      <v-list>
        <v-list-item
          v-for="(item, index) in Object.keys(appearance.theme.themes).sort()"
          :key="index"
          :value="index"
          :variant="appearance.theme.global.name === item ? 'tonal' : undefined"
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

const appearance = useAppearanceStore()

const changeTheme = (themeName: string) => {
  appearance.set(themeName)
}
</script>
