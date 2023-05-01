<template>
  <div class="relative flex flex-col h-full">
    <TitleBar />

    <div class="relative z-0 flex max-h-[calc(100%)] grow">
      <Sidebar />

      <div class="flex flex-col w-full h-full overflow-hidden pt-[30px]">
        <ChatMessages />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Sidebar from '../components/chat/Sidebar.vue';
import ChatMessages from '../components/chat/ChatMessages.vue';
import TitleBar from '../components/TitleBar.vue';
</script>
