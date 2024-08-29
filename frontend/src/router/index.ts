/**
 * router/index.ts
 *
 * Automatic routes for `./src/pages/*.vue`
 */

// Composables
import { createRouter, createWebHistory } from 'vue-router/auto'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/home',
    },
    {
      path: '/home',
      name: 'home',
      component: () => import('../pages/index.vue'),
    },
    {
      path: '/currency',
      name: 'currency',
      component: () => import('../pages/currency.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../pages/settings.vue'),
    },
  ],
})

export default router
