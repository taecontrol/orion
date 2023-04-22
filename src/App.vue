<template>
  <div class="relative flex flex-col h-full">
    <TitleBar />

    <div class="flex max-h-[calc(100%-30px)] grow">
      <div class="flex flex-col w-[300px] h-full bg-gray-100 overflow-hidden">
        <Sidebar :selected-session-id="selectedSession" @select-session="selectSession" />
      </div>

      <div class="flex flex-col w-full h-full overflow-hidden">
        <ChatMessages :selected-session-id="selectedSession" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Sidebar from './components/chat/Sidebar.vue';
import ChatMessages from './components/chat/ChatMessages.vue';
import TitleBar from './components/TitleBar.vue';

const selectedSession = ref<string | undefined>(undefined);

onMounted(async () => {
  selectedSession.value = getSelectedSession();
});

function selectSession(sessionId: string) {
  selectedSession.value = sessionId;
  localStorage.setItem('selectedSession', selectedSession.value);
}

function getSelectedSession() {
  return localStorage.getItem('selectedSession') ?? undefined;
}
</script>
