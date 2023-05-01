<template>
  <router-view />
  <notification />
</template>
<script setup lang="ts">
import { onMounted } from 'vue';
import Notification from './components/Notification.vue';
import { useCurrentAssistantStore } from './stores/currentAssistant';
import { useCurrentSessionStore } from './stores/currentSession';

const currentAsssistantStore = useCurrentAssistantStore();
const currentSessionStore = useCurrentSessionStore();

onMounted(async () => {
  await currentAsssistantStore.init();
  await currentSessionStore.init();
});
</script>
