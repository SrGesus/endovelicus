<template>
  <div>
    <ClientOnly>
      <AppBar />
      <SideBar />
      <v-main
        class="min-h-screen bg-background"
      >
        <slot />
      </v-main>
      <template #fallback>
        <LoadingScreen />
      </template>
    </ClientOnly>
  </div>
</template>
