import { createPinia } from 'pinia';
import { createApp } from 'vue';
import router from './router'
import './index.scss';
import App from './App.vue';

// Import Bootstrap CSS
import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/dist/js/bootstrap.bundle.min.js';
import 'bootstrap-icons/font/bootstrap-icons.css';

const app = createApp(App)

app.use(router)

app.use(createPinia())
app.mount('#app');
