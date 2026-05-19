# 🎬 OpenPlayout — Professional TV Channel Playout & Scheduling System

> A world-class, multi-channel TV playout automation system built with modern technology to compete with industry leaders like Grass Valley, PlayBox Neo, and Imagine Communications.

[![Status](https://img.shields.io/badge/Status-In%20Development-yellow)]()
[![License](https://img.shields.io/badge/License-Private-red)]()
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Web-blue)]()

---

## 🌟 Vision

OpenPlayout is being built to deliver **broadcast-grade reliability** with **modern software architecture** — giving TV channel operators a powerful, cost-effective alternative to legacy playout systems. The system runs as a native Windows application for real-time video processing, paired with a full-featured web management panel accessible from any browser.

---

## 🎯 Core Objectives

- ✅ Compete feature-for-feature with Grass Valley Playout X, PlayBox Neo AirBox Neo-20, and Imagine Versio
- ✅ Multi-channel simultaneous playout (scalable from 1 to N channels)
- ✅ 24/7 unattended automation with intelligent failover
- ✅ Web-based management panel — no remote desktop needed
- ✅ Modern AI-assisted scheduling and QC
- ✅ Full IP workflow support (NDI, SRT, RTMP, HLS, ST 2110)
- ✅ Open REST API for third-party integration

---

## 🏗️ System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    INPUT SOURCES                        │
│  SDI/HDMI │ NDI │ SRT/RTMP │ File/MAM │ Live Feed       │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│              CORE PLAYOUT ENGINE (C++ / Rust)           │
│  ┌──────────────┐ ┌───────────┐ ┌──────────┐ ┌───────┐ │
│  │ Playback Eng │ │ Scheduler │ │ CG/Gfx   │ │ Audio │ │
│  │ 4K/HD/SD     │ │ 24/7 auto │ │ Ticker   │ │ R128  │ │
│  │ GPU accel    │ │ EPG gen   │ │ Logo/DVE │ │ AC3   │ │
│  └──────────────┘ └───────────┘ └──────────┘ └───────┘ │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│           PROFESSIONAL FEATURES LAYER                   │
│  Ad Insert │ Captions │ QC Monitor │ Redundancy │ AI    │
│  SCTE-35   │ CEA-708  │ As-run log │ N+1 auto  │ Smart │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│              OUTPUT & DISTRIBUTION                      │
│  SDI/ASI │ NDI │ SRT/RTMP │ HLS/DASH │ CDN │ Social    │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│           WEB MANAGEMENT PANEL (React)                  │
│  Dashboard │ Schedule Manager │ RBAC │ Analytics        │
└─────────────────────────────────────────────────────────┘
         ↕ REST API + WebSocket (Node.js / TypeScript)
```

---

## 📦 Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Core Engine** | C++ / Rust | Real-time video processing, frame-accurate playout |
| **Video I/O** | FFmpeg + Blackmagic DeckLink SDK | Codec support, SDI I/O |
| **AI Services** | Python | ML scheduling, auto-QC, caption generation |
| **Backend API** | Node.js + TypeScript | REST API, WebSocket, business logic |
| **Web Panel** | React + TypeScript | Management UI, real-time monitoring |
| **Database** | PostgreSQL | Schedules, assets, logs, users |
| **Cache/Events** | Redis | Real-time events, session, pub/sub |
| **Communication** | gRPC | Engine ↔ API communication |

---

## 🚀 Feature Roadmap

### Phase 1 — Foundation (MVP)
- [ ] Core playout engine (single channel, HD)
- [ ] Basic scheduling (file-based playlist)
- [ ] FFmpeg-based video output (SRT/RTMP/HLS)
- [ ] Web panel: channel dashboard + basic playlist control
- [ ] REST API skeleton

### Phase 2 — Professional Features
- [ ] Multi-channel support
- [ ] SDI I/O via Blackmagic DeckLink
- [ ] NDI input/output
- [ ] Real-time CG overlay (ticker, logo, clock)
- [ ] Advanced scheduling (gap/overlap auto-resolve, EPG)
- [ ] Audio loudness normalization (EBU R128)
- [ ] As-run log generation
- [ ] SCTE-35 ad insertion markers
- [ ] Closed captions (CEA-608/708)

### Phase 3 — Enterprise Features
- [ ] 4K/UHD support
- [ ] SMPTE ST 2110 IP workflow
- [ ] 1+1 redundancy with auto-failover
- [ ] AI smart scheduler
- [ ] AI-based QC monitoring
- [ ] DVE (Digital Video Effects)
- [ ] Traffic system integration (MOS, XML import)
- [ ] Multi-user RBAC
- [ ] SNMP monitoring + alerting
- [ ] MAM integration API

### Phase 4 — AI & Cloud
- [ ] AI auto-captioning (Persian + multilingual)
- [ ] Predictive QC (detect issues before air)
- [ ] Smart ad placement (AI-based)
- [ ] Cloud-burst / hybrid deployment
- [ ] FAST channel support
- [ ] Social media simulcast

---

## 📁 Repository Structure

```
openplayout/
├── engine/                  # C++/Rust core playout engine
│   ├── src/
│   │   ├── playback/        # Video playback engine
│   │   ├── scheduler/       # Automation & scheduling
│   │   ├── graphics/        # CG overlay engine
│   │   ├── audio/           # Audio processing
│   │   └── io/              # SDI, NDI, streaming I/O
│   └── CMakeLists.txt
│
├── api/                     # Node.js / TypeScript REST API
│   ├── src/
│   │   ├── routes/          # API endpoints
│   │   ├── services/        # Business logic
│   │   ├── models/          # Database models
│   │   └── grpc/            # Engine communication
│   └── package.json
│
├── web/                     # React management panel
│   ├── src/
│   │   ├── pages/           # Dashboard, Schedule, Media, etc.
│   │   ├── components/      # Reusable UI components
│   │   └── stores/          # State management
│   └── package.json
│
├── ai-services/             # Python AI services
│   ├── scheduler/           # Smart scheduling ML
│   ├── qc/                  # Automated QC
│   └── captions/            # Auto-caption service
│
├── docs/                    # Technical documentation
│   ├── architecture/
│   ├── api-reference/
│   └── deployment/
│
└── docker-compose.yml       # Dev environment
```

---

## 🏢 Team

| Role | Responsibility |
|------|---------------|
| @lead-dev | Architecture, Core Engine |
| @backend-dev | API, Database, Services |
| @frontend-dev | Web Panel, UI/UX |
| @broadcast-ops | Domain expertise, QA, Testing |

---

## 📋 Development Process

- Issues are organized by **Milestone** (Phase 1, 2, 3, 4)
- Each issue has labels: `engine`, `api`, `web`, `ai`, `devops`, `research`
- PRs require review before merge
- Main branch is always deployable

---

## 🔗 Competitor Reference

| Product | Company | Key Differentiator |
|---------|---------|-------------------|
| Playout X / AMPP | Grass Valley | Cloud-native, enterprise-scale |
| AirBox Neo-20 | PlayBox Neo | Best-selling Channel-in-a-Box |
| Versio / ADC | Imagine Communications | Traffic system integration |
| XPlayout | AxelTech | Full IP + cloud flexibility |
| MagicSoft Playout | MagicSoft | Cost-effective 4K solution |
| just:play pro 2026 | ToolsOnAir | NDI/SRT/ST-2110 versatility |

---

> Built with ❤️ by a team that knows What is DABE.
