package models

import "time"

type VOD struct {
	ID        string    `json:"id"`
	ChannelID string    `json:"channel_id"`
	Title     string    `json:"title"`
	Duration  int       `json:"duration"`
	S3URL     string    `json:"s3_url"`
	CreatedAt time.Time `json:"created_at"`
}

type ClipRequest struct {
	VODID     string `json:"vod_id" binding:"required"`
	StartTime int    `json:"start_time" binding:"required"`
	EndTime   int    `json:"end_time" binding:"required"`
	Title     string `json:"title" binding:"required"`
}

type ClipEvent struct {
	EventID   string `json:"event_id"`
	VODID     string `json:"vod_id"`
	ChannelID string `json:"channel_id"`
	UserID    string `json:"user_id"`
	StartTime int    `json:"start_time"`
	EndTime   int    `json:"end_time"`
	Title     string `json:"title"`
}
