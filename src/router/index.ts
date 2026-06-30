import { createRouter, createWebHashHistory } from "vue-router";
import { MODULES } from "@/modules/registry";

const moduleRoutes = MODULES.map((module) => ({
  path: module.route,
  name: module.id,
  component: module.component,
  meta: { module },
}));

export const router = createRouter({
  // Hash history is reliable inside Tauri's custom protocol (history mode often white-screens).
  history: createWebHashHistory(),
  routes: [
    { path: "/", redirect: "/dashboard" },
    ...moduleRoutes,
  ],
});
