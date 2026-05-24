import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';
import { Heart, TrendingUp, PiggyBank, Target } from 'lucide-react';
import { formatCurrency } from '@lib/utils';

export default function WellnessDashboard() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Financial Wellness Command Centre</h1>
      <div className="grid grid-cols-4 gap-4">
        {[
          { label: 'Monthly Spend', value: formatCurrency(3_240), icon: TrendingUp, color: 'text-blue-600' },
          { label: 'Savings Rate', value: '28%', icon: PiggyBank, color: 'text-green-600' },
          { label: 'Bills Due', value: '4', icon: Target, color: 'text-amber-600' },
          { label: 'Wellness Score', value: '82/100', icon: Heart, color: 'text-verity-600' },
        ].map(({ label, value, icon: Icon, color }) => (
          <Card key={label}>
            <CardContent>
              <div className="flex items-center gap-3">
                <Icon size={24} className={color} />
                <div><p className="text-sm text-gray-500">{label}</p><p className="text-2xl font-bold">{value}</p></div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
      <Card><CardHeader><CardTitle>Spending Insights</CardTitle></CardHeader><CardContent><p className="text-gray-500">AI-powered spending categorisation and trend analysis.</p></CardContent></Card>
    </div>
  );
}
