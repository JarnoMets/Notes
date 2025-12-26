import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import LoginView from '../views/LoginView.vue'
import RegisterView from '../views/RegisterView.vue'
import GoogleCallbackView from '../views/GoogleCallbackView.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('../layouts/DefaultLayout.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/notes',
    redirect: '/'
  },
  {
    path: '/graphs',
    redirect: '/'
  },
  {
    path: '/boards',
    redirect: '/'
  },
  {
    path: '/calendar',
    name: 'calendar',
    component: () => import('../views/CalendarView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: { guest: true }
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView,
    meta: { guest: true }
  },
  {
    path: '/auth/google/callback',
    name: 'google-callback',
    component: GoogleCallbackView,
    meta: { guest: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// Navigation guard
router.beforeEach((to, _from, next) => {
  const authStore = useAuthStore()
  
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login')
  } else if (to.meta.guest && authStore.isAuthenticated) {
    next('/')
  } else {
    next()
  }
})

export default router
