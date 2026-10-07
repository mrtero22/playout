# Playout Broadcast App: Milestones

Timeline is relative to project start (W1). Total plan: about 36 weeks to General Availability.

## 1. Architecture Decisions

| Layer | Choice | Why |
|:-|:-|:-|
| Playout engine | Rust + GStreamer (libav, NVDEC or VAAPI hardware decode) | Memory safe, no garbage collector pauses, hardware acceleration, low CPU use |
| Graphics | HTML5 templates in an isolated renderer process (Chromium Embedded), composited into the engine through shared memory | A graphics crash can never take down the program output |
| Control API | NestJS (TypeScript) | Fast to build, strong typing, easy to scale |
| Real time bus | NATS JetStream | Light, fast, built in persistence and replay |
| Data | PostgreSQL (schedules, assets, audit) and Redis (live state) | Proven and simple |
| Operator Web Panel | React + TypeScript, WebSocket for state, WebRTC for live preview | Responsive, real time, works on desktop and tablet |
| Inputs and outputs | SRT, RTMP, NDI, SDI (DeckLink), SMPTE ST 2110 ready | IP first design keeps the app current for years |
| Redundancy | Hot standby engine in lockstep, output arbitrator, systemd supervisor and watchdog, Keepalived for the control plane | Failover in under one second without operator action |
| AI assistant | [Top AI Assistance Recommendations by Priority](./AI-Assistant.md), tool calling against the control API, Whisper for captions | Natural language operations with human confirmation for on air actions |
| Observability | Prometheus, Grafana, Loki, OpenTelemetry | See problems before the audience does |
| Deployment | Engine on host under systemd, control plane in Docker Compose, Kubernetes later if needed | Lowest overhead for the engine |

## 2. Design Rules Behind the Requirements

| Requirement | How it is met |
|:-|:-|
| Should never crash | Process isolation, memory safe language, supervisor with auto restart, watchdog on frame output, soak and chaos testing |
| Backup app | Standby engine receives the same playlist and timecode, decodes and renders in lockstep, only the output switch differs |
| Graphics | Isolated renderer process, template library, data driven fields, preload and cache |
| Minimum server resources | Hardware decode, zero copy frames, single decode per source, shared memory IPC, no transcoding unless required |
| High performance, not obsolete | Modular plugin interfaces, open standards, versioned APIs, IP workflow support |
| Web Panel | Role based operator console with rundown, playlist, graphics control and monitoring |
| AI assistant | Tool calling over the same API as the panel, every on air action needs operator confirmation |
| Live responsive section | WebSocket state, WebRTC low latency preview, layout adapts to desktop, tablet and phone |

## 3. Milestones

### M0: Foundation and Discovery (W1 to W3)
**Goal:** Lock requirements, validate the stack, and set up the engineering base.

**Deliverables**
- Requirements document and target broadcast formats (resolution, frame rate, codecs)
- Proof of concept: play a file to SRT output with hardware decode
- Repository structure, CI pipeline, coding standards
- Hardware and server sizing baseline

**Exit criteria**
- PoC plays 1080p50 for 1 hour with no dropped frames
- Stack decisions signed off

### M1: Core Playout Engine MVP (W4 to W9)
**Goal:** A reliable engine that plays a playlist on air.

**Deliverables**
- Playlist model (clips, live inputs, fixed and follow start times)
- Frame accurate cueing, cuts and transitions
- Output to SRT, RTMP and file recording
- Engine command interface over NATS
- Audio handling (levels, loudness normalization)

**Exit criteria**
- 24 hour continuous playout with zero dropped frames
- Frame accurate transitions verified by test tools

### M2: Control Plane and Operator Web Panel (W10 to W15)
**Goal:** Operators can run the channel from a browser.

**Deliverables**
- NestJS API (auth, roles, assets, schedules, playlists)
- React Web Panel: rundown, playlist editor, transport controls
- Asset ingest and metadata, thumbnails, proxy generation
- Audit log of every operator action

