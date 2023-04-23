export type Session = {
  id: string;
  name: string;
  created_at: string;
};

export type Message = {
  id: string;
  session_id: string;
  content: string;
  role: 'user' | 'assistant' | 'system';
  finish_reason?: string;
  prompt_tokens?: number;
  completion_tokens?: number;
  created_at?: string;
};

export type Assistant = {
  id: string;
  name: string;
  description: string;
  created_at: string;
};

export type Settings = {
  open_ai_secret: string;
};
