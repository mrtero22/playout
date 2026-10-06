mod control;
mod engine;
mod ffmpeg;
mod playlist;
mod timecode;

use std::env;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};

use control::run_http_server;
use engine::{EngineConfig, PlayoutEngine, VideoStandard};
use playlist::Playlist;

fn main() {
    if let Err(err) = run() {
        eprintln!("openplayout-core-engine: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args)?;

    ffmpeg::require_tool("ffmpeg")?;
    ffmpeg::require_tool("ffprobe")?;

    let playlist = if let Some(path) = &config.playlist_path {
        Some(Playlist::from_file(path)?)
    } else {
        None
    };

    let engine = Arc::new(Mutex::new(PlayoutEngine::new(config.engine_config, playlist)));
    let bind = config.bind.clone();
    println!("OpenPlayout core engine listening on http://{bind}");
    println!("Output target: {}", config.output);
    run_http_server(&bind, engine)
}

struct CliConfig {
    bind: String,
    output: String,
    playlist_path: Option<PathBuf>,
    engine_config: EngineConfig,
}

fn parse_args(args: &[String]) -> Result<CliConfig, String> {
    let mut bind = "127.0.0.1:8080".to_string();
    let mut output = "preview".to_string();
    let mut playlist_path = None;
    let mut standard = VideoStandard::P1080p25;
    let mut hwaccel = "auto".to_string();
    let mut ffmpeg_path = "ffmpeg".to_string();
    let mut ffprobe_path = "ffprobe".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--bind" => {
                i += 1;
                bind = read_value(args, i, "--bind")?;
            }
            "--playlist" => {
                i += 1;
                playlist_path = Some(PathBuf::from(read_value(args, i, "--playlist")?));
            }
            "--output" => {
                i += 1;
                output = read_value(args, i, "--output")?;
            }
            "--standard" => {
                i += 1;
                standard = read_value(args, i, "--standard")?.parse()?;
            }
            "--hwaccel" => {
                i += 1;
                hwaccel = read_value(args, i, "--hwaccel")?;
            }
            "--ffmpeg" => {
                i += 1;
                ffmpeg_path = read_value(args, i, "--ffmpeg")?;
            }
            "--ffprobe" => {
                i += 1;
                ffprobe_path = read_value(args, i, "--ffprobe")?;
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        i += 1;
    }

    Ok(CliConfig {
        bind,
        output: output.clone(),
        playlist_path,
        engine_config: EngineConfig {
            output,
            standard,
            hwaccel,
            ffmpeg_path,
            ffprobe_path,
        },
    })
}

fn read_value(args: &[String], index: usize, flag: &str) -> Result<String, String> {
    args.get(index)
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    println!(
        "OpenPlayout Core Engine\n\n\
         Usage:\n  openplayout-core-engine --playlist playlist.csv --output srt://127.0.0.1:9000 --standard 1080p25\n\n\
         Options:\n\
           --bind HOST:PORT          HTTP control address (default 127.0.0.1:8080)\n\
           --playlist PATH           CSV playlist loaded at startup\n\
           --output TARGET           preview, udp://..., srt://..., rtmp://..., or an ffmpeg URL\n\
           --standard STANDARD       1080i50, 1080p25, or 1080p30\n\
           --hwaccel MODE            auto, cuda, dxva2, d3d11va, qsv, vaapi, videotoolbox, none\n\
           --ffmpeg PATH             ffmpeg executable path\n\
           --ffprobe PATH            ffprobe executable path\n"
    );
}

