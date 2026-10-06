use std::path::Path;
use std::process::{Command, Stdio};

use crate::engine::{EngineConfig, VideoStandard};
use crate::playlist::Playlist;

pub fn require_tool(name: &str) -> Result<(), String> {
    let status = Command::new(name)
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|err| format!("{name} is required but could not be started: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{name} is required but returned {status}"))
    }
}

pub fn validate_playlist(config: &EngineConfig, playlist: &Playlist) -> Result<(), String> {
    for clip in &playlist.clips {
        if !clip.path.exists() {
            return Err(format!("clip {} does not exist: {}", clip.id, clip.path.display()));
        }
        let status = Command::new(&config.ffprobe_path)
            .args([
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=codec_name,width,height,r_frame_rate,field_order",
                "-of",
                "default=noprint_wrappers=1",
            ])
            .arg(&clip.path)
            .status()
            .map_err(|err| format!("failed to run ffprobe for {}: {err}", clip.path.display()))?;
        if !status.success() {
            return Err(format!("ffprobe rejected clip {} ({})", clip.id, clip.path.display()));
        }
    }
    Ok(())
}

pub fn build_ffmpeg_args(config: &EngineConfig, concat_file: &Path) -> Vec<String> {
    let mut args = vec![
        "-hide_banner".into(),
        "-nostdin".into(),
        "-re".into(),
    ];

    match config.hwaccel.as_str() {
        "none" => {}
        "auto" => args.extend(["-hwaccel".into(), "auto".into()]),
        mode => args.extend(["-hwaccel".into(), mode.into()]),
    }

    args.extend([
        "-f".into(),
        "concat".into(),
        "-safe".into(),
        "0".into(),
        "-i".into(),
        concat_file.display().to_string(),
    ]);

    args.extend(video_filter_args(config.standard));
    args.extend(output_args(config));
    args
}

fn video_filter_args(standard: VideoStandard) -> Vec<String> {
    let filter = match standard {
        VideoStandard::I1080i50 => "scale=1920:1080:flags=bicubic,fps=25,tinterlace=interleave_top",
        VideoStandard::P1080p25 => "scale=1920:1080:flags=bicubic,fps=25,format=yuv420p",
        VideoStandard::P1080p30 => "scale=1920:1080:flags=bicubic,fps=30,format=yuv420p",
    };
    vec![
        "-vf".into(),
        filter.into(),
        "-af".into(),
        "aresample=async=1:first_pts=0".into(),
    ]
}

fn output_args(config: &EngineConfig) -> Vec<String> {
    if config.output == "preview" {
        return vec![
            "-c:v".into(),
            "libx264".into(),
            "-preset".into(),
            "veryfast".into(),
            "-c:a".into(),
            "aac".into(),
            "-f".into(),
            "mpegts".into(),
            "udp://127.0.0.1:5000?pkt_size=1316".into(),
        ];
    }

    let mut args = vec![
        "-c:v".into(),
        "h264_nvenc".into(),
        "-preset".into(),
        "p4".into(),
        "-b:v".into(),
        "8M".into(),
        "-maxrate".into(),
        "8M".into(),
        "-bufsize".into(),
        "16M".into(),
        "-c:a".into(),
        "aac".into(),
        "-b:a".into(),
        "192k".into(),
    ];

    if config.output.starts_with("rtmp://") || config.output.starts_with("rtmps://") {
        args.extend(["-f".into(), "flv".into(), config.output.clone()]);
    } else if config.output.starts_with("srt://") || config.output.starts_with("udp://") {
        args.extend(["-f".into(), "mpegts".into(), config.output.clone()]);
    } else if config.output.ends_with(".m3u8") {
        args.extend([
            "-f".into(),
            "hls".into(),
            "-hls_time".into(),
            "2".into(),
            "-hls_flags".into(),
            "delete_segments+append_list".into(),
            config.output.clone(),
        ]);
    } else {
        args.push(config.output.clone());
    }
    args
}

