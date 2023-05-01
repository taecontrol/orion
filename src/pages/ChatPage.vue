<template>
  <div class="relative flex flex-col h-full">
    <TitleBar />

    <div class="relative z-0 flex max-h-[calc(100%)] grow">
      <Sidebar
        :selected-assistant-id="selectedAssistant"
        :selected-session-id="selectedSession"
        @select-session="selectSession"
      />

      <div class="flex flex-col w-full h-full overflow-hidden pt-[30px]">
        <ChatMessages :selected-session-id="selectedSession" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Sidebar from '../components/chat/Sidebar.vue';
import ChatMessages from '../components/chat/ChatMessages.vue';
import TitleBar from '../components/TitleBar.vue';

const selectedSession = ref<string | undefined>(undefined);
const selectedAssistant = ref<string | undefined>(undefined);

onMounted(async () => {
  selectedSession.value = getSelectedSession();
  selectedAssistant.value = getSelectedAssistant();
});

function selectSession(sessionId: string) {
  selectedSession.value = sessionId;
  localStorage.setItem('selectedSession', selectedSession.value);
}

function getSelectedSession() {
  return localStorage.getItem('selectedSession') ?? undefined;
}

function getSelectedAssistant() {
  return localStorage.getItem('selectedAssistant') ?? 'chatgpt';
}
</script>
