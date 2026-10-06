use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Clip {
    pub id: String,
    pub path: PathBuf,
    pub in_point: Duration,
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug, Default)]
pub struct Playlist {
    pub clips: Vec<Clip>,
}

impl Playlist {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|err| format!("failed to read playlist {}: {err}", path.display()))?;
        Self::from_csv(&text, path.parent().unwrap_or_else(|| Path::new(".")))
    }

    pub fn from_csv(text: &str, base_dir: &Path) -> Result<Self, String> {
        let mut clips = Vec::new();
        for (line_no, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line_no == 0 && line.to_ascii_lowercase().starts_with("id,") {
                continue;
            }
            let fields = parse_csv_line(line);
            if fields.len() < 2 {
                return Err(format!("playlist line {} needs at least id,path", line_no + 1));
            }
            let path = PathBuf::from(&fields[1]);
            clips.push(Clip {
                id: fields[0].clone(),
                path: if path.is_absolute() { path } else { base_dir.join(path) },
                in_point: parse_duration(fields.get(2).map(String::as_str).unwrap_or("0"))?,
                duration: match fields.get(3).map(String::as_str).unwrap_or("").trim() {
                    "" => None,
                    value => Some(parse_duration(value)?),
                },
            });
        }
        if clips.is_empty() {
            return Err("playlist contains no clips".to_string());
        }
        Ok(Self { clips })
    }

    pub fn to_json(&self) -> String {
        let items = self
            .clips
            .iter()
            .map(|clip| {
                format!(
                    "{{\"id\":\"{}\",\"path\":\"{}\",\"inPointMs\":{},\"durationMs\":{}}}",
                    json_escape(&clip.id),
                    json_escape(&clip.path.display().to_string()),
                    clip.in_point.as_millis(),
                    clip.duration
                        .map(|d| d.as_millis().to_string())
                        .unwrap_or_else(|| "null".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("[{items}]")
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    for ch in line.chars() {
        match ch {
            '"' => quoted = !quoted,
            ',' if !quoted => {
                out.push(field.trim().trim_matches('"').to_string());
                field.clear();
            }
            _ => field.push(ch),
        }
    }
    out.push(field.trim().trim_matches('"').to_string());
    out
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(Duration::ZERO);
    }
    if let Ok(seconds) = value.parse::<f64>() {
        return Ok(Duration::from_secs_f64(seconds));
    }
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() == 3 {
        let hours = parts[0].parse::<u64>().map_err(|_| format!("bad duration {value}"))?;
        let minutes = parts[1].parse::<u64>().map_err(|_| format!("bad duration {value}"))?;
        let seconds = parts[2].parse::<f64>().map_err(|_| format!("bad duration {value}"))?;
        return Ok(Duration::from_secs(hours * 3600 + minutes * 60) + Duration::from_secs_f64(seconds));
    }
    Err(format!("bad duration {value}; use seconds or HH:MM:SS.mmm"))
}

pub fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_playlist_csv() {
        let playlist = Playlist::from_csv(
            "id,path,in,duration\nA,a.mp4,00:00:01.5,30\nB,\"folder,b.mxf\",0,\n",
            Path::new("C:/media"),
        )
        .unwrap();
        assert_eq!(playlist.clips.len(), 2);
        assert_eq!(playlist.clips[0].in_point, Duration::from_millis(1500));
        assert_eq!(playlist.clips[0].duration, Some(Duration::from_secs(30)));
        assert!(playlist.clips[1].path.ends_with("folder,b.mxf"));
    }
}

