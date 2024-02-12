<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

const timer = ref("00:00:00");

type TimerEvent = {
  timer: string;
};

onMounted(async () => {
  invoke("init_process");

  await listen("timer", (event) => {
    if (timer.value != (event.payload as TimerEvent).timer) {
      timer.value = (event.payload as TimerEvent).timer;
    }
  });
});
</script>

<template>
  <div data-tauri-drag-region>
    <p style="font-size: 56px; color: #fff; font-weight: 500; -webkit-user-select: none">
      {{ timer }}
    </p>
  </div>
</template>
