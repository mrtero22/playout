# NATS Overview

## 1. What It Is

NATS is a small, very fast message broker. Services publish messages to named **subjects**, and any service subscribed to that subject receives them. It is a single lightweight binary that uses little CPU and memory, which fits the minimum resource goal of the Playout app.

## 2. Why the Playout App Uses It

| Need | How NATS helps |
|:-|:-|
| Keep panel traffic away from the engine | The engine only publishes to NATS and never talks to browsers directly |
| Real time state to the Web Panel | API containers subscribe once and push to browsers over WebSocket |
| Commands to the engine | The API sends commands such as take next, and gets a reply |
| Primary and backup engines | Both engines receive the same commands and timecode, so they stay in lockstep |
| Failover detection | Heartbeats on a subject let the standby and arbitrator see a failure within a second |
| Several API containers | Each one subscribes by itself, so no extra fan out layer is needed |

## 3. Core Concepts

| Concept | Meaning |
|:-|:-|
| Subject | A dotted name such as `engine.ch1.state` |
| Publish and subscribe | Every subscriber of a subject gets each message |
| Wildcards | `*` matches one token (`engine.*.state`), `>` matches the rest (`engine.>`) |
| Request and reply | A publisher sends a message and waits for one answer, used for commands |
| Queue groups | Subscribers in one group share messages, so only one of them handles each message |
| JetStream | Optional persistence layer that stores messages and can replay them |

## 4. Core NATS and JetStream

| | Core NATS | JetStream |
|:-|:-|:-|
| Delivery | At most once, fire and forget | At least once, with acknowledgements |
| Storage | None | Messages kept on disk |
| Speed | Fastest | Slightly slower |
| Use in this app | Heartbeats, live state deltas, preview requests | Commands that must not be lost, audit events, alarm history, replay after a restart |

Rule of thumb: use core NATS when the next message makes the last one obsolete (state, heartbeat), and use JetStream when losing a message would be a problem (commands, audit).

## 5. Subjects Used in the App

| Subject | Direction | Purpose |
|:-|:-|:-|
| `engine.<channel>.state` | Engine to API | State deltas (on air item, next up) |
| `engine.<channel>.alarm` | Engine to API | Alarms (black frame, silence, failover) |
| `engine.<channel>.heartbeat` | Engine to arbitrator and standby | Health checks |
| `engine.<channel>.cmd` | API to engine | Operator commands (request and reply) |
| `audit.>` | API to storage | Audit log events |

## 6. Code Example

```ts
import { connect, StringCodec } from 'nats'

const nc = await connect({ servers: 'nats://nats:4222' })
const sc = StringCodec()

// Publish a state delta (engine side)
nc.publish('engine.ch1.state', sc.encode(JSON.stringify(delta)))

// Subscribe to state for all channels (API side)
const sub = nc.subscribe('engine.*.state')
for await (const m of sub) {
  const delta = JSON.parse(sc.decode(m.data))
  // push delta to the WebSocket room for this channel
}

// Send a command and wait for the answer
const reply = await nc.request(
  'engine.ch1.cmd',
  sc.encode(JSON.stringify({ type: 'take_next' })),
  { timeout: 1000 },
)
```

In the NestJS API, wrap this connection in a `NatsService` so the WebSocket gateway can call `subscribe` and `publish` without handling connection details.

## 7. Running It

```yaml
nats:
  image: nats:2
  restart: unless-stopped
  command: ["-js"]
```

The `-js` flag enables JetStream.

- Keep NATS on a private network and never expose it to the internet.
- Turn on authentication and give each service its own credentials, with permissions limited to the subjects it needs.
- Mount a volume for JetStream data so stored messages survive a restart.
- For high availability, run a 3 node NATS cluster. It is one of the control plane tasks in the redundancy milestone.
- Watch connections, slow consumers, and message rates in [Prometheus](./PROMETHEUS.md).

## 8. Limits to Keep in Mind

- Core NATS drops messages if a subscriber is offline, so a reconnecting browser or service must request a fresh snapshot (the WebSocket design already does this).
- NATS carries control and state messages only. Video, audio, and live stream media must never pass through it.
