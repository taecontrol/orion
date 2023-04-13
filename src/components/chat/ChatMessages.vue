<template>
  <div id="chat" class="overflow-y-auto divide-y">
    <div
      v-for="message of messages"
      :key="message.id"
      class="p-4"
    >
      <div v-html="convertMarkdownToHtml(message.content)" class="prose"></div>
    </div>
  </div>

  <MessageTextarea @submit="onSubmit" />
</template>

<script setup lang="ts">
import {ref, watch, defineProps} from "vue";
import {Message} from "../../types";
import {marked} from "marked";
import {invoke} from "@tauri-apps/api/tauri";
import MessageTextarea from "./MessageTextarea.vue";
import {v4 as uuidv4} from "uuid";

const messages = ref<Message[]>([]);

const props = defineProps({
  selectedSessionId: {
    type: String,
    required: false,
  },
});

watch(() => props.selectedSessionId, async (sessionId) => {
  if (sessionId) {
    messages.value = await listMessages();
  }
});

function listMessages(): Promise<Message[]> {
  return invoke('list_messages', { parentSessionId: props.selectedSessionId })
}
function convertMarkdownToHtml(markdown: string) {
  return marked(markdown);
}

async function onSubmit(message: string) {
  const dummyMessage: Message = {
    id: uuidv4(),
    role: 'user',
    content: message,
    created_at: new Date().toISOString(),
    session_id: props.selectedSessionId!,
  };

  messages.value = [...messages.value, dummyMessage];

  await invoke('ask', { parentSessionId: props.selectedSessionId, message: dummyMessage.content })

  messages.value = await listMessages();

  setTimeout(() => {
    const chat = document.getElementById('.overflow-y-auto');
    chat?.scrollTo(0, chat.scrollHeight);
  }, 100);
}
</script>
