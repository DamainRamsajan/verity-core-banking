import { useMemo } from 'react';
import {
  AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer,
} from 'recharts';
import { formatCurrency, formatCompact } from '@lib/utils';

interface Props { data: Array<{ date: string; volume: number }>; }

export function TransactionChart({ data }: Props) {
  const formatted = useMemo(() => data.map(d => ({
    ...d,
    formatted: formatCompact(d.volume),
  })), [data]);

  return (
    <ResponsiveContainer width="100%" height={300}>
      <AreaChart data={formatted} margin={{ top: 5, right: 20, left: 10, bottom: 5 }}>
        <defs>
          <linearGradient id="txGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#0ea5e9" stopOpacity={0.3} />
            <stop offset="95%" stopColor="#0ea5e9" stopOpacity={0} />
          </linearGradient>
        </defs>
        <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
        <XAxis dataKey="date" tick={{ fontSize: 12 }} />
        <YAxis tick={{ fontSize: 12 }} tickFormatter={(v: number) => formatCompact(v)} />
        <Tooltip formatter={(value: number) => [formatCurrency(value), 'Volume']} />
        <Area type="monotone" dataKey="volume" stroke="#0ea5e9" fill="url(#txGradient)" strokeWidth={2} />
      </AreaChart>
    </ResponsiveContainer>
  );
}