**Exit criteria**
- An operator builds and can run a 2 hour or more (based on the operator role can be until one week) show from the panel alone
- Role permissions verified

### M3: Graphics (W16 to W20)
**Goal:** Broadcast quality overlays under operator control.

**Deliverables**
- Isolated HTML5 graphics renderer with shared memory compositing
- Template library: lower third, bug or logo, ticker, scoreboard, full screen
- Data driven templates (fields, JSON, spreadsheet feed)
- Graphics control in the Web Panel with preview

**Exit criteria**
- Renderer crash does not interrupt program output, verified by kill test
- Graphics take and clear are frame accurate

### M4: Redundancy and Resilience (W21 to W25)
**Goal:** Primary failure never reaches the audience.

**Deliverables**
- Hot standby engine in lockstep
- Output arbitrator and heartbeat health checks
- Supervisor, watchdog on frame output, automatic restart
- State recovery after restart (resume at the correct playlist position)
- Control plane high availability

**Exit criteria**
- Kill primary during playout: switch to backup in under 1 second, no visible glitch beyond one frame
- Chaos test suite passes

### M5: Live Responsive Section (W26 to W28)
**Goal:** Real time monitoring and live control on any device.

**Deliverables**
- WebRTC preview of program and preview output
- Real time state (on air item, countdown, next up, alarms) over WebSocket
- Responsive layout for desktop, tablet and phone
- Live alarms for black frame, silence, frozen frame, failover events

**Exit criteria**
- Preview latency under 1 second on LAN
- Panel usable on a 10 inch tablet and a phone

### M6: AI Assistant (W29 to W31)
**Goal:** Operators work faster with natural language help.

**Deliverables**
- Assistant inside the Web Panel with tool calling against the control API
- Commands such as "add the news package after the next break" with preview and confirm step
- Playlist checks (gaps, overlaps, missing media, loudness issues)
- Automatic captions with Whisper and incident summaries

**Exit criteria**
- No on air action runs without operator confirmation
- Assistant actions fully logged and reversible where possible

### M7: Performance, Hardening and Soak (W32 to W34)
**Goal:** Prove stability and efficiency.

**Deliverables**
- Resource profiling and optimization (CPU, GPU, memory, network)
- 14 day soak test, load test with multiple channels per server
- Security review and penetration test
- Disaster recovery and backup runbook

**Exit criteria**
- 14 days with zero crashes and zero dropped frames
- Resource targets met (defined in M0)

### M8: Pilot and General Availability (W35 to W36)
**Goal:** Go live with a real channel.

**Deliverables**
- Pilot on a real channel with operator training
- Documentation, runbooks, support process
- Release candidate and GA release

**Exit criteria**
- Pilot week with no breaking defects
- Sign off from operations team

## 4. Milestone Summary

| Milestone | Target | Main outcome |
|:-|:-|:-|
| M0 Foundation and Discovery | W3 | Stack validated |
| M1 Core Playout Engine | W9 | Playlist on air |
| M2 Control Plane and Web Panel | W15 | Browser operation |
| M3 Graphics | W20 | Overlays on air |
| M4 Redundancy and Resilience | W25 | Automatic failover |
| M5 Live Responsive Section | W28 | Real time monitoring on any device |
| M6 AI Assistant | W31 | Natural language operations |
| M7 Performance and Hardening | W34 | Proven stability |
| M8 Pilot and GA | W36 | Production release |

## 5. Risks

| Risk | Impact | Mitigation |
|:-|:-|:-|
| Frame accuracy problems at transitions | Visible glitches on air | Early prototype in M0, automated frame comparison tests |
| Chromium memory growth in graphics | Resource spikes | Isolated process, periodic recycle, memory limits |
| Failover desync between primary and standby | Jump or repeat at switch | Shared timecode, lockstep tests, chaos testing |
| AI assistant makes a wrong on air change | Wrong content on air | Confirmation step, role limits, audit log |
| Hardware differences across sites | Inconsistent performance | Sizing baseline in M0, hardware compatibility list |
