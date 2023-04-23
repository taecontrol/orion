<template>
  <TitleBar />

  <div class="max-w-screen-sm mx-auto mt-10">
    <div class="flex items-center justify-between">
      <h1 class="text-xl font-bold">Settings</h1>
      <a @click="$router.go(-1)">
        <XMarkIcon class="w-8 h-8 cursor-pointer" />
      </a>
    </div>

    <div class="mt-10">
      <label for="open-ai-secret" class="block text-sm font-medium leading-6 text-gray-900"
        >Open AI secret</label
      >
      <div class="mt-2">
        <input
          name="open-ai-secret"
          id="open-ai-secret"
          class="block px-2 w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
          placeholder="secret..."
          aria-describedby="open-ai-secret-description"
          v-model="settings.open_ai_secret"
        />
      </div>
    </div>

    <div class="flex items-center justify-end mt-4">
      <button
        type="button"
        class="relative min-w-[86px] min-h-[36px] inline-flex justify-center items-center gap-x-2 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        @click="save"
      >
        <Transition
          enter-from-class="-translate-x-[86px]"
          enter-to-class="translate-x-0"
          enter-active-class="transition duration-300 ease-out translate-x-0"
          leave-active-class="transition duration-300 ease-in -translate-x-[86px]"
        >
          <span v-show="!saving" class="absolute">Save</span>
        </Transition>
        <Transition
          enter-from-class="translate-x-[86px]"
          leave-to-class="translate-x-[86px]"
          enter-active-class="transition duration-300 ease-in translate-x-0"
          leave-active-class="transition duration-300 ease-out translate-x-0"
        >
          <CheckBadgeIcon v-show="saving" class="absolute -mr-0.5 h-5 w-5" />
        </Transition>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import TitleBar from '../components/TitleBar.vue';
import { CheckBadgeIcon, XMarkIcon } from '@heroicons/vue/24/outline';
import { invoke } from '@tauri-apps/api';
import { Settings } from '../types';

const settings = ref<Settings>({
  open_ai_secret: '',
});
const saving = ref(false);

onMounted(async () => {
  settings.value = await invoke('get_settings');
});

async function save() {
  await invoke('set_settings', { settings: settings.value });
  setToSaving();
}

function setToSaving() {
  saving.value = true;

  setTimeout(() => {
    saving.value = false;
  }, 1000);
}
</script>
