import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import ChatPage from './pages/ChatPage.vue';
import SettingsPage from './pages/SettingsPage.vue';
import CreateAssistantPage from './pages/CreateAssistantPage.vue';
import EditAssistantPage from './pages/EditAssistantPage.vue';
import AssistantsPage from './pages/AssistantsPage.vue';
import { createRouter, createWebHistory } from 'vue-router';
import { createPinia } from 'pinia';

const routes = [
  { path: '/', component: ChatPage },
  { path: '/settings', component: SettingsPage },
  { path: '/assistants', component: AssistantsPage },
  { path: '/assistants/create', component: CreateAssistantPage },
  { path: '/assistants/:id', component: EditAssistantPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const pinia = createPinia();

createApp(App).use(router).use(pinia).mount('#app');
