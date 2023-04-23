import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import ChatPage from './pages/ChatPage.vue';
import SettingsPage from './pages/SettingsPage.vue';
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  { path: '/', component: ChatPage },
  { path: '/settings', component: SettingsPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

createApp(App).use(router).mount('#app');
