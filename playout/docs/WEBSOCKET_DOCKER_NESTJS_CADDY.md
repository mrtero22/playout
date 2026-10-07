# Real Time State over WebSocket: NestJS, Docker and Caddy

This document explains how the Web Panel receives real time state (on air item, countdown, next up, alarms) in the Playout app. It covers the architecture, the NestJS gateway, the Docker setup, the Caddy reverse proxy, and the checks to run before going on air.

## 1. Why Not Serverless

Classic serverless platforms (AWS Lambda, Google Cloud Functions, Vercel functions) cannot keep a WebSocket connection open, because each function runs briefly and then stops. They only work through a managed layer such as API Gateway WebSockets, Cloudflare Durable Objects, or a service like Ably or Pusher. That adds cost per message, extra latency, and less control over state, which is a poor fit for an on air panel.

**Decision:** run long lived NestJS containers in Docker.

## 2. Architecture

```
Playout Engine -> NATS -> NestJS API containers -> Caddy -> Browser (Web Panel)
                              |
                              +-> Redis (current state snapshot)
```

[What is NATS?](./NATS_OVERVIEW.md)

| Part | Role |
|:-|:-|
| Playout engine | Publishes state changes to NATS. Never talks to browsers, so panel traffic cannot affect program output |
| NATS | Delivers state changes to every API container |
| Redis | Holds the latest state per channel, used for snapshots |
| NestJS API containers | Subscribe to NATS and push updates to connected browsers through a WebSocket gateway |
| Caddy | Terminates HTTPS, proxies WebSocket and API traffic, spreads connections across API containers |
| Browser | Joins one room per channel, applies snapshot then deltas |

Each API container subscribes to NATS itself, so several containers can run side by side without a Redis adapter for fan out.

## 3. Message Flow

1. The browser connects and sends its JWT and channel id.
2. The gateway verifies the token and checks channel permission.
3. The browser joins the room `channel:<id>` and receives a full **snapshot** of the current state.
4. From then on it receives small **delta** messages, each with a sequence number.
5. If the browser sees a gap in sequence numbers, it asks for a fresh snapshot.
6. Alarms are sent as separate events so they are never delayed behind state updates.

## 4. NestJS Gateway

```ts
import {
  OnGatewayConnection,
  OnGatewayDisconnect,
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from '@nestjs/websockets'
import { OnModuleInit } from '@nestjs/common'
import { Server, Socket } from 'socket.io'

@WebSocketGateway({ namespace: '/live', transports: ['websocket'] })
export class LiveGateway
  implements OnModuleInit, OnGatewayConnection, OnGatewayDisconnect
{
  @WebSocketServer() server: Server

  constructor(
    private nats: NatsService,
    private state: StateService,
    private auth: AuthService,
  ) {}

  async handleConnection(client: Socket) {
    const user = await this.auth.verify(client.handshake.auth.token)
    if (!user) return client.disconnect()

    const channelId = String(client.handshake.query.channelId)
    if (!(await this.auth.canViewChannel(user, channelId))) {
      return client.disconnect()
    }

    client.data.channelId = channelId
    client.join(`channel:${channelId}`)
    client.emit('snapshot', await this.state.get(channelId))
  }

  handleDisconnect(client: Socket) {
    client.leave(`channel:${client.data.channelId}`)
  }

  @SubscribeMessage('resync')
  async resync(client: Socket) {
    client.emit('snapshot', await this.state.get(client.data.channelId))
  }

  onModuleInit() {
    this.nats.subscribe('engine.*.state', (channelId, delta) => {
      this.server.to(`channel:${channelId}`).emit('delta', delta)
    })

    this.nats.subscribe('engine.*.alarm', (channelId, alarm) => {
      this.server.to(`channel:${channelId}`).emit('alarm', alarm)
    })
  }
}
```

**Why websocket transport only:** with no long polling fallback, a connection stays on one container for its whole life, so no sticky sessions are needed behind the load balancer.

## 5. Browser Client

```ts
import { io } from 'socket.io-client'

const socket = io('https://panel.example.com/live', {
  transports: ['websocket'],
  auth: { token },
  query: { channelId },
})

let lastSeq = 0
let clockOffsetMs = 0

socket.on('snapshot', (state) => {
  store.replace(state)
  lastSeq = state.seq
  clockOffsetMs = state.serverTimeMs - Date.now()
})

socket.on('delta', (delta) => {
  if (delta.seq !== lastSeq + 1) {
    socket.emit('resync')
    return
  }
  store.apply(delta)
  lastSeq = delta.seq
})

socket.on('alarm', (alarm) => {
  alarms.show(alarm)
})
```

