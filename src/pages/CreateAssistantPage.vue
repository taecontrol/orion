<template>
  <TitleBar />

  <form @submit.prevent="save" class="max-w-lg mx-auto mt-10">
    <PageHeader title="Create new assistant" />

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
import { ref } from 'vue';
import PageHeader from '../components/PageHeader.vue';
import TitleBar from '../components/TitleBar.vue';
import TextInput from '../components/form/TextInput.vue';
import TextareaInput from '../components/form/TextareaInput.vue';
import { invoke } from '@tauri-apps/api';
import { useRouter } from 'vue-router';

const router = useRouter();

const formData = ref({
  name: '',
  description: '',
});

async function save() {
  await invoke('create_assistant', { newAssistant: formData.value });
  router.go(-1);
}
</script>
