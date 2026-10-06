# OpenPlayout Core Engine

Single-channel HD playout engine with a small built-in web control surface.

The engine is intentionally thin around FFmpeg: FFprobe validates each playlist item, FFmpeg performs demux/decode/format conversion/output, and the Rust process owns playlist loading, lifecycle, status, and HTTP control.

## Runtime Requirements

- Rust 1.75+ to build.
- FFmpeg and FFprobe on `PATH`.
- NVIDIA GPU for the default production output path (`h264_nvenc`). Use `--hwaccel dxva2`, `--hwaccel d3d11va`, `--hwaccel qsv`, or `--hwaccel none` as appropriate for the target system.

FFmpeg supplies container/codec support for H.264, H.265/HEVC, ProRes, MXF, MOV, and MP4. The engine normalizes output to 1920x1080 at `1080i50`, `1080p25`, or `1080p30`.

## Build

```powershell
cargo build --release
```

## Run

```powershell
.\target\release\openplayout-core-engine.exe `
  --playlist .\config\playlist.example.csv `
  --standard 1080p25 `
  --output srt://127.0.0.1:9000?mode=caller `
  --bind 127.0.0.1:8080
```

Open `http://127.0.0.1:8080` for basic control.

## Playlist Format

CSV columns:

- `id`: operator-visible clip id.
- `path`: absolute path or path relative to the playlist file.
- `in`: optional in-point in seconds or `HH:MM:SS.mmm`.
- `duration`: optional playout duration in seconds or `HH:MM:SS.mmm`.

## HTTP API

- `GET /api/status`
- `GET /api/playlist`
- `POST /api/play`
- `POST /api/stop`
- `POST /api/restart`
- `POST /api/playlist` with a plain-text playlist path in the request body.

## Acceptance Test Procedure

1. Build in release mode on the target playout server.
2. Create a 30-clip CSV with representative H.264, H.265, ProRes, MXF, and MP4 media.
3. Run with the required output standard:

```powershell
.\target\release\openplayout-core-engine.exe --playlist C:\playout\thirty-clips.csv --standard 1080i50 --output udp://239.10.10.10:5000?pkt_size=1316
```

4. Monitor the encoded output with a downstream receiver or analyzer for at least one hour.
5. Verify transitions are below one output frame by inspecting PTS continuity at clip boundaries.
6. Confirm system CPU remains below 40% on the target hardware with GPU decode/encode enabled.

This repository cannot certify those hardware acceptance criteria without FFmpeg and target GPU hardware present on the test machine.