Socket.IO reconnects automatically. After every reconnect the server sends a new snapshot, so the panel always returns to the correct state.

## 6. Countdown Without Per Second Messages

Do not send a message every second. Send the start time and duration of the on air item once, and let the browser compute the countdown using the server clock offset from the snapshot.

```ts
function remainingMs(item) {
  const serverNow = Date.now() + clockOffsetMs
  return item.startsAtMs + item.durationMs - serverNow
}
```

This keeps network traffic and server load very low, and every operator sees the same countdown.

## 7. Docker Setup

```yaml
services:
  caddy:
    image: caddy:2
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
      - "443:443/udp"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile:ro
      - ./web/dist:/srv/web:ro
      - caddy_data:/data
      - caddy_config:/config
    depends_on: [api]

  api:
    build: ./api
    restart: unless-stopped
    deploy:
      replicas: 2
    environment:
      NATS_URL: nats://nats:4222
      REDIS_URL: redis://redis:6379
    depends_on: [nats, redis]

  nats:
    image: nats:2
    restart: unless-stopped
    command: ["-js"]

  redis:
    image: redis:7
    restart: unless-stopped

volumes:
  caddy_data:
  caddy_config:
```

**Notes**
- Do not publish the `api` port to the host. Only Caddy is reachable from outside.
- The `caddy_data` volume stores certificates. Without it, every container recreation requests new certificates and can hit Let's Encrypt rate limits.
- The playout engine runs on the host under systemd (see the milestone architecture) and reaches NATS through a published NATS port on a private network only.

## 8. Caddy Configuration

```
panel.example.com {
	encode zstd gzip

	@api path /api/* /socket.io/*
	handle @api {
		reverse_proxy {
			dynamic a {
				name api
				port 3000
				refresh 5s
			}
			lb_policy least_conn
			lb_try_duration 3s
			lb_try_interval 250ms
			stream_close_delay 5m
		}
	}

	handle {
		root * /srv/web
		try_files {path} /index.html
		file_server
	}
}
```

| Setting | Purpose |
|:-|:-|
| `dynamic a` | Docker DNS returns every API container for the name `api`. Caddy re resolves it every 5 seconds, so all replicas receive traffic and a restarted container is picked up automatically |
| `lb_policy least_conn` | Spreads long lived WebSocket connections evenly |
| `lb_try_duration` and `lb_try_interval` | If one API container is down, the request is retried on another instead of failing |
| `stream_close_delay 5m` | Keeps WebSocket connections open for up to 5 minutes when Caddy reloads its config, so a reload never kicks operators off the live panel |
| `try_files` | Serves the React app for any panel route |
| `encode zstd gzip` | Compresses responses (WebSocket frames are not affected) |

Caddy handles WebSocket upgrades automatically, so no upgrade header configuration is needed.

## 9. What Caddy Does Not Handle

- **Media traffic:** SRT, RTMP, NDI and WebRTC media (UDP) must go straight to the engine or ingest ports, not through Caddy.
- **Viewer delivery:** for HLS in the live stream milestone, put a CDN in front rather than serving all viewers from Caddy.
- **Service discovery by labels:** Caddy has none, which does not matter here because the service list is small and fixed.

## 10. Pre Air Checklist

| Check | How to test | Expected result |
|:-|:-|:-|
| Snapshot on connect | Open the panel on a clean browser | Full state shown right away |
| Delta ordering | Drop a delta in a test client | Client requests a resync and recovers |
| API container restart | Stop one API container during a session | Browser reconnects in seconds and state is correct |
| Caddy reload | Reload Caddy while operators are connected | No disconnects |
| Load spread | Open 100 test connections | Connections spread across both API containers |
| Network loss | Disconnect a tablet from Wi-Fi for 30 seconds | Panel reconnects and resyncs on its own |
| Auth | Connect with an invalid token or wrong channel | Connection refused |
| Alarm latency | Trigger a black frame alarm | Alarm visible in the panel within 1 second |

## 11. Troubleshooting

| Symptom | Likely cause | Fix |
|:-|:-|:-|
| Connection fails with 400 or falls back to polling | Client allows polling and the load balancer has no sticky sessions | Set `transports: ['websocket']` on both client and gateway |
| Operators disconnected on config change | `stream_close_delay` missing | Add `stream_close_delay 5m` to `reverse_proxy` |
| All traffic goes to one API container | Static upstream instead of dynamic DNS | Use `dynamic a` as shown above |
| New certificate requested on every restart | `caddy_data` volume missing | Mount the volume |
| Stale state after reconnect | Client skipped the snapshot | Always replace the store on `snapshot` |
| Countdown differs between operators | Browsers using local clocks | Use the server clock offset from the snapshot |
