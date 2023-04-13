<template>
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
      :class="{ 'bg-indigo-100': selectedSessionId === session.id }"
    >
      <p>{{session.name}}</p>
      <span class="text-gray-400">{{dayjs(session.created_at).fromNow()}}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import * as dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import {PlusCircleIcon} from "@heroicons/vue/20/solid/index.js";
import {onMounted, ref} from "vue";
import {Session} from "../../types";
import {invoke} from "@tauri-apps/api/tauri";

dayjs.extend(relativeTime)

const sessions = ref<Session[]>([]);

const emit = defineEmits(['select-session']);

defineProps({
  selectedSessionId: {
    type: String,
    required: false,
  },
});

onMounted(async () => {
  sessions.value = await listSessions();
});

async function listSessions(): Promise<Session[]> {
  return invoke('list_sessions');
}

async function newSession() {
  await invoke('new_session');
  await listSessions();
}

function selectSession(sessionId: string) {
  emit('select-session', sessionId);
}
</script>