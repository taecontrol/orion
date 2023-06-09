<template>
  <div class="relative flex flex-col w-[300px] h-full bg-gray-100 overflow-hidden pt-[30px]">
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
        @click="currentSessionStore.selectSession(session.id)"
        class="p-2 hover:cursor-pointer"
        :class="{ 'bg-indigo-100': currentSessionStore.currentSession?.id === session.id }"
      >
        <p class="truncate">{{ session.name }}</p>
        <span class="text-gray-400">{{ dayjs.utc(session.created_at).local().fromNow() }}</span>
      </div>
    </div>

    <div class="absolute inset-x-0 bottom-0 flex items-center justify-between">
      <router-link to="/assistants" class="p-2 rounded-full">
        <UserCircleIcon class="w-6 h-6" />
      </router-link>
      <router-link to="/settings" class="p-2 rounded-full">
        <Cog8ToothIcon class="w-6 h-6" />
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import utc from 'dayjs/plugin/utc';
import { PlusCircleIcon } from '@heroicons/vue/20/solid/index.js';
import { onMounted, ref, watch } from 'vue';
import { Session } from '../../types';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { Cog8ToothIcon, UserCircleIcon } from '@heroicons/vue/24/outline';
import { useCurrentAssistantStore } from '../../stores/currentAssistant';
import { useCurrentSessionStore } from '../../stores/currentSession';

dayjs.extend(relativeTime);
dayjs.extend(utc);

const currentAssistantStore = useCurrentAssistantStore();
const currentSessionStore = useCurrentSessionStore();

const sessions = ref<Session[]>([]);

onMounted(async () => {
  if (currentSessionStore.currentSession?.id) {
    sessions.value = await listSessions();
  }

  if (!currentSessionStore.currentSession?.id) {
    selectFirstSession();
  }

  await listen('session_updated', async () => {
    sessions.value = await listSessions();
  });

  await listen('session_deleted', async () => {
    sessions.value = await listSessions();
    selectFirstSession();
  });
});

currentAssistantStore.$subscribe(async (_, state) => {
  if (!state.currentAssistant?.id) {
    return;
  }

  sessions.value = await listSessions();
  selectFirstSession();
});

async function listSessions(): Promise<Session[]> {
  return invoke('list_sessions', { assistantId: currentAssistantStore.currentAssistant?.id });
}

async function newSession() {
  if (!currentAssistantStore.currentAssistant?.id) {
    return;
  }

  await invoke('new_session', { assistantId: currentAssistantStore.currentAssistant?.id });
  sessions.value = await listSessions();
  selectFirstSession();
}

function selectFirstSession() {
  if (sessions.value.length > 0) {
    currentSessionStore.selectSession(sessions.value[0].id);
  }
}
</script>
