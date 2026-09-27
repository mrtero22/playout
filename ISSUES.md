# 📋 GitHub Issues — OpenPlayout
## 🏁 MILESTONE: Phase 1 — MVP Foundation

### [ISSUE-001] Core Playout Engine — Single Channel HD
**Labels:** `engine` `phase-1` `high-priority`
**Description:**
Implement the core video playback engine capable of playing a single HD channel from a file playlist.
- Frame-accurate playback
- Support for H.264, H.265, ProRes, MXF, MP4 containers
- Auto format detection and conversion
- GPU-accelerated decoding (NVIDIA/AMD)
- Seamless back-to-back clip playback (no black frames)

**Acceptance Criteria:**
  - [ ] Plays a 30-clip playlist without interruption for 1 hour
  - [ ] Supports 1080i/50, 1080p/25, 1080p/30 output
  - [ ] Clip transition < 1 frame gap
  - [ ] CPU usage < 40% on target hardware

---

### [ISSUE-002] Basic Playlist Scheduler
**Labels:** `engine` `phase-1` `high-priority`

**Description:**
Build a file-based scheduler that manages a playlist and controls playback engine timing.
- Import playlist from XML/CSV
- Scheduled start times (fixed + floating)
- Automatic gap filling with "filler" content
- Overlap resolution (trim/skip logic)
- Save/load daily schedules

**Acceptance Criteria:**
- [ ] Load a 24-hour schedule from XML
- [ ] Auto-resolve time gaps and overlaps
- [ ] Support "hard start" (fixed time) events
- [ ] Log all played events to as-run.log

---

### [ISSUE-003] FFmpeg-based Streaming Output
**Labels:** `engine` `phase-1` `high-priority`

**Description:**
Output the playout engine video stream to multiple IP destinations simultaneously.
- RTMP output (YouTube, Facebook, Wowza)
- SRT output (low-latency streaming)
- HLS output (web player compatible)
- UDP multicast output
- Configurable bitrate, resolution, codec (H.264/H.265)

**Acceptance Criteria:**
- [ ] Simultaneous RTMP + SRT output
- [ ] HLS with 2-second segments
- [ ] Bitrate control: 1–50 Mbps
- [ ] Reconnect on disconnect (auto-retry)

---

### [ISSUE-004] REST API Skeleton
**Labels:** `api` `phase-1`

**Description:**
Create the Node.js/TypeScript REST API that connects the web panel to the playout engine.
- Channel CRUD operations
- Playlist management endpoints
- WebSocket for real-time status
- JWT authentication
- API documentation (Swagger/OpenAPI)

**Endpoints:**
- GET/POST /channels
- GET/PUT/DELETE /channels/:id/playlist
- GET /channels/:id/status (WebSocket)
- POST /auth/login

---

### [ISSUE-005] Web Panel — Channel Dashboard (MVP)
**Labels:** `web` `phase-1`

**Description:**
Build the first version of the web management panel with a channel monitoring dashboard.
- Live channel status (playing/stopped/error)
- Current & next clip display
- Basic playlist view (current day)
- Play/Stop/Skip controls
- Real-time clock display

**Tech:** React + TypeScript + Tailwind CSS

---

### [ISSUE-006] Database Schema Design
**Labels:** `api` `phase-1` `architecture`

**Description:**
Design and implement the PostgreSQL database schema for the entire system.

**Tables needed:**
- channels (id, name, config, status)
- playlists (id, channel_id, date, items[])
- media_assets (id, path, duration, format, metadata)
- as_run_logs (id, channel_id, clip_id, start_time, end_time, status)
- users (id, role, permissions)
- system_config (key, value)

---

## 🚀 MILESTONE: Phase 2 — Professional Features

### [ISSUE-007] Multi-Channel Support
**Labels:** `engine` `phase-2`

**Description:**
Extend the engine to support N simultaneous channels from a single server.
- Independent playout engine per channel
- Shared media asset storage
- Per-channel resource limits (CPU, GPU, bandwidth)
- Channel add/remove without restart

---

### [ISSUE-008] SDI Output via Blackmagic DeckLink
**Labels:** `engine` `phase-2` `hardware`

**Description:**
Integrate Blackmagic DeckLink SDK for professional SDI video output.
- Support DeckLink 4K Extreme, Studio, Mini Monitor cards
- 1080i/50, 1080p/25, 1080p/50, 4K output
- Embedded audio (16 channels)
- VANC data passthrough
- Hardware reference lock (genlock)

**Reference:** Blackmagic DeckLink SDK documentation

---

### [ISSUE-009] NDI Input and Output
**Labels:** `engine` `phase-2`

**Description:**
Add NDI (Network Device Interface) support for IP-based studio workflows.
- NDI source discovery (mDNS)
- NDI input as live source in playlist
- NDI output alongside other outputs
- NDI High Bandwidth + NDI HX support

