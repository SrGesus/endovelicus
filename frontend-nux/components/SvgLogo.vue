<template>
  <svg
    :class="props.class"
    :height="props.height"
    viewBox="0 0 39 5"
    :width="props.width"
    xmlns="http://www.w3.org/2000/svg"
  >
    <use href="@/public/logo.svg#logo" />
  </svg>
</template>

<script setup lang="ts">
const props = defineProps<{
  width?: string
  height?: string
  class?: string
}>()
</script>
