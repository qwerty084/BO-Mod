<script setup lang="ts">
import { ref } from "vue";

const counter = ref(0);
</script>

<template>
  <h1>Home</h1>
  <button @click="() => counter++">Increment</button>
  <p>{{ counter }}</p>
</template>
