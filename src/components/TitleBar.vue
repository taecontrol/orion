<template>
  <div data-tauri-drag-region class="flex items-center py-1 px-2 h-[30px] space-x-2">
    <button @click="appWindow.close()" class="p-[1px] bg-red-400 rounded-full" id="titlebar-close">
      <XMarkIcon class="w-3 h-3 transition duration-75 ease-in-out opacity-0 hover:opacity-100" />
    </button>
    <button
      @click="appWindow.minimize()"
      class="p-[1px] bg-yellow-400 rounded-full"
      id="titlebar-close"
    >
      <MinusSmallIcon
        class="w-3 h-3 transition duration-75 ease-in-out opacity-0 hover:opacity-100"
      />
    </button>
    <button
      @click="appWindow.maximize()"
      class="p-[1px] bg-green-500 rounded-full"
      id="titlebar-close"
    >
      <PlusSmallIcon
        class="w-3 h-3 transition duration-75 ease-in-out opacity-0 hover:opacity-100"
      />
    </button>
  </div>
</template>

<script setup lang="ts">
import { MinusSmallIcon, PlusSmallIcon, XMarkIcon } from '@heroicons/vue/20/solid';
import { appWindow } from '@tauri-apps/api/window';
</script>
