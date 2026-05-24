import { z } from 'zod';

export const transactionSchema = z.object({
  fromAccount: z.string().uuid(),
  toAccount: z.string().min(1),
  amount: z.number().positive().max(10_000_000),
  currency: z.string().length(3),
  reference: z.string().max(140).optional(),
});

export type Transaction = z.infer<typeof transactionSchema>;
