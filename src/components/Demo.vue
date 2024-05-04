<script setup lang="ts">
import type { GraphicsInst } from "vue3-pixi";
import { useWindowSize } from "@vueuse/core";

const { width, height } = useWindowSize();

function drawArc(e: GraphicsInst) {
  e.beginFill("#4f455c");
  e.arc(0, 0, 100, 0, Math.PI, false);
}
</script>

<template>
  <Graphics :x="width - 200" :y="height - 200" @render="drawArc" />
</template>
