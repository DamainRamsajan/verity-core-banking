// Verity Supabase Edge Functions — Shared Utilities
// Source: ARC42 v20.0 §5 Deployment View

export interface VerityRequest {
  traceId: string;
  userId?: string;
  agentId?: string;
  action: string;
  payload: Record<string, unknown>;
}

export interface VerityResponse {
  status: number;
  body: Record<string, unknown>;
  traceId: string;
  timestamp: string;
}

export function createResponse(
  status: number,
  body: Record<string, unknown>,
  traceId: string,
): Response {
  const resp: VerityResponse = {
    status,
    body,
    traceId,
    timestamp: new Date().toISOString(),
  };
  return new Response(JSON.stringify(resp), {
    status,
    headers: {
      "Content-Type": "application/json",
      "X-Trace-Id": traceId,
      "Access-Control-Allow-Origin": "*",
    },
  });
}

export function errorResponse(
  status: number,
  message: string,
  traceId: string,
): Response {
  return createResponse(status, { error: message }, traceId);
}

// Deny-by-default action allowlist
const ALLOWED_ACTIONS = new Set([
  "auth.login",
  "auth.verify",
  "dashboard.summary",
  "agent.activity",
  "realtime.subscribe",
  "webhook.stripe",
  "webhook.twilio",
]);

export function isActionAllowed(action: string): boolean {
  return ALLOWED_ACTIONS.has(action);
}
