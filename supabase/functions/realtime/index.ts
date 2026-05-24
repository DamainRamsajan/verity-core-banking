// Verity Realtime Edge Function
// Manages WebSocket connections for real-time agent activity streaming.
// Source: ARC42 v20.0 §5 Deployment View

import { createResponse, errorResponse } from "../_shared/mod.ts";

interface RealtimeRequest {
  action: string;
  channel?: string;
  userId?: string;
}

// In-memory rooms map (per Deno isolate)
const rooms = new Map<string, Set<string>>();

Deno.serve(async (req: Request) => {
  const traceId = crypto.randomUUID();

  try {
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

    const body: RealtimeRequest = await req.json();

    // Handle WebSocket upgrade for real-time streaming
    const upgradeHeader = req.headers.get("Upgrade");
    if (upgradeHeader === "websocket") {
      const { socket, response } = Deno.upgradeWebSocket(req);
      const channel = body.channel || "default";

      // Track connection
      if (!rooms.has(channel)) {
        rooms.set(channel, new Set());
      }
      const clientId = crypto.randomUUID();
      rooms.get(channel)!.add(clientId);

      socket.onclose = () => {
        rooms.get(channel)?.delete(clientId);
      };

      socket.onmessage = (event) => {
        // Broadcast to all clients in the channel
        const members = rooms.get(channel);
        if (members) {
          // In production: fan-out via Supabase Realtime Broadcast
        }
      };

      return response;
    }

    return createResponse(200, {
      action: body.action,
      channel: body.channel,
      activeConnections: rooms.get(body.channel || "default")?.size || 0,
    }, traceId);

  } catch (err) {
    console.error("Realtime error:", err);
    return errorResponse(500, "Internal server error", traceId);
  }
});
