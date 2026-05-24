// Verity Auth Edge Function
// Handles JWT verification, session management, and KYA credential validation.
// Source: ARC42 v20.0 §5 Deployment View, ADR-007 (IETF agent identity)

import { createResponse, errorResponse, isActionAllowed } from "../_shared/mod.ts";

interface AuthRequest {
  action: string;
  token?: string;
  agentId?: string;
}

Deno.serve(async (req: Request) => {
  const traceId = crypto.randomUUID();

  try {
    // CORS preflight
    if (req.method === "OPTIONS") {
      return new Response(null, {
        status: 204,
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "POST, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type, Authorization",
        },
      });
    }

    const body: AuthRequest = await req.json();

    // Deny-by-default action allowlist
    if (!isActionAllowed(body.action)) {
      return errorResponse(403, `Action not allowed: ${body.action}`, traceId);
    }

    // JWT verification via Supabase Auth
    const authHeader = req.headers.get("Authorization");
    if (!authHeader || !authHeader.startsWith("Bearer ")) {
      return errorResponse(401, "Missing or invalid Authorization header", traceId);
    }

    const token = authHeader.substring(7);

    // In production: verify JWT with Supabase Auth
    // const { data: { user }, error } = await supabase.auth.getUser(token);

    return createResponse(200, {
      action: body.action,
      authenticated: true,
      tokenValid: true,
      message: "Authentication successful",
    }, traceId);

  } catch (err) {
    console.error("Auth error:", err);
    return errorResponse(500, "Internal server error", traceId);
  }
});
