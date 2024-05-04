<script setup lang="ts">
import { Application } from 'vue3-pixi'
import Demo from "./components/Demo.vue";

</script>

<template>
  <Application :width="800" :height="600" :background-alpha="0">
    <Demo />
  </Application>
</template>