**Reference:** NDI SDK by NewTek/Vizrt

---

### [ISSUE-010] Real-Time CG Overlay Engine
**Labels:** `engine` `phase-2` `graphics`

**Description:**
Build a compositing engine for real-time graphics overlays.

**Features:**
- Static/animated logo (with transparency)
- Scrolling ticker (horizontal text crawl)
- Rolling text (vertical)
- Digital clock overlay
- "Now Playing" lower-third graphic
- Schedule-driven: show/hide at specific times
- Multiple logo presets with fade transitions

**Tech:** OpenGL / Vulkan / DirectX compositing

---

### [ISSUE-011] Advanced Scheduler with EPG
**Labels:** `engine` `api` `phase-2`

**Description:**
Upgrade the scheduler to professional broadcast-grade capabilities.
- Fixed-time events (hard start)
- Follow-on events (soft/relative timing)
- Repeat/loop programs
- Auto-filler rules (fill gaps with promos)
- EPG (Electronic Program Guide) data export (XMLTV format)
- Import from traffic systems (XML, CSV, MXF)
- "Now/Next/Later" data feed for graphics

---

### [ISSUE-012] Audio Loudness Normalization (EBU R128)
**Labels:** `engine` `phase-2` `audio`

**Description:**
Implement real-time audio loudness control compliant with EBU R128 / ATSC A/85 standards.
- Real-time loudness measurement (LUFS)
- Automatic gain control per clip
- Multi-channel audio support (up to 16 channels)
- Audio mapping (channel routing)
- Audio monitoring in web panel (VU meters)

---

### [ISSUE-013] Closed Captions & Subtitles
**Labels:** `engine` `phase-2`

**Description:**
Support industry-standard caption and subtitle formats.
- CEA-608 / CEA-708 closed captions
- DVB Subtitles
- SRT subtitle files (overlay from file)
- STL subtitle format
- Schedule-based caption enable/disable
- Language selection

---

### [ISSUE-014] SCTE-35 Ad Insertion Markers
**Labels:** `engine` `phase-2` `advertising`

**Description:**
Implement SCTE-35 cue messages for ad insertion in broadcast/streaming workflows.
- SCTE-35 splice_insert and splice_null
- SCTE-104 triggers via SDI VANC
- Embed SCTE-35 in SRT/UDP/HLS streams
- Configurable ad break duration
- As-run log includes ad events

---

### [ISSUE-015] As-Run Log & Reporting
**Labels:** `api` `web` `phase-2`

**Description:**
Comprehensive as-run logging and reporting system.
- Log every played event (start time, end time, clip ID, duration, status)
- Discrepancy report (planned vs actual)
- Export to CSV/PDF
- Web panel report viewer
- Traffic system reconciliation export

---

### [ISSUE-016] Web Panel — Schedule Manager
**Labels:** `web` `phase-2`

**Description:**
Full-featured schedule management UI in the web panel.
- Week/day view calendar
- Drag-and-drop clip scheduling
- Import schedule from XML/CSV
- Edit individual events (trim in/out, duration)
- Conflict detection and resolution UI
- Preview scheduled content thumbnail

---

### [ISSUE-017] Web Panel — Media Asset Manager
**Labels:** `web` `phase-2`

**Description:**
Media library browser in the web panel.
- Browse and search media files
- Preview thumbnail + metadata
- Upload media via web browser
- Storage usage indicators
- Validation status per file (OK / Error)
- Tagging and categorization

---

### [ISSUE-018] Real-Time Monitoring & Alerts
**Labels:** `api` `web` `phase-2`

**Description:**
System monitoring dashboard and alert system.
- Live multiviewer (video thumbnail per channel in web panel)
- Audio level meters (web panel)
- System health: CPU, GPU, disk, network
- Alert rules: silence, freeze, low disk, clip missing
- SNMP trap support
- Telegram bot notifications
- Email alerts

---

## 🏢 MILESTONE: Phase 3 — Enterprise Features

### [ISSUE-019] 1+1 Redundancy with Auto-Failover
**Labels:** `engine` `api` `phase-3` `critical`

**Description:**
Implement professional redundancy for 24/7 broadcast reliability.
- Primary + backup engine synchronization
- Playlist replication between servers
- Automatic failover (< 1 second)
- Manual failover switch
- Health check heartbeat
- Database replication

---

### [ISSUE-020] SMPTE ST 2110 IP Workflow
**Labels:** `engine` `phase-3` `networking`

**Description:**
Full SMPTE ST 2110 support for professional IP studio environments.
- ST 2110-20 (uncompressed video)
- ST 2110-30 (audio)
- NMOS IS-04/IS-05 discovery and control
- PTP (IEEE 1588) synchronization

**Hardware:** Requires compatible IP video cards (Blackmagic IP)

---

### [ISSUE-021] 4K UHD Playout
**Labels:** `engine` `phase-3`

