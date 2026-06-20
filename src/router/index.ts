import { createRouter, createWebHistory } from "vue-router";
import { MODULES } from "@/modules/registry";

const moduleRoutes = MODULES.map((module) => ({
  path: module.route,
  name: module.id,
  component: module.component,
  meta: { module },
}));

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", redirect: "/dashboard" },
    ...moduleRoutes,
  ],
});
