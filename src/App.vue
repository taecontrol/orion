<template>
  <router-view />
  <notification />
</template>
<script setup lang="ts">
import { onMounted } from 'vue';
import Notification from './components/Notification.vue';
import { useCurrentAssistantStore } from './stores/currentAssistant';

const currentAsssistantStore = useCurrentAssistantStore();

onMounted(() => {
  currentAsssistantStore.init();
});
</script>
