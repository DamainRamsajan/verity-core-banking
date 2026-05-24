import { z } from 'zod';

export const agentBoundarySchema = z.object({
  agentId: z.string().uuid(),
  spendingLimit: z.number().positive().max(1_000_000_000),
  approvalThreshold: z.number().positive(),
  allowedOperations: z.array(z.string()).min(1),
  counterpartyAllowlist: z.array(z.string()).optional(),
  jurisdictionAllowlist: z.array(z.string()).optional(),
  timeWindowStart: z.string().regex(/^\d{2}:\d{2}$/).optional(),
  timeWindowEnd: z.string().regex(/^\d{2}:\d{2}$/).optional(),
});

export type AgentBoundary = z.infer<typeof agentBoundarySchema>;
