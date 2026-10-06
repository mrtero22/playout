use std::fs;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::ffmpeg;
use crate::playlist::{json_escape, Playlist};
use crate::timecode::Rational;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoStandard {
    I1080i50,
    P1080p25,
    P1080p30,
}

impl VideoStandard {
    pub fn rate(self) -> Rational {
        match self {
            Self::I1080i50 | Self::P1080p25 => Rational::new(25, 1),
            Self::P1080p30 => Rational::new(30, 1),
        }
    }
}

impl FromStr for VideoStandard {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "1080i50" => Ok(Self::I1080i50),
            "1080p25" => Ok(Self::P1080p25),
            "1080p30" => Ok(Self::P1080p30),
            other => Err(format!("unsupported video standard {other}; use 1080i50, 1080p25, or 1080p30")),
        }
    }
}

impl std::fmt::Display for VideoStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I1080i50 => write!(f, "1080i50"),
            Self::P1080p25 => write!(f, "1080p25"),
            Self::P1080p30 => write!(f, "1080p30"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub output: String,
    pub standard: VideoStandard,
    pub hwaccel: String,
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
}

#[derive(Clone, Debug)]
pub enum EngineState {
    Stopped,
    Starting,
    Playing,
    Error(String),
}

pub struct PlayoutEngine {
    config: EngineConfig,
    playlist: Option<Playlist>,
    state: EngineState,
    process: Option<Child>,
    started_at: Option<SystemTime>,
}

impl PlayoutEngine {
    pub fn new(config: EngineConfig, playlist: Option<Playlist>) -> Self {
        Self {
            config,
            playlist,
            state: EngineState::Stopped,
            process: None,
            started_at: None,
        }
    }

    pub fn load_playlist(&mut self, playlist: Playlist) {
        self.playlist = Some(playlist);
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.is_running() {
            return Ok(());
        }
        let playlist = self
            .playlist
            .as_ref()
            .ok_or_else(|| "no playlist loaded".to_string())?;
        ffmpeg::validate_playlist(&self.config, playlist)?;
        let concat_file = write_concat_file(playlist)?;
        let args = ffmpeg::build_ffmpeg_args(&self.config, &concat_file);

        self.state = EngineState::Starting;
        let child = Command::new(&self.config.ffmpeg_path)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                self.state = EngineState::Error(err.to_string());
                format!("failed to start ffmpeg: {err}")
            })?;

        self.process = Some(child);
        self.started_at = Some(SystemTime::now());
        self.state = EngineState::Playing;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.process.take() {
            child.kill().map_err(|err| format!("failed to stop ffmpeg: {err}"))?;
            let _ = child.wait();
        }
        self.started_at = None;
        self.state = EngineState::Stopped;
        Ok(())
    }

    pub fn restart(&mut self) -> Result<(), String> {
        self.stop()?;
        self.start()
    }

    pub fn status_json(&mut self) -> String {
        self.refresh_process_state();
        let state = match &self.state {
            EngineState::Stopped => "stopped".to_string(),
            EngineState::Starting => "starting".to_string(),
            EngineState::Playing => "playing".to_string(),
            EngineState::Error(err) => format!("error: {}", json_escape(err)),
        };
        let uptime_ms = self
            .started_at
            .and_then(|t| t.elapsed().ok())
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let playlist_len = self.playlist.as_ref().map(|p| p.clips.len()).unwrap_or(0);
        format!(
            "{{\"state\":\"{}\",\"standard\":\"{}\",\"frameRate\":\"{}\",\"frameDurationMs\":{},\"output\":\"{}\",\"hwaccel\":\"{}\",\"uptimeMs\":{},\"playlistClips\":{}}}",
            json_escape(&state),
            self.config.standard,
            self.config.standard.rate(),
            self.config.standard.rate().frame_duration().as_millis(),
            json_escape(&self.config.output),
            json_escape(&self.config.hwaccel),
            uptime_ms,
            playlist_len
        )
    }

    pub fn playlist_json(&self) -> String {
        self.playlist
            .as_ref()
            .map(Playlist::to_json)
            .unwrap_or_else(|| "[]".to_string())
    }

    fn is_running(&mut self) -> bool {
        self.refresh_process_state();
        matches!(self.state, EngineState::Starting | EngineState::Playing)
    }

    fn refresh_process_state(&mut self) {
        if let Some(child) = self.process.as_mut() {
            match child.try_wait() {
                Ok(Some(status)) => {
                    self.process = None;
                    self.started_at = None;
                    self.state = if status.success() {
                        EngineState::Stopped
                    } else {
                        EngineState::Error(format!("ffmpeg exited with {status}"))
                    };
                }
                Ok(None) => {}
                Err(err) => {
                    self.process = None;
                    self.started_at = None;
                    self.state = EngineState::Error(format!("failed to query ffmpeg: {err}"));
                }
            }
        }
    }
}

fn write_concat_file(playlist: &Playlist) -> Result<std::path::PathBuf, String> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis();
    path.push(format!("openplayout-playlist-{stamp}.ffconcat"));

    let mut file = fs::File::create(&path)
        .map_err(|err| format!("failed to create concat file {}: {err}", path.display()))?;
    writeln!(file, "ffconcat version 1.0").map_err(|err| err.to_string())?;
    for clip in &playlist.clips {
        let escaped = clip.path.display().to_string().replace('\'', "'\\''");
        writeln!(file, "file '{}'", escaped).map_err(|err| err.to_string())?;
        if clip.in_point > Duration::ZERO {
            writeln!(file, "inpoint {:.3}", clip.in_point.as_secs_f64()).map_err(|err| err.to_string())?;
        }
        if let Some(duration) = clip.duration {
            writeln!(file, "duration {:.3}", duration.as_secs_f64()).map_err(|err| err.to_string())?;
        }
    }
    Ok(path)
}

