<script setup lang="ts">
import { RouterView } from "vue-router";
import { WebviewWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

async function handleClick() {
  try {
    const webview = new WebviewWindow("my-label", {
      url: "/about",
      title: "timer",
      visible: true,
      decorations: false,
      transparent: true,
      width: 400,
      height: 300,
    });
    webview.once("tauri://created", function () {
      webview.startDragging();
      // webview window successfully created
    });
  } catch (e) {
    console.log(e);
  }
}

onMounted(async () => {
  handleClick();
});
</script>

<template>
  <div class="container"></div>

  <RouterView />
</template>
