<template>
  <TitleBar />

  <div class="max-w-xl mx-auto mt-10">
    <PageHeader title="Assistants" />

    <div class="flex justify-end mt-4">
      <router-link
        to="/assistants/create"
        class="flex items-center p-2 space-x-1 text-indigo-500 hover:underline"
      >
        <PlusCircleIcon class="w-5 h-5" />
        <span>New assistant</span>
      </router-link>
    </div>

    <div class="grid grid-cols-3 gap-2 mt-10">
      <div
        v-for="assistant in assistants"
        :key="assistant.id"
        class="relative flex flex-col items-center justify-center p-4 bg-gray-100 rounded-lg"
      >
        <router-link to="/assistants/create" class="absolute p-2 top-1 right-1 hover:underline">
          <PencilSquareIcon class="w-4 h-4" />
        </router-link>
        <img class="w-20 h-20 rounded-full" :src="robotImage" alt="AI avatar" />

        <div class="mt-2 text-center">
          <h2 class="font-medium">{{ assistant.name }}</h2>
          <p class="mt-2 text-sm text-gray-500 line-clamp-3 min-h-[60px]">
            {{ assistant.description }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import PageHeader from '../components/PageHeader.vue';
import TitleBar from '../components/TitleBar.vue';
import { invoke } from '@tauri-apps/api';
import { Assistant } from '../types';
import robotImage from '../assets/robot-1.png';
import { PencilSquareIcon, PlusCircleIcon } from '@heroicons/vue/24/outline';

const assistants = ref<Assistant[]>([]);

onMounted(async () => {
  assistants.value = await invoke('list_assistants');
});
</script>
