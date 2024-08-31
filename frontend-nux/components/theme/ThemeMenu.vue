<template>
  <ClientOnly>
    <ThemeMenuInner />
    <template #fallback>
      <v-btn
        append-icon="mdi-chevron-down"
        color="primary"
        text="Theme"
      />
    </template>
  </ClientOnly>
</template>
