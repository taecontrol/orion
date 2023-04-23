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

    <RadioGroup v-model="selectedAssistant">
      <div class="grid grid-cols-3 gap-2 mt-10">
        <RadioGroupOption
          v-for="assistant in assistants"
          as="template"
          :key="assistant.id"
          :value="assistant.id"
          v-slot="{ checked, active }"
        >
          <div
            :class="[
              checked ? 'border-indigo-600 bg-indigo-50 ring-1' : 'border-transparent',
              active ? 'border-indigo-600 ring-2 ring-indigo-600' : '',
              'relative flex flex-col cursor-pointer border focus:outline-none items-center justify-center p-4 bg-gray-100 rounded-lg',
            ]"
          >
            <CheckCircleIcon
              class="absolute top-1 left-1"
              :class="[!checked ? 'invisible' : '', 'h-5 w-5 text-indigo-600']"
              aria-hidden="true"
            />

            <router-link
              :to="`/assistants/${assistant.id}`"
              class="absolute p-2 top-1 right-1 hover:underline"
            >
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
        </RadioGroupOption>
      </div>
    </RadioGroup>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import PageHeader from '../components/PageHeader.vue';
import TitleBar from '../components/TitleBar.vue';
import { invoke } from '@tauri-apps/api';
import { Assistant } from '../types';
import robotImage from '../assets/robot-1.png';
import { PencilSquareIcon, PlusCircleIcon } from '@heroicons/vue/24/outline';
import { RadioGroup, RadioGroupOption } from '@headlessui/vue';
import { CheckCircleIcon } from '@heroicons/vue/20/solid';

const assistants = ref<Assistant[]>([]);
const selectedAssistant = ref<string | undefined>(undefined);

onMounted(async () => {
  assistants.value = await invoke('list_assistants');
  selectedAssistant.value = getSelectedAssistant();
});

watch(selectedAssistant, (value) => {
  if (value !== undefined) {
    selectAssistant();
  }
});

function getSelectedAssistant() {
  return localStorage.getItem('selectedAssistant') ?? undefined;
}

function selectAssistant() {
  localStorage.setItem('selectedAssistant', selectedAssistant.value!);
  localStorage.removeItem('selectedSession');
}
</script>
