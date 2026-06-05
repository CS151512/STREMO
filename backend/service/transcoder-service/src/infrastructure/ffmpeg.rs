use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

pub struct FFmpegSupervisor {
    ffmpeg_path: String,
}

impl FFmpegSupervisor {
    pub fn new(ffmpeg_path: String) -> Self {
        Self { ffmpeg_path }
    }

    pub async fn run_abr_pipeline(&self, stream_id: &str, output_dir: &str) -> anyhow::Result<()> {
        let master_playlist_path = format!("{}/{}_master.m3u8", output_dir, stream_id);

        tracing::info!(
            "Spawning FFmpeg for ABR pipeline. Output: {}",
            master_playlist_path
        );

        let mut child = Command::new(&self.ffmpeg_path)
            .arg("-i")
            .arg("pipe:0")
            // Видео поток 1 (1080p)
            .arg("-map")
            .arg("0:v:0")
            .arg("-c:v:0")
            .arg("libx264")
            .arg("-b:v:0")
            .arg("5000k")
            .arg("-s:v:0")
            .arg("1920x1080")
            // Видео поток 2 (720p)
            .arg("-map")
            .arg("0:v:0")
            .arg("-c:v:1")
            .arg("libx264")
            .arg("-b:v:1")
            .arg("2500k")
            .arg("-s:v:1")
            .arg("1280x720")
            // Аудио поток (один для всех)
            .arg("-map")
            .arg("0:a:0?")
            .arg("-c:a")
            .arg("aac")
            .arg("-b:a")
            .arg("128k")
            // HLS настройки
            .arg("-f")
            .arg("hls")
            .arg("-hls_time")
            .arg("2")
            .arg("-hls_list_size")
            .arg("5")
            .arg("-hls_flags")
            .arg("independent_segments")
            .arg("-master_pl_name")
            .arg(format!("{}_master.m3u8", stream_id))
            .arg("-var_stream_map")
            .arg("v:0,a:0 v:1,a:0")
            .arg(format!("{}/%v/playlist.m3u8", output_dir))
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit()) // Логи FFmpeg в общий stdout
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut ffmpeg_stdin = child.stdin.take().expect("Failed to open FFmpeg stdin");

        let mut stdin = tokio::io::stdin();
        let mut buffer = vec![0; 64 * 1024];

        tracing::info!("Started piping stdin to FFmpeg...");

        loop {
            match stdin.read(&mut buffer).await {
                Ok(0) => {
                    tracing::info!("Stdin closed (EOF). Stream ended.");
                    break;
                }
                Ok(n) => {
                    if let Err(e) = ffmpeg_stdin.write_all(&buffer[..n]).await {
                        tracing::error!("Failed to write to FFmpeg: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }

        drop(ffmpeg_stdin);

        let status = child.wait().await?;
        tracing::info!("FFmpeg exited with status: {}", status);

        Ok(())
    }
}
