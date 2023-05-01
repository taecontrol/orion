import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia';
import { Assistant } from '../types';

interface State {
  currentAssistant?: Assistant;
}

export const useCurrentAssistantStore = defineStore('currentAssistant', {
  state: (): State => ({
    currentAssistant: undefined,
  }),
  actions: {
    async init() {
      const id = localStorage.getItem('selectedAssistant') ?? 'chatgpt';
      this.currentAssistant = await invoke('get_assistant', { id });
    },

    async selectAssistant(id: string) {
      this.currentAssistant = await invoke('get_assistant', { id });
      localStorage.setItem('selectedAssistant', id);
      localStorage.removeItem('selectedSession');
    },
  },
});
