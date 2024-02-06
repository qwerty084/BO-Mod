<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { WebviewWindow } from "@tauri-apps/api/window";

function handleClick() {
  const label = crypto.randomUUID();
  console.log(label);
  const webview = new WebviewWindow(label, {
    url: "/about",
  });
  webview.show();
}
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/about">About</RouterLink>
    <button @click="handleClick">Open about window</button>
  </div>

  <RouterView />
</template>
