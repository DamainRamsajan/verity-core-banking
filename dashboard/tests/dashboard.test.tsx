import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { DashboardLayout } from '../src/components/layout/DashboardLayout';

const qc = new QueryClient({ defaultOptions: { queries: { retry: false } } });

describe('DashboardLayout', () => {
  it('renders navigation links', () => {
    render(
      <QueryClientProvider client={qc}>
        <MemoryRouter><DashboardLayout><div /></DashboardLayout></MemoryRouter>
      </QueryClientProvider>,
    );
    expect(screen.getByText('Verity')).toBeDefined();
  });
});
