<template>
  <div class="flex items-center justify-between">
    <h1 class="text-xl font-bold">{{ title }}</h1>
    <a @click="$router.go(-1)">
      <XMarkIcon class="w-8 h-8 cursor-pointer" />
    </a>
  </div>
</template>

<script setup lang="ts">
import { XMarkIcon } from '@heroicons/vue/24/outline';

defineProps({
  title: {
    type: String,
    required: true,
  },
});
</script>
