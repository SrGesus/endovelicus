<template>
  <div>
    <ClientOnly>
      <AppBar />
      <SideBar />
      <v-main
        class="min-h-screen m"
      >
        <slot />
      </v-main>
      <template #fallback>
        <LoadingScreen />
      </template>
    </ClientOnly>
  </div>
</template>
