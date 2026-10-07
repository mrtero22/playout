# Playout Broadcast App: Tasks

## 1. Priority Scheme

Work is always picked in this order: Urgent, then Highest, then Very High, then High.

| Priority | Meaning | Rule of thumb |
|:-|:-|:-|
| Urgent | Crucial features to have in the app | Without it the app cannot go on air or cannot be trusted on air (playout core, outputs, failover, never crash) |
| Highest | Very important features to have in the app | The app works without it, but not as a usable broadcast product (operator control, graphics, recovery, security) |
| Very High | Important features to have in the app | Clearly expected by operators, and the app is weaker without it (monitoring, alarms, audit, documentation) |
| High | Enhancements or improvements | Adds comfort, speed or reach, and can ship after the core is stable |

**Rules**
- A task that protects on air output or failover is never lower than Highest.
- A milestone exit review and stabilization task is included in every milestone so defects found in testing are fixed before moving on.
- Enhancements move up one level if operators report that the missing item slows live work.

## 2. Tasks by Milestone

### M0: Foundation and Discovery (W1 to W3)

| ID | Task | Priority |
|:-|:-|:-|
| T0.1 | Write requirements: formats, frame rates, codecs, channel count, latency targets | Urgent |
| T0.2 | Define resource targets (CPU, GPU, RAM per channel) | Highest |
| T0.3 | Build PoC: file to SRT output with GStreamer and hardware decode | Urgent |
| T0.4 | Set up repository structure, branching strategy, code standards | Very High |
| T0.5 | Set up CI pipeline (build, lint, unit test, container build) | Very High |
| T0.6 | Set up staging environment and observability baseline | Very High |
| T0.7 | M0 exit review and stabilization of PoC issues | Highest |

### M1: Core Playout Engine MVP (W4 to W9)

| ID | Task | Priority |
|:-|:-|:-|
| T1.1 | Engine process skeleton in Rust with command interface over NATS | Urgent |
| T1.2 | Playlist data model (clip, live input, fixed start, follow) | Urgent |
| T1.3 | Frame accurate clip cueing and preloading | Urgent |
| T1.4 | Transitions: cut, dissolve, fade to black | Highest |
| T1.5 | Live input handling (SRT, RTMP, NDI) with fallback slate | Highest |
| T1.6 | Outputs: SRT, RTMP, file recording | Urgent |
| T1.7 | Audio pipeline: channel mapping, loudness normalization | Highest |
| T1.8 | Filler and slate content when the playlist runs empty | Highest |
| T1.9 | Timecode and clock source (system clock, PTP ready) | Highest |
| T1.10 | Structured logging and metrics from the engine | Very High |
| T1.11 | Automated frame accuracy test harness | Very High |
| T1.12 | M1 exit review and stabilization | Highest |

### M2: Control Plane and Operator Web Panel (W10 to W15)

| ID | Task | Priority |
|:-|:-|:-|
| T2.1 | NestJS API: auth, roles and permissions | Highest |
| T2.2 | Asset ingest service, metadata extraction, thumbnails, proxy files | Highest |
| T2.3 | Schedule and playlist API with validation (gaps, overlaps) | Urgent |
| T2.4 | Bridge between API and engine over NATS | Urgent |
| T2.5 | React Web Panel shell, routing, design system | Highest |
| T2.6 | Rundown and playlist editor with drag and drop | Highest |
| T2.7 | Transport controls: play, pause, skip, take next, hold | Urgent |
| T2.8 | Asset browser with search and preview | Very High |
| T2.9 | Audit log of all operator actions | Very High |
| T2.10 | API rate limits, input validation, security headers | Very High |
| T2.11 | M2 exit review and stabilization | Highest |

### M3: Graphics (W16 to W20)

| ID | Task | Priority |
|:-|:-|:-|
| T3.1 | Isolated renderer process with Chromium Embedded | Urgent |
| T3.2 | Shared memory frame transfer into the engine compositor | Urgent |
| T3.3 | Template format and loader (HTML, CSS, JS, JSON schema for fields) | Highest |
| T3.4 | Template set: lower third, logo bug, ticker, scoreboard, full screen | Highest |
| T3.5 | Data feeds into templates (JSON, spreadsheet, REST) | Very High |
| T3.6 | Take, update and clear commands with frame accurate timing | Highest |
| T3.7 | Graphics panel in the Web Panel with preview | Highest |
| T3.8 | Renderer watchdog, memory limit and automatic recycle | Highest |
| T3.9 | M3 exit review and stabilization | Highest |

### M4: Redundancy and Resilience (W21 to W25)

