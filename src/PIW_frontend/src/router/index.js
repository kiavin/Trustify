import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
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
path: '/Contact',
name: 'contact',
component: () => import('@/views/Contact.vue')
    },
    {
        path: '/Features',
        name: 'features',
        component: () => import('@/views/Features.vue')
    },
    {
        path: '/dashboard/new-transaction',
        name: 'new-transaction',
        component: () => import('@/views/Transaction.vue')
    },
    {
        path: '/dashboard/Escrow-Transactions',
        name: 'escrow-transactions',
        component: () => import('@/views/EscrowTransaction.vue')
    },
    {
        path: '/dashboard/Help',
        name: 'help',
        component: () => import('@/views/Help.vue')
    },
    {
        path: '/dashboard/wallet',
        name: 'wallet',
        component: () => import('@/views/Wallet.vue')
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

]

const router = createRouter({
    linkActiveClass: 'active',
    linkExactActiveClass: 'exact-active',
    history: createWebHistory(import.meta.env.BASE_URL),
    base: import.meta.env.BASE_URL,
    routes
})

// router.beforeEach(async (to, from, next) => {



// })

export default router
