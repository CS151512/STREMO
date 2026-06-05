mod config;
mod infrastructure;

use clap::Parser;
use config::CliArgs;
use infrastructure::{ffmpeg::FFmpegSupervisor, webhook::WebhookClient};
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Transcoder Worker Process...");

    let args = CliArgs::parse();
    tracing::info!("Stream ID: {}", args.stream_id);
    tracing::info!("Output Dir: {}", args.output_dir);

    fs::create_dir_all(format!("{}/0", args.output_dir)).ok();
    fs::create_dir_all(format!("{}/1", args.output_dir)).ok();

    let ffmpeg = FFmpegSupervisor::new(args.ffmpeg_path);
    let webhook_client = WebhookClient::new(args.vod_url);

    match ffmpeg.run_abr_pipeline(&args.stream_id, &args.output_dir).await {
        Ok(_) => {
            tracing::info!("Transcoding completed successfully.");

            let master_pl = format!("{}/{}_master.m3u8", args.output_dir, args.stream_id);
            if let Err(e) = webhook_client.notify_vod_ready(&args.stream_id, &master_pl).await {
                tracing::error!("Failed to notify VOD Manager: {}", e);
            }
        }
        Err(e) => {
            tracing::error!("Transcoder pipeline failed: {}", e);
            std::process::exit(1);
        }
    }

    tracing::info!("Transcoder Worker Process shutting down.");
    Ok(())
}
