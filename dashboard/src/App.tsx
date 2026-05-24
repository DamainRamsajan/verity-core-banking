import { lazy, Suspense } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { DashboardLayout } from '@components/layout/DashboardLayout';
import { LoadingSpinner } from '@components/feedback/LoadingSpinner';

const GovernanceDashboard = lazy(() => import('@pages/governance/GovernanceDashboard'));
const WellnessDashboard   = lazy(() => import('@pages/wellness/WellnessDashboard'));
const LifeStageDashboard  = lazy(() => import('@pages/life-stage/LifeStageDashboard'));
const AgentFleet          = lazy(() => import('@pages/agents/AgentFleet'));
const ATMFleet            = lazy(() => import('@pages/atm/ATMFleet'));
const MigrationDashboard  = lazy(() => import('@pages/migration/MigrationDashboard'));
const ComplianceDashboard = lazy(() => import('@pages/compliance/ComplianceDashboard'));
const SettingsPage        = lazy(() => import('@pages/settings/SettingsPage'));

export default function App() {
  return (
    <DashboardLayout>
      <Suspense fallback={<LoadingSpinner />}>
        <Routes>
          <Route path="/" element={<Navigate to="/governance" replace />} />
          <Route path="/governance"   element={<GovernanceDashboard />} />
          <Route path="/wellness"     element={<WellnessDashboard />} />
          <Route path="/life-stage"   element={<LifeStageDashboard />} />
          <Route path="/agents"       element={<AgentFleet />} />
          <Route path="/atm"          element={<ATMFleet />} />
          <Route path="/migration"    element={<MigrationDashboard />} />
          <Route path="/compliance"   element={<ComplianceDashboard />} />
          <Route path="/settings"     element={<SettingsPage />} />
        </Routes>
      </Suspense>
    </DashboardLayout>
  );
}
