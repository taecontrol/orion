import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia';
import { Session } from '../types';

interface State {
  currentSession?: Session;
}

export const useCurrentSessionStore = defineStore('currentSession', {
  state: (): State => ({
    currentSession: undefined,
  }),
  actions: {
    async init() {
      const id = localStorage.getItem('selectedSession');

      if (!id) {
        return;
      }

      this.currentSession = await invoke('get_session', { id });
    },

    async selectSession(id: string) {
      this.currentSession = await invoke('get_session', { id });
      localStorage.setItem('selectedSession', id);
    },
  },
});