**Description:**
Full 4K/UHD playout capability.
- 3840×2160 @ 25/29.97/50/59.94 fps
- HDR support (HLG, PQ/HDR10)
- 4K SDI output (quad-link / 12G-SDI)
- 4K streaming output (H.265/HEVC)
- 4K CG overlay

---

### [ISSUE-022] Traffic System Integration
**Labels:** `api` `phase-3` `integration`

**Description:**
Two-way integration with broadcast traffic/scheduling systems.
- Import schedule from external traffic systems (XML, AS-11, MXF)
- Export as-run log back to traffic system
- MOS protocol support
- API webhooks for schedule updates
- Support common formats: BXF, VDCP

---

### [ISSUE-023] Multi-User RBAC (Role-Based Access Control)
**Labels:** `api` `web` `phase-3` `security`

**Description:**
Enterprise-grade user management and permission system.

**Roles:**
- Admin: full system access
- Operator: playlist control, no system config
- Viewer: read-only monitoring
- Scheduler: schedule editing only
- API: external system integration

**Features:**
- SSO/LDAP integration option
- Audit log (who did what, when)
- Session management

---

### [ISSUE-024] DVE — Digital Video Effects
**Labels:** `engine` `phase-3` `graphics`

**Description:**
Picture-in-picture and DVE capabilities.
- Picture-in-Picture (PiP) with position/size control
- Squeeze-back (main channel + PiP for promos)
- Wipe transitions between clips
- Schedule-driven DVE events

---

## 🤖 MILESTONE: Phase 4 — AI & Cloud

### [ISSUE-025] AI Smart Scheduler
**Labels:** `ai` `phase-4`

**Description:**
ML-based intelligent scheduling assistant.
- Analyze viewing patterns to optimize content placement
- Suggest optimal ad break positions
- Auto-fill gaps with contextually relevant content
- Rule-based + AI hybrid mode
- Integration with audience analytics data

**Tech:** Python, scikit-learn or LLM API

---

### [ISSUE-026] AI-Powered Auto-QC
**Labels:** `ai` `phase-4`

**Description:**
Automated quality control using machine learning.
- Detect black frames, freeze, audio silence before air
- Color/luminance anomaly detection
- Aspect ratio validation
- Missing subtitle detection
- Pre-air validation report

**Tech:** Python, OpenCV, FFmpeg

---

### [ISSUE-027] AI Auto-Captioning (Persian + Multilingual)
**Labels:** `ai` `phase-4` `persian`

**Description:**
Automatic subtitle generation using speech-to-text AI.
- Persian (Farsi) speech recognition
- Multi-language support (Arabic, English, etc.)
- Real-time and offline captioning modes
- Integration with playout for live caption overlay
- Persian text RTL rendering in CG engine

**Tech:** Whisper (OpenAI) + Persian fine-tuning

---

### [ISSUE-028] FAST Channel Support
**Labels:** `engine` `api` `phase-4`

**Description:**
Free Ad-Supported Streaming Television (FAST) channel capabilities.
- Ad-supported linear channel output
- Dynamic ad insertion for OTT
- SCTE-35 triggers in HLS/DASH streams
- EPG integration with FAST platform APIs
- Channel packaging for Samsung TV Plus, Pluto TV, etc.

---

### [ISSUE-029] Cloud & Hybrid Deployment
**Labels:** `devops` `phase-4`

**Description:**
Support cloud and hybrid deployment models.
- Docker containerization of all services
- Kubernetes orchestration
- Cloud burst (spin up additional channels on demand)
- Azure / AWS / on-prem flexible deployment
- License management for cloud instances

---

### [ISSUE-030] Social Media Simulcast
**Labels:** `engine` `phase-4`

**Description:**
Direct publishing to social platforms alongside broadcast.
- YouTube Live output
- Facebook Live output
- Instagram Live (RTMP)
- Configurable per-platform encoding settings
- Platform-specific graphic overlays

---

## 📐 MILESTONE: Infrastructure & DevOps

### [ISSUE-031] Development Environment Setup (Docker Compose)
**Labels:** `devops` `phase-1`

**Description:**
Create a Docker Compose setup for local development.
- PostgreSQL container
- Redis container
- API dev server
- Web panel dev server
- Mock engine simulator (for frontend dev without real hardware)

---

### [ISSUE-032] CI/CD Pipeline
**Labels:** `devops` `phase-2`

**Description:**
Automated testing and deployment pipeline.
- GitHub Actions workflow
- Unit tests (engine, API, web)
- Integration tests
- Build Windows installer for engine
- Build Docker images for API/web

---

### [ISSUE-033] API Documentation
**Labels:** `api` `docs` `phase-2`

**Description:**
Comprehensive API documentation.
- Swagger/OpenAPI spec
- Authentication guide
- WebSocket events reference
- Code examples (Python, JavaScript, curl)
- Postman collection
