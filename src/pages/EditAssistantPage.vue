<template>
  <TitleBar />

  <form @submit.prevent="save" class="max-w-md mx-auto mt-10">
    <PageHeader title="Edit assistant" />

    <TextInput
      class="mt-10"
      label="Name"
      name="name"
      id="name"
      placeholder="Jhon"
      required
      v-model="formData.name"
    />

    <TextareaInput
      class="mt-4"
      label="Description"
      name="description"
      id="description"
      required
      v-model="formData.description"
    />

    <div class="flex items-center justify-end mt-4">
      <button
        type="submit"
        class="relative min-w-[86px] min-h-[36px] inline-flex justify-center items-center gap-x-2 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
      >
        Save
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import PageHeader from '../components/PageHeader.vue';
import TitleBar from '../components/TitleBar.vue';
import TextInput from '../components/form/TextInput.vue';
import TextareaInput from '../components/form/TextareaInput.vue';
import { invoke } from '@tauri-apps/api';
import { useRouter, useRoute } from 'vue-router';
import { Assistant } from '../types';

const router = useRouter();
const route = useRoute();

const formData = ref({
  name: '',
  description: '',
});
const assistant = ref<Assistant | null>(null);

onMounted(async () => {
  assistant.value = await invoke('get_assistant', {
    id: route.params.id,
  });

  if (assistant.value === null) {
    router.go(-1);
    return;
  }

  formData.value = {
    name: assistant.value.name,
    description: assistant.value.description,
  };
});

async function save() {
  await invoke('update_assistant', { id: assistant.value?.id, assistant: formData.value });
  router.go(-1);
}
</script>
