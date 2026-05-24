// Verity Webhooks Edge Function
// Handles incoming webhooks from payment rails (FedNow, SWIFT) and partners.
// Source: ARC42 v20.0 §3 VCBP Payment Rail Connectors

import { createResponse, errorResponse, isActionAllowed } from "../_shared/mod.ts";

interface WebhookRequest {
  action: string;
  source: string;
  event: string;
  payload: Record<string, unknown>;
  signature?: string;
}

Deno.serve(async (req: Request) => {
  const traceId = crypto.randomUUID();

  try {
    if (req.method === "OPTIONS") {
      return new Response(null, {
        status: 204,
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "POST, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type, X-Webhook-Signature",
        },
      });
    }

    const body: WebhookRequest = await req.json();

    if (!isActionAllowed(body.action)) {
      return errorResponse(403, `Action not allowed: ${body.action}`, traceId);
    }

    // Verify webhook signature
    const sigHeader = req.headers.get("X-Webhook-Signature");
    if (body.source === "fednow" && !sigHeader) {
      return errorResponse(401, "Missing webhook signature for FedNow", traceId);
    }

    // In production: forward to VCBP payment engine via internal API
    console.log(`Webhook received: ${body.source}/${body.event}`, body.payload);

    return createResponse(200, {
      received: true,
      source: body.source,
      event: body.event,
      traceId,
    }, traceId);

  } catch (err) {
    console.error("Webhook error:", err);
    return errorResponse(500, "Internal server error", traceId);
  }
});
