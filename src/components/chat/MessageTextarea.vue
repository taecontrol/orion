<template>
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

            <div class="py-2" aria-hidden="true">
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
</template>

<script setup lang="ts">
import {ref} from "vue";

const message = ref('');

const emit = defineEmits(['submit']);

function onSubmit() {
  emit('submit', message.value);
  message.value = '';
}
</script>
