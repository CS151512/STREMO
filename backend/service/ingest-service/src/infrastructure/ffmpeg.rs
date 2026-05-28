use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

pub struct FFmpegRunner {
    pub ffmpeg_path: String,
    pub output_dir: String,
    pub processes: Arc<Mutex<std::collections::HashMap<String, Child>>>,
}

impl FFmpegRunner {
    pub fn new(ffmpeg_path: String, output_dir: String) -> Self {
        Self {
            ffmpeg_path,
            output_dir,
            processes: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub async fn start_transcoder(&self, stream_id: &str) -> anyhow::Result<()> {
        let output_path = format!("{}/{}.m3u8", self.output_dir, stream_id);

        tracing::info!(
            "Starting FFmpeg for stream {} to {}",
            stream_id,
            output_path
        );
        let mut child = Command::new(&self.ffmpeg_path)
            .arg("-i")
            .arg("pipe:0")
            .arg("-c:v")
            .arg("libx264")
            .arg("-preset")
            .arg("veryfast")
            .arg("-f")
            .arg("hls")
            .arg("-hls_time")
            .arg("2")
            .arg("-hls_list_size")
            .arg("5")
            .arg(&output_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut processes = self.processes.lock().await;
        processes.insert(stream_id.to_string(), child);

        Ok(())
    }

    pub async fn stop_transcoder(&self, stream_id: &str) -> anyhow::Result<()> {
        let mut processes = self.processes.lock().await;
        if let Some(mut child) = processes.remove(stream_id) {
            tracing::info!("Stopping FFmpeg for stream {}", stream_id);
            let _ = child.kill().await;
        }
        Ok(())
    }
}
