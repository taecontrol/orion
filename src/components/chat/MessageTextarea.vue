<template>
  <div class="relative flex-1 px-4 pb-4 shadow">
    <Transition
      enter-from-class="translate-y-[100px]"
      enter-active-class="transition duration-100 ease-out"
      leave-active-class="transition ease-in duration-75 translate-y-[100px]"
    >
      <div v-show="loading" class="absolute inset-x-0 z-0 flex justify-center -top-20">
        <div class="flex items-center p-4 shadow bg-indigo-50 rounded-xl shadow-indigo-500">
          <SpinnerIcon class="w-5 h-5 text-indigo-500" />
          <span class="font-medium text-indigo-500">Loading...</span>
        </div>
      </div>
    </Transition>
    <div class="relative z-10 flex items-start space-x-4 bg-white">
      <div class="flex-1 min-w-0">
        <form @submit.prevent="onSubmit" class="relative">
          <div
            class="overflow-hidden rounded-lg shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-indigo-600"
          >
            <label for="message" class="sr-only">Ask something</label>
            <textarea
              rows="3"
              name="message"
              id="message"
              class="block w-full resize-none border-0 bg-transparent text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:py-1.5 sm:text-sm sm:leading-6"
              placeholder="Ask something..."
              v-model="message"
              :disabled="loading"
              @keydown="onKeyPress"
            />

            <div class="py-2" aria-hidden="true">
              <div class="py-px">
                <div class="h-9" />
              </div>
            </div>
          </div>

          <div class="absolute inset-x-0 bottom-0 flex justify-end py-2 pl-3 pr-2">
            <div class="flex-shrink-0">
              <button
                :disabled="loading"
                type="submit"
                class="inline-flex items-center px-3 py-2 text-sm font-semibold text-white bg-indigo-600 rounded-md shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:bg-indigo-400"
              >
                Submit
              </button>
            </div>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import SpinnerIcon from '../icons/SpinnerIcon.vue';

const message = ref('');

const emit = defineEmits(['submit']);
defineProps({
  loading: {
    type: Boolean,
    required: false,
  },
});

function onKeyPress(e: KeyboardEvent) {
  if (e.key === 'Enter' && e.metaKey) {
    e.preventDefault();
    onSubmit();
  }
}

function onSubmit() {
  if (message.value.trim() === '') return;

  emit('submit', message.value);
  message.value = '';
}
</script>
