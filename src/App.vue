<script setup lang="ts">
import { onMounted } from "vue";
import AppSidebar from "./components/ui/AppSidebar.vue";
import { RouterView, RouterLink } from "vue-router";

onMounted(() => {
  document.addEventListener("contextmenu", (e) => e.preventDefault());
});
</script>

<template>
  <AppSidebar />

  <main class="py-10 lg:pl-72">
    <div class="px-4 sm:px-6 lg:px-8">
      <RouterLink :to="{ name: 'Home' }">Home</RouterLink>
      <RouterLink :to="{ name: 'Settings' }">Settings</RouterLink>

      <RouterView v-slot="{ Component }">
        <Transition>
          <template v-if="Component">
            <KeepAlive>
              <component :is="Component"></component>
            </KeepAlive>
          </template>
        </Transition>
      </RouterView>
    </div>
  </main>
</template>
