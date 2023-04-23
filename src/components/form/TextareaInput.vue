<template>
  <div>
    <label :for="id" class="block text-sm font-medium leading-6 text-gray-900">
      {{ label }}
    </label>
    <div class="mt-2">
      <textarea
        :required="required"
        :rows="rows"
        :name="name"
        :id="id"
        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  label: {
    type: String,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  id: {
    type: String,
    required: true,
  },
  rows: {
    type: Number,
    default: 4,
  },
  required: {
    type: Boolean,
    default: false,
  },
  modelValue: {
    type: String,
    required: true,
  },
});

defineEmits(['update:modelValue']);
</script>
