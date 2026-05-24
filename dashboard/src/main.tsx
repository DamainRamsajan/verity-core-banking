import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { Toaster } from 'sonner';
import { IntlProvider } from 'next-intl';
import App from './App';
import { ErrorBoundary } from '@components/feedback/ErrorBoundary';
import './index.css';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30_000,
      retry: 3,
      refetchOnWindowFocus: false,
    },
  },
});

async function bootstrap() {
  const locale = navigator.language.startsWith('es') ? 'es' : 'en';
  const messages = (await import(`./i18n/locales/${locale}.json`)).default;

  ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
      <ErrorBoundary fallback={<div className="p-8 text-center">Something went wrong. Please refresh.</div>}>
        <IntlProvider locale={locale} messages={messages}>
          <QueryClientProvider client={queryClient}>
            <BrowserRouter>
              <App />
              <Toaster richColors position="top-right" />
            </BrowserRouter>
            <ReactQueryDevtools initialIsOpen={false} />
          </QueryClientProvider>
        </IntlProvider>
      </ErrorBoundary>
    </React.StrictMode>,
  );
}

bootstrap();
