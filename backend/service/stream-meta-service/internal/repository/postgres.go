package repository

import (
	"context"
	"stremo/stream-meta-service/internal/models"

	"github.com/jackc/pgx/v5/pgxpool"
)

type PostgresRepo struct {
	db *pgxpool.Pool
}

func NewPostgresRepo(db *pgxpool.Pool) *PostgresRepo {
	return &PostgresRepo{db: db}
}

func (r *PostgresRepo) GetMetaByChannel(ctx context.Context, channelID string) (*models.StreamMeta, error) {
	query := `
		SELECT id, channel_id, stream_key, title, category, tags, is_live, started_at, thumbnail_url
		FROM stream_metadata
		WHERE channel_id = $1
	`
	var m models.StreamMeta
	err := r.db.QueryRow(ctx, query, channelID).Scan(
		&m.ID, &m.ChannelID, &m.StreamKey, &m.Title,
		&m.Category, &m.Tags, &m.IsLive, &m.StartedAt, &m.ThumbnailURL,
	)
	if err != nil {
		return nil, err
	}
	return &m, nil
}

func (r *PostgresRepo) GetLiveStreams(ctx context.Context,
	category string, limit, offset int) ([]models.StreamMeta, error) {
	var query string
	var args []interface{}

	if category != "" {
		query = `
			SELECT id, channel_id, title, category, tags, is_live, started_at, thumbnail_url
			FROM stream_metadata
			WHERE is_live = true AND category = $1
			ORDER BY started_at DESC
			LIMIT $2 OFFSET $3
		`
		args = []interface{}{category, limit, offset}
	} else {
		query = `
			SELECT id, channel_id, title, category, tags, is_live, started_at, thumbnail_url
			FROM stream_metadata
			WHERE is_live = true
			ORDER BY started_at DESC
			LIMIT $1 OFFSET $2
		`
		args = []interface{}{limit, offset}
	}

	rows, err := r.db.Query(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var streams []models.StreamMeta
	for rows.Next() {
		var m models.StreamMeta
		if err := rows.Scan(&m.ID, &m.ChannelID,
			&m.Title, &m.Category, &m.Tags, &m.IsLive, &m.StartedAt, &m.ThumbnailURL); err != nil {
			return nil, err
		}
		streams = append(streams, m)
	}
	return streams, nil
}

func (r *PostgresRepo) UpdateMeta(ctx context.Context, channelID string, req models.UpdateMetaRequest) error {
	query := `
		UPDATE stream_metadata
		SET title = $1, category = $2, tags = $3, updated_at = NOW()
		WHERE channel_id = $4
	`
	_, err := r.db.Exec(ctx, query,
		req.Title, req.Category, req.Tags, channelID)
	return err
}

func (r *PostgresRepo) VerifyStreamKey(ctx context.Context, streamKey string) (*models.StreamMeta, error) {
	query := `
		SELECT channel_id
		FROM stream_metadata
		WHERE stream_key = $1
	`
	var m models.StreamMeta
	err := r.db.QueryRow(ctx, query, streamKey).Scan(&m.ChannelID)
	if err != nil {
		return nil, err
	}
	return &m, nil
}