| ID | Task | Priority |
|:-|:-|:-|
| T4.1 | Standby engine running in lockstep from the same playlist and timecode | Urgent |
| T4.2 | Heartbeat and health checks over NATS | Urgent |
| T4.3 | Output arbitrator that switches output sources on failure | Urgent |
| T4.4 | Frame output watchdog (detect black, frozen, silent output) | Urgent |
| T4.5 | Supervisor with systemd, automatic restart and backoff | Urgent |
| T4.6 | State recovery: resume at the right position after restart | Highest |
| T4.7 | Control plane high availability (Keepalived, database replication) | Highest |
| T4.8 | Automatic resync of the failed engine as the new standby | Very High |
| T4.9 | Chaos test suite: kill process, drop network, fill disk, exhaust memory | Highest |
| T4.10 | Alerting rules and on call runbook | Very High |
| T4.11 | M4 exit review and stabilization | Highest |

### M5: Live Responsive Section (W26 to W28)

| ID | Task | Priority |
|:-|:-|:-|
| T5.1 | WebRTC preview service for program and preview feeds | Highest |
| T5.2 | Real time state channel over WebSocket (on air, countdown, next up) | Highest |
| T5.3 | Live dashboard: output health, audio meters, alarms | Highest |
| T5.4 | Responsive layouts for desktop, tablet and phone | Highest |
| T5.5 | Touch friendly controls and safe confirmation for on air actions | Very High |
| T5.6 | Alarm center: black frame, silence, frozen frame, failover events | Very High |
| T5.7 | Reconnect and state resync after network loss | Very High |
| T5.8 | M5 exit review and stabilization | Highest |

### M6: AI Assistant (W29 to W31)

| ID | Task | Priority |
|:-|:-|:-|
| T6.1 | Assistant service with Claude API and tool calling over the control API | Very High |
| T6.2 | Tool definitions: search assets, edit playlist, schedule, graphics, status | Very High |
| T6.3 | Confirmation flow: preview of the change and explicit operator approval | Highest |
| T6.4 | Role based limits on what the assistant may do | Highest |
| T6.5 | Playlist checks: gaps, overlaps, missing media, loudness | High |
| T6.6 | Automatic captions with Whisper | High |
| T6.7 | Incident summaries from logs and alarms | High |
| T6.8 | Assistant panel inside the Web Panel with history | Very High |
| T6.9 | Full logging and undo support for assistant actions | Very High |
| T6.10 | M6 exit review and stabilization | Very High |

### M7: Performance, Hardening and Soak (W32 to W34)

| ID | Task | Priority |
|:-|:-|:-|
| T7.1 | Profile CPU, GPU, memory and network per channel and optimize hot paths | Highest |
| T7.2 | Multi channel load test on one server | Very High |
| T7.3 | 14 day soak test with automated monitoring | Urgent |
| T7.4 | Security review, dependency audit and penetration test | Highest |
| T7.5 | Backup and disaster recovery runbook with a tested restore | Very High |
| T7.6 | Plugin and API versioning policy to keep the app current | Very High |
| T7.7 | M7 exit review and stabilization | Highest |

### M8: Pilot and General Availability (W35 to W36)

| ID | Task | Priority |
|:-|:-|:-|
| T8.1 | Pilot deployment on a real channel | Highest |
| T8.2 | Operator training and quick reference guide | Very High |
| T8.3 | Documentation: install, operate, troubleshoot | Very High |
| T8.4 | Support process and defect intake flow | Very High |
| T8.5 | Release candidate, release notes, GA release | Highest |
| T8.6 | GA exit review and stabilization | Highest |

## 3. Ongoing Tasks (All Milestones)

| ID | Task | Priority |
|:-|:-|:-|
| TX.1 | Investigate and fix any crash, black or frozen output, or failed failover | Urgent |
| TX.2 | Weekly dependency updates and security patches | Very High |
| TX.3 | Keep dashboards, alerts and runbooks current | Very High |
| TX.4 | Review resource usage each milestone against targets from T0.2 | High |

## 4. Enhancement Backlog (After GA)

| ID | Task | Priority |
|:-|:-|:-|
| TE.1 | Keyboard shortcuts for common operator actions | High |
| TE.2 | Light and dark theme for the Web Panel | High |
| TE.3 | Visual template editor for graphics | High |
| TE.4 | Multi channel overview dashboard | High |
| TE.5 | SMPTE ST 2110 output support | High |
| TE.6 | Ad break markers (SCTE-35) | High |
| TE.7 | Assistant suggestions for schedule filling based on past playlists | High |
| TE.8 | Multi language Web Panel | High |
