import { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import {
  LayoutDashboard, Shield, Heart, Map, Bot, Monitor,
  Truck, FileCheck, Settings, ChevronLeft, ChevronRight, Menu, Bell,
} from 'lucide-react';
import { Button } from '@components/ui/Button';
import { Avatar, AvatarFallback } from '@radix-ui/react-avatar';
import { useDashboardStore } from '@stores/dashboardStore';
import { cn } from '@lib/utils';

const navigation = [
  { path: '/governance',  label: 'Governance',   icon: Shield,            badge: '3' },
  { path: '/wellness',    label: 'Wellness',      icon: Heart,             badge: null },
  { path: '/life-stage',  label: 'Life Stage',    icon: Map,               badge: null },
  { path: '/agents',      label: 'Agent Fleet',   icon: Bot,               badge: '12' },
  { path: '/atm',         label: 'ATM Fleet',     icon: Monitor,           badge: '340' },
  { path: '/migration',   label: 'Migration',     icon: Truck,             badge: null },
  { path: '/compliance',  label: 'Compliance',    icon: FileCheck,         badge: '1' },
  { path: '/settings',    label: 'Settings',      icon: Settings,          badge: null },
];

export function DashboardLayout({ children }: { children: React.ReactNode }) {
  const [collapsed, setCollapsed] = useState(false);
  const { pathname } = useLocation();
  const user = useDashboardStore(s => s.user);

  return (
    <div className="flex h-screen overflow-hidden">
      {/* Sidebar */}
      <motion.aside
        animate={{ width: collapsed ? 64 : 256 }}
        className="flex flex-col border-r bg-gray-950 text-gray-50 transition-all"
      >
        <div className="flex h-14 items-center justify-between px-4 border-b border-gray-800">
          {!collapsed && <span className="font-bold text-lg">Verity</span>}
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setCollapsed(!collapsed)}
            aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
          >
            {collapsed ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
          </Button>
        </div>

        <nav className="flex-1 space-y-1 p-2" role="navigation" aria-label="Main navigation">
          {navigation.map(({ path, label, icon: Icon, badge }) => {
            const active = pathname === path;
            return (
              <Link
                key={path}
                to={path}
                className={cn(
                  'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors focus-ring',
                  active
                    ? 'bg-verity-600 text-white'
                    : 'text-gray-400 hover:bg-gray-800 hover:text-white',
                )}
                aria-current={active ? 'page' : undefined}
              >
                <Icon size={20} aria-hidden="true" />
                {!collapsed && (
                  <>
                    <span className="flex-1">{label}</span>
                    {badge && (
                      <span className="rounded-full bg-red-500 px-1.5 py-0.5 text-xs text-white">
                        {badge}
                      </span>
                    )}
                  </>
                )}
              </Link>
            );
          })}
        </nav>

        {!collapsed && user && (
          <div className="border-t border-gray-800 p-3">
            <div className="flex items-center gap-3">
              <Avatar>
                <AvatarFallback className="flex h-8 w-8 items-center justify-center rounded-full bg-verity-600 text-xs font-bold">
                  {user.name?.charAt(0) ?? 'U'}
                </AvatarFallback>
              </Avatar>
              <div className="text-sm">
                <p className="font-medium">{user.name ?? 'Bank Operator'}</p>
                <p className="text-gray-400 text-xs">{user.role ?? 'Administrator'}</p>
              </div>
            </div>
          </div>
        )}
      </motion.aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto" role="main">
        <header className="sticky top-0 z-10 flex h-14 items-center gap-4 border-b bg-white px-6">
          <Bell size={20} className="text-gray-400" />
          <div className="flex-1" />
          <span className="text-sm text-gray-500">{new Date().toLocaleDateString()}</span>
        </header>
        <div className="p-6">{children}</div>
      </main>
    </div>
  );
}
