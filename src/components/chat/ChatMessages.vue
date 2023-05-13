<template>
  <div class="absolute right-0 p-4">
    <button
      class="p-2 transition duration-150 bg-white rounded-full shadow group hover:bg-red-500 easy-in-out"
      @click="confirmDeleteSession"
    >
      <TrashIcon class="w-6 h-6 group-hover:text-white" />
    </button>
  </div>
  <ul ref="chatContainer" class="h-full px-4 overflow-y-auto divide-y">
    <li v-for="message of messages" :key="message.id" class="p-4">
      <div class="flex space-x-2">
        <img
          v-if="message.role === 'assistant'"
          class="w-8 h-8 rounded-full"
          :src="robotImage"
          alt="AI avatar"
        />
        <img
          v-if="message.role === 'user'"
          class="w-8 h-8 rounded-full"
          :src="youImage"
          alt="You avatar"
        />
        <div>
          <p v-if="message.role === 'assistant'" class="mt-1 font-medium text-gray-400">
            {{ currentAsssistantStore.currentAssistant?.name ?? 'chatGPT' }}
          </p>
          <p v-if="message.role === 'user'" class="mt-1 font-medium text-gray-400">You</p>
          <div
            v-html="convertMarkdownToHtml(message.content)"
            class="mt-2 prose max-w-[100ch]"
          ></div>
        </div>
      </div>
    </li>
  </ul>

  <MessageTextarea @submit="onSubmit" :loading="loading" :disabled="inputDisabled" />

  <DeleteSessionDialog
    :open="openConfirmation"
    @on-close="openConfirmation = false"
    @on-cancel="openConfirmation = false"
    @on-delete="deleteSession"
  />
</template>

<script setup lang="ts">
import robotImage from '../../assets/robot-1.png';
import youImage from '../../assets/you.png';
import { ref, watch, defineProps, onMounted } from 'vue';
import { Message } from '../../types';
import { marked } from 'marked';
import { invoke } from '@tauri-apps/api/tauri';
import MessageTextarea from './MessageTextarea.vue';
import { v4 as uuidv4 } from 'uuid';
import DeleteSessionDialog from './DeleteSessionDialog.vue';
import { TrashIcon } from '@heroicons/vue/24/outline';
import { useCurrentAssistantStore } from '../../stores/currentAssistant';
import { useCurrentSessionStore } from '../../stores/currentSession';

const currentAsssistantStore = useCurrentAssistantStore();
const currentSessionStore = useCurrentSessionStore();

const openConfirmation = ref(false);
const loading = ref(false);
const inputDisabled = ref(false);
const messages = ref<Message[]>([]);
const chatContainer = ref<HTMLElement | null>(null);

onMounted(async () => {
  if (currentSessionStore.currentSession?.id) {
    messages.value = await listMessages();
    scrollToBottom();
  } else {
    inputDisabled.value = true;
  }
});

currentSessionStore.$subscribe(async (_, state) => {
  if (!state.currentSession?.id) {
    messages.value = [];
    inputDisabled.value = true;
    return;
  }

  inputDisabled.value = false;
  messages.value = await listMessages();
  scrollToBottom();
});

function listMessages(): Promise<Message[]> {
  return invoke('list_messages', { sessionId: currentSessionStore.currentSession?.id });
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
        behavior: 'smooth',
      });
    }
  }, 10);
}

function confirmDeleteSession() {
  openConfirmation.value = true;
}

async function deleteSession() {
  openConfirmation.value = false;
  await invoke('delete_session', { sessionId: currentSessionStore.currentSession?.id });
  messages.value = [];
}

async function onSubmit(message: string) {
  const dummyMessage: Message = {
    id: uuidv4(),
    role: 'user',
    content: message,
    created_at: new Date().toISOString(),
    session_id: currentSessionStore.currentSession?.id!,
  };

  messages.value = [...messages.value, dummyMessage];
  scrollToBottom();

  loading.value = true;
  await invoke('ask', {
    sessionId: currentSessionStore.currentSession?.id,
    message: dummyMessage.content,
  });

  messages.value = await listMessages();

  scrollToBottom();
  loading.value = false;
}
</script>
