<template>
  <v-responsive
    class="align-top mx-10 py-16 justify-center fill-height"
  >
    <h2>Themes</h2>
    <v-divider />
    <div class="pl-2 pt-5">

      <v-expansion-panels>
        <v-expansion-panel
          v-for="(theme, index) in Object.keys(store.themeDefinitions)"
          :key="index"
        >
          <v-expansion-panel-title>
            <h3 class="text-capitalize">{{ theme }}</h3>
          <!-- <v-icon class="ml-2" color="primary" icon="mdi-pencil" /> -->
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-switch
              v-model="store.themeDefinitions[theme].dark"
              :label="store.themeDefinitions[theme].dark ? 'Dark' : 'Light'"
            />
            <v-row
              v-if="store.themeDefinitions[theme].colors"
              width="10$"
            >
              <v-col cols="12"><h4 class="text-capitalize">Colors</h4></v-col>
              <v-col
                v-for="(color, j) in Object.keys(store.themeDefinitions[theme].colors!)"
                :key="j"
                cols="auto"
              >
                <v-btn
                  append-icon="mdi-palette"
                  class="text-capitalize fill-width"
                  :color="store.themeDefinitions[theme].colors![color]"
                  width="10rem"
                >
                  {{ color }}
                  <v-menu activator="parent" :close-on-content-click="false">
                    <v-color-picker
                      v-model="store.themeDefinitions[theme].colors![color]"
                      hide-inputs
                      @update:model-value="updateTheme(theme)"
                    />
                  </v-menu>
                </v-btn>
              </v-col>

            </v-row>
            <h4 class="text-capitalize my-5">JSON</h4>
            <v-code class="my-5">
              <pre>{{ JSON.stringify(store.themeDefinitions[theme], null, 2) }}</pre>
            </v-code>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>

      <v-text-field
        v-if="dialog"
        v-model="theme"
        append-icon="mdi-close"
        block
        class="mt-5"
        placeholder="Theme Name"
        @click:append="dialog = false; theme = ''"
        @keyup.enter="console.log('enter')"
      />
      <v-btn
        class="my-5"
        color="primary"
        prepend-icon="mdi-plus"
        text="Create Theme"
        width="100%"
        @click="newTheme"
      />
    </div>
  </v-responsive>
</template>

<script lang="ts" setup>
  import { useAppearanceStore } from '@/stores/appearance'

  const store = useAppearanceStore()

  const dialog = ref(false)

  const theme = ref('')

  console.log(store.themeDefinitions)

  const newTheme = () => {
    if (dialog.value) {
      if (theme.value) {
        const test = store.theme.themes[store.theme.global.name]
        const themeDef = {
          dark: test.dark,
          colors: Object.fromEntries(
            Object.entries(test.colors)
              .filter(([key]) => !key.includes('-')),
          ),
          variables: test.variables,
        }

        store.add(theme.value, themeDef)
      }
    }
    dialog.value = !dialog.value
  }

  const updateTheme = (theme: string) => {
    store.add(theme, store.themeDefinitions[theme])
  }
</script>
