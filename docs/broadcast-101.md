# 📺 Broadcast 101 — A Developer's Guide to TV Playout

> **Who is this for?**
> Developers who have never worked in the TV broadcast industry but are about to build a professional playout system. We assume zero broadcast knowledge — only software development experience.

---

## Table of Contents

1. [How Does a TV Channel Actually Work?](#1-how-does-a-tv-channel-actually-work)
2. [What Is Playout — and Why Is It Critical?](#2-what-is-playout--and-why-is-it-critical)
3. [Why Can't We Just Use VLC?](#3-why-cant-we-just-use-vlc)
4. [The Main Components of a Playout System](#4-the-main-components-of-a-playout-system)
5. [Video Signals and Formats](#5-video-signals-and-formats)
6. [Scheduling — The Brain of a TV Channel](#6-scheduling--the-brain-of-a-tv-channel)
7. [CG and Graphics — Everything on Top of the Video](#7-cg-and-graphics--everything-on-top-of-the-video)
8. [Audio in Broadcast](#8-audio-in-broadcast)
9. [Advertising and Ad Insertion](#9-advertising-and-ad-insertion)
10. [Redundancy — Why Backup Is Not Optional](#10-redundancy--why-backup-is-not-optional)
11. [Monitoring — Knowing When Something Goes Wrong](#11-monitoring--knowing-when-something-goes-wrong)
12. [Glossary — Key Terms at a Glance](#12-glossary--key-terms-at-a-glance)

---

## 1. How Does a TV Channel Actually Work?

Imagine a TV channel like GEM TV. It broadcasts 24 hours a day, 7 days a week, 365 days a year. The question is: **where does that video actually come from?**

Here's the journey of a single video frame from a hard drive to your TV screen:

```
Hard Drive          Playout Server        Satellite / CDN       Viewer's TV
(Video Files)  →→→  (Our Software)   →→→  (Distribution)   →→→  (TV / Phone)
```

Think of it like this:

- The **video files** are the inventory — the movies, shows, promos sitting on disk.
- The **playout server** is the DJ booth — it decides what plays, when, and in what order, mixes everything together in real time, and sends out a single continuous video stream.
- The **satellite or CDN** is the delivery truck — it takes that stream and distributes it to millions of viewers.
- The **viewer's TV** just receives and decodes the stream.

**Our job is to build the DJ booth.**

---

## 2. What Is Playout — and Why Is It Critical?

**Playout** is the process of taking scheduled media files, stitching them together in real time, adding graphics and audio, and outputting a continuous, broadcast-quality video stream — without ever stopping.

The word "critical" is not an exaggeration. Here's why:

### Broadcast is live, real-time, and unforgiving

When a movie plays on TV, viewers expect:
- The picture to never freeze, stutter, or go black
- The audio to always be at a consistent volume
- The right program to appear at the right time
- The channel logo to always be in the corner
- Commercials to play at the right moments

If your playout software crashes at 9:00 PM on a Friday night and the channel goes black — **that is a business emergency**. Advertisers paid for those ad slots. Viewers change the channel. The broadcaster loses money and credibility every second the screen is black.

This is why playout software is engineered with a completely different mindset from typical software:

| Typical App | Playout System |
|-------------|---------------|
| Downtime is acceptable | Zero tolerance for downtime |
| Bugs can be patched later | A bug on air = instant crisis |
| Performance is "good enough" | Frame-perfect timing required |
| One user at a time | Running 24/7 unattended |
| Error = show a message | Error = automatic recovery |

---

## 3. Why Can't We Just Use VLC?

This is the first question every developer asks. The answer reveals the entire complexity of the problem.

**VLC plays one file at a time.** A playout system needs to:

### Problem 1: Seamless transitions
When one movie ends and the next begins, there must be **zero black frames** between them. VLC has a small but noticeable gap between files. On broadcast TV, even one black frame is visible and unacceptable. A playout engine pre-buffers the next clip before the current one finishes, so the transition is truly seamless.

### Problem 2: Timed scheduling
A news bulletin must start at **exactly** 8:00:00:00 (hours:minutes:seconds:frames). Not 8:00:00:02. Not 7:59:59:23. Frame-accurate timing. VLC has no concept of "start this file at a specific time."

### Problem 3: Graphics overlay in real time
While a movie is playing, the channel logo must appear in the top-right corner. The "now playing" ticker must scroll across the bottom. A breaking news banner might pop up. None of this is in the video file — it has to be **composited on top** in real time, every frame, at 25 or 50 frames per second.

### Problem 4: Multiple outputs simultaneously
The same playout engine must output to SDI hardware (for satellite), SRT stream (for OTT/internet), RTMP (for YouTube), and HLS (for the website) — all at the same time, all in sync.

### Problem 5: Automatic error recovery
If a video file is corrupted, missing, or the wrong format, the system must automatically skip it, play a backup "filler" clip, log the error, send an alert, and continue — all without human intervention at 3 AM.

VLC does none of these things. That's why professional playout software costs $50,000–$500,000 per channel per year. And that's what we're building.

---

## 4. The Main Components of a Playout System

A playout system is not one big program. It's several interconnected subsystems working together. Here's a map:

```
┌─────────────────────────────────────────────────────┐
│                  PLAYOUT SYSTEM                     │
│                                                     │
│  ┌─────────────┐    ┌─────────────────────────┐    │
│  │  SCHEDULER  │───▶│    PLAYOUT ENGINE        │    │
│  │             │    │  (the video machine)     │    │
│  │  What plays │    │                          │    │
│  │  When       │    │  Reads files from disk   │    │
│  │  For how    │    │  Decodes video/audio     │    │
│  │  long       │    │  Composites graphics     │    │
│  └─────────────┘    │  Outputs the stream      │    │
│                     └──────────┬────────────────┘   │
│  ┌─────────────┐               │                    │
│  │  CG ENGINE  │───────────────┘                    │
│  │             │    ┌─────────────────────────┐    │
│  │  Logo       │    │     OUTPUT LAYER         │    │
│  │  Ticker     │    │                          │    │
│  │  Clock      │───▶│  SDI (hardware)          │    │
│  │  Graphics   │    │  SRT / RTMP / HLS        │    │
│  └─────────────┘    │  NDI (network)           │    │
│                     └─────────────────────────┘    │
│  ┌──────────────────────────────────────────┐      │
│  │            WEB MANAGEMENT PANEL          │      │
│  │  Dashboard │ Schedule │ Media │ Alerts   │      │
│  └──────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────┘
```

### 4.1 The Playout Engine
This is the core. Think of it as a real-time video processing pipeline. It reads compressed video files from disk, decodes them frame by frame using the CPU/GPU, composites graphics on top, mixes the audio tracks, encodes the result, and sends it to the output.

It must do all of this at exactly 25 or 50 frames per second, without ever dropping a frame or falling behind. This is why the engine is written in **C++ or Rust** — not Python or JavaScript. Performance is not optional.

### 4.2 The Scheduler
This is the brain. It holds the answer to: "what should be playing right now?"

The scheduler maintains a playlist — an ordered list of clips with their planned start times. It handles gaps, overlaps, hard/soft starts, and importing schedules from traffic systems.

### 4.3 The CG Engine (Character Generator)
CG stands for "Character Generator." It composites graphics on top of the video in real time: channel logo, scrolling ticker, clock, lower-thirds, promotional banners.

These graphics are composited on top of every single video frame before output. At 50fps, that's 50 compositions per second, per channel.

### 4.4 The Web Management Panel
This is what the operators use. A browser-based interface for monitoring channels, editing schedules, managing media files, viewing alerts, and managing users. This is the only layer that uses standard web technologies (React, Node.js).

---

## 5. Video Signals and Formats

### 5.1 SDI — The Professional Video Cable

**SDI (Serial Digital Interface)** is the cable used in professional TV studios. It's the "ethernet cable" of the video world — except it carries uncompressed digital video.

Why SDI instead of HDMI?
- HDMI max cable length ~15 meters. SDI runs up to **300 meters**.
- SDI carries embedded audio (no separate cable needed).
- No copy protection (HDCP) that would interfere with professional workflows.
- Standardized signal levels used globally in broadcast.

For our playout software, SDI output is provided by a **capture card** installed in the Windows PC — typically a **Blackmagic DeckLink** or **AJA** card. Our software sends frames to this card via its SDK.

### 5.2 NDI — SDI Over Network

**NDI (Network Device Interface)** does the same job as SDI but over a standard ethernet network. No special cables needed — any device on the gigabit LAN can receive the stream. Low latency (typically under 100ms). Our software must support NDI as both input and output.

### 5.3 Streaming Protocols

| Protocol | Use Case | Latency |
|----------|----------|---------|
| **RTMP** | YouTube Live, Facebook Live | ~5–30 sec |
| **SRT** | Professional OTT, low-latency | ~0.5–5 sec |
| **HLS** | Websites, mobile apps, CDNs | ~10–30 sec |
| **DASH** | Cross-platform OTT | ~10–30 sec |
| **UDP** | Internal distribution, satellite muxers | ~0.1 sec |

Our playout system outputs to all of these simultaneously:

```
Playout Engine  ──▶  SDI    (satellite uplink)
                ──▶  SRT    (OTT / backup)
                ──▶  RTMP   (YouTube simulcast)
                ──▶  HLS    (website player)
```

### 5.4 Video Codecs

| Codec | Common Use |
|-------|-----------|
| **H.264 / AVC** | Standard streaming, web |
| **H.265 / HEVC** | 4K streaming |
| **ProRes** | Editing, production |
| **MXF / XDCAM** | Broadcast delivery |
| **DNxHD** | Avid editing |

Our playout engine must decode all of these **without pre-transcoding**. A system that requires all files to be in one format is a major operational burden.

### 5.5 Frame Rates

| Standard | Frame Rate | Used In |
|----------|-----------|---------|
| **PAL** | 25 fps | Europe, Middle East, Asia |
| **NTSC** | 29.97 fps | USA, Japan |
| **1080i/50** | 50 interlaced fields/sec | Global HD broadcast |
| **1080p/25** | 25 progressive frames/sec | Modern HD broadcast |
| **4K/50** | 50 fps @ 3840×2160 | Modern 4K broadcast |

The engine must automatically **convert** between formats when a file doesn't match the channel's output standard.

---

## 6. Scheduling — The Brain of a TV Channel

### 6.1 What Is a Schedule?

```
09:00:00  →  "Morning News"          (30 min)
09:30:00  →  "Commercial Break #1"   (3 min)
09:33:00  →  "Cooking Show S01E04"   (27 min)
10:00:00  →  "Commercial Break #2"   (3 min)
10:03:00  →  "The Pursuit of Happyness" (117 min)
```

Each entry is called an **event**. Every event has a start time, a reference to a media file, a duration, and optional parameters.

### 6.2 Traffic System Integration

A **traffic system** is external software used by the programming and ad sales teams. The workflow:

```
Traffic System  →  exports XML schedule  →  Our Scheduler
                                                │
                                          runs it on air
                                                │
Our Scheduler  →  exports as-run log  →  Traffic System
                                        (billing reconciliation)
```

Our scheduler must import schedule files from third-party traffic systems and export as-run logs back to them.

### 6.3 Hard Start vs Soft Start

- **Hard Start**: Event MUST start at the exact scheduled time. If the previous show is still running, cut it. Example: "9 o'clock news at exactly 09:00:00."
- **Soft Start**: Event starts when the previous one ends. Time is flexible.

### 6.4 Gaps and Overlaps

- **Gap**: Nothing scheduled for a time slot → auto-fill with a promo clip.
- **Overlap**: Two events at the same time → trim or delay one automatically.

The scheduler must resolve these 24/7 without human intervention.

### 6.5 EPG — Electronic Program Guide

The EPG is the "TV guide" — what's on each channel at what time. Our system generates EPG data from the schedule and exports it in **XMLTV format** for satellite providers, IPTV platforms, and TV guide services.

---

## 7. CG and Graphics — Everything on Top of the Video

### 7.1 Types of Broadcast Graphics

**Logo / Bug** — Channel logo always in the corner:
```
[GEM TV]  ← always here
    Video Content
```

**Lower Third** — Program or person name at the bottom:
```
    Video Content
┌─────────────────────┐
│ Now Playing:        │
│ The Dark Knight     │
└─────────────────────┘
```

**Ticker / Crawl** — Text scrolling across the bottom:
```
    Video Content
◀ Breaking News: Markets up 2% ... Weather: sunny ...
```

**Squeeze-Back / L-Shape** — Video squeezed to make room for promo graphics.

**Clock** — Real-time digital clock overlay.

### 7.2 How Compositing Works

```
Layer 4: Logo (PNG with alpha)
Layer 3: Ticker text
Layer 2: Lower third graphic      →  composite  →  final frame  →  output
Layer 1: Main video frame
```

This runs at 25 or 50 times per second, per channel, in real time.

---

## 8. Audio in Broadcast

### 8.1 Loudness Normalization — EBU R128

The problem: a movie might be at a certain volume. A commercial might be louder. Viewers constantly adjust their volume.

Regulators in Europe, the Middle East, and most countries **require** broadcasters to normalize audio loudness. The standard is **EBU R128**, measured in **LUFS** (Loudness Units relative to Full Scale). Target: **-23 LUFS** (Europe).

Our playout system must measure loudness in real time and apply automatic gain correction. Without this, we're not compliant with broadcast regulations.

### 8.2 Multi-Channel Audio

Professional broadcast typically carries 8–16 audio channels per video stream:
- Channels 1-2: Main language (stereo)
- Channels 3-4: Second language
- Channels 5-6: Music & Effects (M&E)
- Etc.

Our engine must handle all channels and allow operators to route them (e.g., "use channels 3-4 of this file as output channels 1-2").

---

## 9. Advertising and Ad Insertion

### 9.1 SCTE-35 — The Ad Cue Standard

**SCTE-35** is an industry standard for signaling ad break opportunities inside a video stream. It's a metadata marker that says:

> "Ad break starting here. Duration: 120 seconds."

In modern OTT delivery, the ad is not hardcoded into the stream:
1. Our playout system embeds a SCTE-35 marker at the ad break point
2. A downstream ad server reads the marker
3. The ad server inserts the correct ad for each viewer dynamically

This is called **DAI — Dynamic Ad Insertion**.

**SCTE-104** is the same concept but for SDI signals (embedded in the VANC data area of the SDI signal).

### 9.2 Ad Positions

- **Pre-roll**: Ad before the main content
- **Mid-roll**: Ad break during the content
- **Post-roll**: Ad after the content ends

### 9.3 As-Run Log for Billing

The as-run log proves to the advertiser that their ad aired. The traffic system uses it to generate invoices. Getting this right is directly tied to the channel's revenue.

---

## 10. Redundancy — Why Backup Is Not Optional

### 10.1 What Can Fail

| What Can Fail | Consequence | Solution |
|--------------|-------------|----------|
| Software crash | Black screen | Auto-restart, backup instance |
| Hard drive failure | Can't read files | RAID storage, dual servers |
| Network failure | No output | Dual NICs |
| Power failure | Everything off | UPS, dual PSU |
| OS crash | Everything stops | Watchdog process |

### 10.2 1+1 Hot Standby

Two servers run identically in parallel. If Server A fails, Server B takes over in under 1 second.

```
Server A (Primary)  ──▶  SDI Router  ──▶  Output
Server B (Backup)   ──▶  SDI Router
                         │
                   monitors A, switches automatically on failure
```

### 10.3 N+1 Redundancy

N active channels, 1 spare server. If any channel fails, the spare takes over. More cost-efficient for multi-channel facilities.

### 10.4 The Failover Process

When a failure is detected:
1. Detect failure (within 1–3 seconds)
2. Backup server starts playing from the correct position
3. SDI routing switches to backup output
4. Alert sent to operations team
5. Event logged with timestamp
6. Playout continues — no human needed

---

## 11. Monitoring — Knowing When Something Goes Wrong

### 11.1 What to Monitor

**Video:** Black frames, freeze detection, loss of signal

**Audio:** Silence, level out of range, audio/video sync error

**System:** CPU, GPU, disk space, disk I/O speed, network bandwidth

**Scheduling:** Missing clips, corrupted files, schedule gaps

### 11.2 As-Run Log

The definitive record of what played on air:

```
2025-11-14 09:00:00.000  START  clip="morning_news.mxf"       status=OK
2025-11-14 09:30:00.001  END    clip="morning_news.mxf"       status=OK
2025-11-14 09:30:12.500  ERROR  clip="commercial_04.mp4"      status=FILE_NOT_FOUND
2025-11-14 09:30:12.500  AUTO   clip="filler_promo.mp4"       reason=AUTO_FILL
```

### 11.3 Alert Methods

- **SNMP traps** — Standard protocol for NOC monitoring systems
- **Telegram / Slack** — Modern operations teams prefer this
- **Email** — For less urgent issues
- **Web panel** — Visual alerts in the dashboard

---

## 12. Glossary — Key Terms at a Glance

| Term | What It Means |
|------|--------------|
| **Playout** | Playing scheduled video as a continuous broadcast stream |
| **SDI** | Serial Digital Interface — the professional video cable standard |
| **NDI** | Network Device Interface — SDI over ethernet |
| **SRT** | Secure Reliable Transport — low-latency streaming protocol |
| **RTMP** | Real-Time Messaging Protocol — YouTube Live, Facebook Live |
| **HLS** | HTTP Live Streaming — for websites and mobile apps |
| **Codec** | Compression algorithm (H.264, H.265, ProRes...) |
| **MXF** | Material eXchange Format — broadcast delivery container |
| **CG** | Character Generator — the subsystem for on-screen graphics |
| **Ticker** | Scrolling text at the bottom of the screen |
| **Lower Third** | Graphic overlay in the lower portion of the frame |
| **EPG** | Electronic Program Guide — the TV schedule data |
| **Traffic System** | External software for schedule planning and ad sales |
| **As-Run Log** | Record of what actually aired, used for billing |
| **SCTE-35** | Standard for embedding ad break markers in a video stream |
| **DAI** | Dynamic Ad Insertion — different ads for different viewers |
| **EBU R128** | Standard for broadcast audio loudness normalization |
| **LUFS** | Unit for measuring audio loudness |
| **Redundancy** | Backup systems that take over automatically on failure |
| **Failover** | The automatic switch from a failed system to its backup |
| **SNMP** | Protocol for system monitoring and alerts |
| **Hard Start** | Event that must start at an exact clock time |
| **Soft Start** | Event that follows immediately after the previous one ends |
| **Gap Fill** | Auto-filling empty schedule slots with promo/filler content |
| **1+1 Redundancy** | One primary + one backup server, failover in < 1 second |
| **SMPTE ST 2110** | Standard for uncompressed video over IP networks |
| **FAST** | Free Ad-Supported Streaming Television |
| **OTT** | Over The Top — video delivered over the internet |
| **MAM** | Media Asset Management — system for storing and managing media files |
| **Channel-in-a-Box** | A single server that handles everything a broadcast channel needs |
| **VANC** | Vertical Ancillary Data — carries metadata inside an SDI signal |
| **Genlock** | Synchronizing video equipment to a common timing reference |

---

## Final Thought: What We're Building

To summarize everything in one paragraph:

We're building software that **never stops running**, takes a schedule of video files, plays them in order with frame-perfect timing, composites live graphics on top, normalizes the audio, inserts ad break markers, outputs everything simultaneously to SDI hardware and multiple streaming protocols, monitors itself for problems, automatically recovers from failures, logs every single event, and lets operators manage all of this through a web browser — across multiple TV channels at the same time.

That's what a professional playout system does. Now you know why it's hard — and why it's worth building properly.

---

*Document maintained by the OpenPlayout team. Questions? Open a GitHub issue with the label `docs`.*
