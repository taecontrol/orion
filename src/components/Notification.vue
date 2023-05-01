<template>
  <div
    aria-live="assertive"
    class="fixed inset-0 flex items-end px-4 py-6 pointer-events-none sm:items-start sm:p-6"
  >
    <div class="flex flex-col items-center w-full space-y-4 sm:items-end">
      <transition
        enter-active-class="transition duration-300 ease-out transform"
        enter-from-class="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
        enter-to-class="translate-y-0 opacity-100 sm:translate-x-0"
        leave-active-class="transition duration-100 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-if="show"
          class="w-full max-w-sm overflow-hidden bg-white rounded-lg shadow-lg pointer-events-auto ring-1 ring-black ring-opacity-5"
        >
          <div class="p-4">
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <component
                  :is="iconComponent"
                  class="w-6 h-6"
                  :class="[typeOptions[notification.type].class]"
                />
              </div>
              <div class="ml-3 w-0 flex-1 pt-0.5">
                <p class="text-sm font-medium text-gray-900">{{ notification.title }}</p>
                <p class="mt-1 text-sm text-gray-500">{{ notification.body }}</p>
              </div>
              <div class="flex flex-shrink-0 ml-4">
                <button
                  type="button"
                  @click="show = false"
                  class="inline-flex text-gray-400 bg-white rounded-md hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                >
                  <span class="sr-only">Close</span>
                  <XMarkIcon class="w-5 h-5" aria-hidden="true" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineAsyncComponent, onMounted, ref, shallowRef } from 'vue';
import { XMarkIcon } from '@heroicons/vue/20/solid';
import { listen } from '@tauri-apps/api/event';

interface TypeOption {
  component: string;
  class: string;
}
interface Notification {
  title: string;
  body: string;
  type: string;
}

const notification = ref<Notification>({
  title: '',
  body: '',
  type: 'info',
});
const typeOptions = ref<{ [key: string]: TypeOption }>({
  info: {
    component: './icons/notifications/InfoIcon.vue',
    class: 'text-blue-500',
  },
  success: {
    component: './icons/notifications/SuccessIcon.vue',
    class: 'text-green-500',
  },
  warning: {
    component: './icons/notifications/WarningIcon.vue',
    class: 'text-amber-500',
  },
  error: {
    component: './icons/notifications/ErrorIcon.vue',
    class: 'text-red-500',
  },
});
const iconComponent = shallowRef<Promise<any> | null>(null);
const show = ref(false);

onMounted(async () => {
  listen<Notification>('notification', async (data) => {
    notification.value = data.payload;
    iconComponent.value = defineAsyncComponent(
      () => import(typeOptions.value[notification.value.type].component)
    );
    show.value = true;

    setTimeout(() => {
      show.value = false;
    }, 3000);
  });
});
</script>
