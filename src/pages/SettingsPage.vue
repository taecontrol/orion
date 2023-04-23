<template>
  <TitleBar />

  <div class="max-w-md mx-auto mt-10">
    <PageHeader title="Settings" />

    <TextInput
      class="mt-10"
      label="Open AI secret"
      name="open-ai-secret"
      id="open-ai-secret"
      placeholder="secret..."
      v-model="settings.open_ai_secret"
    />

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
import { CheckBadgeIcon } from '@heroicons/vue/24/outline';
import { invoke } from '@tauri-apps/api';
import { Settings } from '../types';
import TextInput from '../components/form/TextInput.vue';
import PageHeader from '../components/PageHeader.vue';

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
