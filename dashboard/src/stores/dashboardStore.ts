import { create } from 'zustand';

interface User { name: string; role: string; }

interface DashboardState {
  sidebarOpen: boolean;
  user: User | null;
  theme: 'light' | 'dark';
  toggleSidebar: () => void;
  setTheme: (t: 'light' | 'dark') => void;
}

export const useDashboardStore = create<DashboardState>((set) => ({
  sidebarOpen: true,
  user: { name: 'Bank Operator', role: 'Administrator' },
  theme: 'light',
  toggleSidebar: () => set(s => ({ sidebarOpen: !s.sidebarOpen })),
  setTheme: (theme) => set({ theme }),
}));
