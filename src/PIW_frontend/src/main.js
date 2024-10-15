import { createPinia } from 'pinia';
import { createApp } from 'vue';
import './index.scss';
import App from './App.vue';
import router from './router';
import 'bootstrap/scss/bootstrap.scss';
import 'bootstrap';

import Aura from '@primevue/themes/aura';
import PrimeVue from 'primevue/config';
import './assets/styles.scss';
import './assets/tailwind.css';
import './index.css'
import '@/assets/flowbite.js';
// Create the app instance
const app = createApp(App);

// Create the Pinia store
const pinia = createPinia();
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
darkModeSelector: '.dark-mode',
        },
        }
    }
);


// Use the router and Pinia on the app instance
app.use(router).use(pinia);


// Mount the app
app.mount('#app');
