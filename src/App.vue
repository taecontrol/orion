<template>
  <div class="flex h-full">

    <div class="flex flex-col w-[300px] h-full bg-gray-100 overflow-hidden">
      <Sidebar
        :selected-session-id="selectedSession"
        @select-session="selectSession"
      />
    </div>

    <div class="w-full h-full flex flex-col overflow-hidden">
      <ChatMessages :selected-session-id="selectedSession" />
    </div>
  </div>
</template>

<script setup lang="ts">

import {onMounted, ref} from 'vue';
import Sidebar from "./components/chat/Sidebar.vue";
import ChatMessages from "./components/chat/ChatMessages.vue";

const selectedSession = ref<string|undefined>(undefined);

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
