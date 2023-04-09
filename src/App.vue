<template>
  <div class="flex h-full">

    <div class="flex flex-col w-[300px] h-full bg-gray-100 overflow-hidden">
      <div class="w-full p-4">
        <button
          type="button"
          class="w-full inline-flex justify-between items-center gap-x-2 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          @click="newSession"
        >
          New chat
          <PlusCircleIcon class="-mr-0.5 h-5 w-5" aria-hidden="true" />
        </button>
      </div>

      <div class="overflow-y-auto divide-y">
        <div
          v-for="session of sessions"
          :key="session.id"
          @click="selectSession(session.id)"
          class="p-2 hover:cursor-pointer"
          :class="{ 'bg-indigo-100': selectedSession === session.id }"
        >
          <p>{{session.name}}</p>
          <span>{{session.created_at}}</span>
        </div>
      </div>
    </div>

    <div class="w-full h-full flex flex-col overflow-hidden">
      <div class="overflow-y-auto divide-y">
        <div
          v-for="message of messages"
          :key="message.id"
          class="p-4"
        >
          <div v-html="convertMarkdownToHtml(message.content)" class="prose"></div>
        </div>
      </div>

      <div class="flex-1 p-4">
        <div class="flex items-start space-x-4">
          <div class="min-w-0 flex-1">
            <form @submit.prevent="onSubmit" class="relative">
              <div class="overflow-hidden rounded-lg shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-indigo-600">
                <label for="message" class="sr-only">Ask something</label>
                <textarea
                  rows="3"
                  name="message"
                  id="message"
                  class="block w-full resize-none border-0 bg-transparent text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:py-1.5 sm:text-sm sm:leading-6"
                  placeholder="Ask something..."
                  v-model="message"
                />

                <!-- Spacer element to match the height of the toolbar -->
                <div class="py-2" aria-hidden="true">
                  <!-- Matches height of button in toolbar (1px border + 36px content height) -->
                  <div class="py-px">
                    <div class="h-9" />
                  </div>
                </div>
              </div>

              <div class="absolute inset-x-0 bottom-0 flex justify-end py-2 pl-3 pr-2">
                <div class="flex-shrink-0">
                  <button
                    type="submit"
                    class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Submit
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">

import {onMounted, ref, watch} from 'vue';
import {invoke} from '@tauri-apps/api/tauri';
import {PlusCircleIcon} from '@heroicons/vue/20/solid';
import {Message, Session} from './components/types';
import {marked} from 'marked';

const message = ref('');
const sessions = ref<Session[]>([]);
const selectedSession = ref<string|null>(null);
const messages = ref<Message[]>([]);

onMounted(async () => {
  sessions.value = await listSessions();
  selectedSession.value = getSelectedSession();
});

watch(selectedSession, async (sessionId) => {
  if (sessionId) {
    messages.value = await listMessages();
  }
});

async function listSessions(): Promise<Session[]> {
  return invoke('list_sessions')
}

async function newSession() {
  await invoke('new_session')
}

function selectSession(sessionId: string) {
  selectedSession.value = sessionId;
  localStorage.setItem('selectedSession', selectedSession.value);
}

function getSelectedSession() {
  return localStorage.getItem('selectedSession');
}

function listMessages(): Promise<Message[]> {
  return invoke('list_messages', { session_id: selectedSession.value })
}

async function onSubmit() {
  const newMessages: Message[] = await invoke('ask', { parentSessionId: selectedSession.value, message:  message.value })

  messages.value = [...messages.value, ...newMessages];
  console.log(messages.value);
}

function convertMarkdownToHtml(markdown: string) {
  return marked(markdown);
}
</script>
