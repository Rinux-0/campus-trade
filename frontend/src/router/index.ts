import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  { path: '/', name: 'home', component: () => import('../views/Home.vue') },
  { path: '/users', name: 'users', component: () => import('../views/Users.vue') },
  { path: '/items', name: 'items', component: () => import('../views/Items.vue') },
  { path: '/orders', name: 'orders', component: () => import('../views/Orders.vue') },
  { path: '/queries', name: 'queries', component: () => import('../views/Queries.vue') },
]

export default createRouter({
  history: createWebHashHistory(),
  routes,
})
