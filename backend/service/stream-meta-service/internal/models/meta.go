package models

import "time"

type StreamMeta struct {
	ID           string     `json:"id"`
	ChannelID    string     `json:"channel_id"`
	StreamKey    string     `json:"-"`
	Title        string     `json:"title"`
	Category     string     `json:"category"`
	Tags         []string   `json:"tags"`
	IsLive       bool       `json:"is_live"`
	StartedAt    *time.Time `json:"started_at,omitempty"`
	ThumbnailURL string     `json:"thumbnail_url"`
}

type LiveStreamDTO struct {
	ChannelID    string   `json:"channel_id"`
	Title        string   `json:"title"`
	Category     string   `json:"category"`
	Tags         []string `json:"tags"`
	ViewerCount  int      `json:"viewer_count"`
	ThumbnailURL string   `json:"thumbnail_url"`
}

type UpdateMetaRequest struct {
	Title    string   `json:"title" binding:"required"`
	Category string   `json:"category" binding:"required"`
	Tags     []string `json:"tags"`
}
