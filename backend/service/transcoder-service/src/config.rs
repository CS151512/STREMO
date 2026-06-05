use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long)]
    pub stream_id: String,

    #[arg(long, default_value = "/tmp/hls")]
    pub output_dir: String,

    #[arg(long, default_value = "ffmpeg")]
    pub ffmpeg_path: String,

    #[arg(long, default_value = "http://vod-manager-service:8080")]
    pub vod_url: String,
}
