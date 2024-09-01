<!-- This is a client only component -->
<!-- eslint-disable vue/no-v-text-v-html-on-component -->
<!-- eslint-disable vue/no-v-html -->
<template>
  <div>
    <v-expansion-panels
      class="my-4"
    >
      <v-expansion-panel
        v-for="(theme, index) in appearance.list_def()"
        :key="index"
        @group:selected="editting = ''; appearance.set(theme)"
      >
        <template #title>
          <h6 class="text-h6 text-capitalize">
            {{ theme }}
          </h6>
        </template>
        <template #text>
          <v-switch
            v-model="appearance.themeDefinitions[theme].dark"
            :label="appearance.themeDefinitions[theme].dark ? 'Dark' : 'Light'"
            color="primary"
            :disabled="editting === theme"
          />
          <h6 class="text-capitalize text-h6 mt-5">
            Colors
          </h6>
          <v-divider class="mb-5" />
          <v-row
            v-if="appearance.themeDefinitions[theme].colors"
            width="10rem"
          >
            <v-col
              v-for="(color, j) in Object.keys(appearance.themeDefinitions[theme].colors)"
              :key="j"
              cols="auto"
            >
              <v-btn
                append-icon="mdi-palette"
                class="text-capitalize fill-width"
                :color="appearance.themeDefinitions[theme].colors[color]"
                width="10rem"
              >
                {{ color }}
                <v-menu
                  activator="parent"
                  :close-on-content-click="false"
                >
                  <v-color-picker
                    v-model="appearance.themeDefinitions[theme].colors[color]"
                    hide-inputs
                    @update:model-value="updateTheme(theme)"
                  />
                </v-menu>
              </v-btn>
            </v-col>
          </v-row>
          <h6 class="text-capitalize text-h6 mt-5">
            Json
          </h6>
          <v-divider class="mb-5" />
          <template v-if="editting !== theme">
            <div class="p-4 mb-5 bg-[#2e3440] tracking-[0.009375em] rounded">
              <div
                v-html="highlighter.codeToHtml(JSON.stringify(appearance.themeDefinitions[theme], null, 2), { lang: 'json', theme: 'nord' })"
              />
            </div>
            <v-col>
              <v-row>
                <v-btn
                  append-icon="mdi-pencil"
                  text="Edit"
                  class="my-2"
                  color="primary"
                  @click="editTheme(theme)"
                />
                <v-spacer />
                <v-btn
                  append-icon="mdi-trash-can"
                  text="Delete"
                  class="my-2"
                  color="error"
                  @click="deleteTheme(theme)"
                />
              </v-row>
            </v-col>
          </template>
          <template v-else>
            <v-textarea
              v-model="json"
              rows="30"
              style="font-family: monospace;"
              auto-grow
              :rules="jsonRules"
              validate-on-blur
            />
            <v-btn
              append-icon="mdi-content-save"
              text="Save"
              class="my-2 mr-2"
              color="primary"
              :disabled="!valid"
              @click="saveTheme(theme)"
            />
            <v-btn
              text="Cancel"
              variant="tonal"
              color="primary"
              @click="editting = ''"
            />
          </template>
        </template>
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
      @keyup.enter="newTheme"
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
</template>

<script setup lang="ts">
import { createHighlighter } from 'shiki'

const appearance = useAppearanceStore()

const editting = ref('')
const json = ref('')
const valid = ref(true)

// Add a new theme
const newThemeName = ref('')
const dialog = ref(false)

const editTheme = (theme: string) => {
  editting.value = theme
  json.value = JSON.stringify(appearance.themeDefinitions[theme], null, 2)
  console.log(json.value)
}

const saveTheme = (theme: string) => {
  try {
    appearance.themeDefinitions[theme] = JSON.parse(json.value)
    editting.value = ''
    updateTheme(theme)
  }
  catch (e) {
    console.error(e)
  }
}

const deleteTheme = (theme: string) => {
  appearance.remove(theme)
}

const jsonRules = [
  (v: string) => {
    try {
      JSON.parse(v)
      valid.value = true
      return true
    }
    catch (e) {
      valid.value = false
      return (e as URIError).message
    }
  },
]

const updateTheme = (theme: string) => {
  appearance.add(theme, appearance.themeDefinitions[theme])
}

const newTheme = () => {
  if (dialog.value && newThemeName.value) {
    const raw = appearance.theme.themes[appearance.theme.global.name]
    const themeDef = {
      dark: raw.dark,
      colors: Object.fromEntries(
        Object.entries(raw.colors)
          .filter(([key]) => !key.includes('-')),
      ),
      variables: raw.variables,
    }
    appearance.add(newThemeName.value, themeDef)
  }
  newThemeName.value = ''
  dialog.value = !dialog.value
}

// `createHighlighter` is async, it initializes the internal and
// loads the themes and languages specified.
const highlighter = await createHighlighter({
  themes: ['nord'],
  langs: ['json'],
})
</script>
