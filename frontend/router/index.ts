import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      name: "Home",
      path: "/",
      component: HomeView,
    },
    {
      name: "Settings",
      path: "/settings",
      component: () => import("../views/SettingsView.vue"),
    },
  ],
});

export default router;
