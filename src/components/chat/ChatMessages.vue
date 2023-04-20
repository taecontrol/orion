<template>
  <div class="absolute right-0 p-4">
    <button class="rounded-full p-2 shadow group hover:bg-red-500 transition easy-in-out duration-150" @click="confirmDeleteSession">
      <TrashIcon class="w-6 h-6 group-hover:text-white" />
    </button>
  </div>
  <ul ref="chatContainer" class="overflow-y-auto divide-y h-full px-4">
    <li
      v-for="message of messages"
      :key="message.id"
      class="p-4"
    >
      <div v-html="convertMarkdownToHtml(message.content)" class="prose max-w-[100ch]"></div>
    </li>
  </ul>

  <MessageTextarea @submit="onSubmit" :loading="loading" />

  <DeleteSessionDialog
    :open="openConfirmation"
    @on-close="openConfirmation = false"
    @on-cancel="openConfirmation = false"
    @on-delete="deleteSession"
  />
</template>

<script setup lang="ts">
import {ref, watch, defineProps} from "vue";
import {Message} from "../../types";
import {marked} from "marked";
import {invoke} from "@tauri-apps/api/tauri";
import MessageTextarea from "./MessageTextarea.vue";
import {v4 as uuidv4} from "uuid";
import TrashIcon from '../icons/TrashIcon.vue';
import DeleteSessionDialog from './DeleteSessionDialog.vue';

const openConfirmation = ref(false);
const loading = ref(false);
const messages = ref<Message[]>([]);
const chatContainer = ref<HTMLElement|null>(null);

const props = defineProps({
  selectedSessionId: {
    type: String,
    required: false,
  },
});

watch(() => props.selectedSessionId, async (sessionId) => {
  if (sessionId) {
    messages.value = await listMessages();
    scrollToBottom();
  }
});

function listMessages(): Promise<Message[]> {
  return invoke('list_messages', { parentSessionId: props.selectedSessionId })
}
function convertMarkdownToHtml(markdown: string) {
  return marked(markdown);
}

function scrollToBottom() {
    setTimeout(() => {
      if (chatContainer.value) {
        chatContainer.value.scrollTo({
          top: chatContainer.value.scrollHeight,
          left: 0,
          behavior: "smooth",
        });
      }
    }, 10)
}

function confirmDeleteSession() {
  openConfirmation.value = true;
}

async function deleteSession() {
  openConfirmation.value = false;
  await invoke('delete_session', { parentSessionId: props.selectedSessionId });
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

  loading.value = true;
  await invoke('ask', { parentSessionId: props.selectedSessionId, message: dummyMessage.content })

  messages.value = await listMessages();

  scrollToBottom();
  loading.value = false;
}
</script>
