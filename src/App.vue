<script setup lang="ts">
import { Application } from "vue3-pixi";
import Demo from "./components/Demo.vue";
import { useWindowSize } from "@vueuse/core";

const { width, height } = useWindowSize();
</script>

<template>
  <Application :width="width" :height="height - 8" :background-alpha="0">
    <Demo />
  </Application>
</template>
