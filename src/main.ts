import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import ChatPage from './pages/ChatPage.vue';
import SettingsPage from './pages/SettingsPage.vue';
import CreateAssistantPage from './pages/CreateAssistantPage.vue';
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  { path: '/', component: CreateAssistantPage },
  { path: '/settings', component: SettingsPage },
  { path: '/assistants/create', component: CreateAssistantPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

createApp(App).use(router).mount('#app');
