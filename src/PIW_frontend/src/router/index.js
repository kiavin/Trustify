import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Register from '@/views/auth/Register.vue';
import Dashboard from '@/views/Dashboard.vue';
import EscrowTransaction from '@/views/EscrowTransaction.vue';

const routes = [
    // Default Pages
    {
        path: '/',
        name: 'home',
        component: Home,
    },
    {
        path: '/About',
        name: 'about',
        component: () => import('@/views/AboutUs.vue')
    },
    {
        path: '/auth/Register',
        component: Register
    },
    {
        path: '/dashboard',
        component: Dashboard
    },
    {
        path: '/escrow',
        component: EscrowTransaction
    },
    {
        path: '/auth/Login',
        component: () => import('@/views/auth/Login.vue')
    }
];

const router = createRouter({
    linkActiveClass: 'active',
    linkExactActiveClass: 'exact-active',
    history: createWebHistory(import.meta.env.BASE_URL),
    base: import.meta.env.BASE_URL,
    routes
});

export default router;