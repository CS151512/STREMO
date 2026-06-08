package repository

import (
	"context"
	"stremo/vod-manager-service/internal/models"

	"github.com/jackc/pgx/v5/pgxpool"
)

type PostgresRepo struct {
	db *pgxpool.Pool
}

func NewPostgresRepo(db *pgxpool.Pool) *PostgresRepo {
	return &PostgresRepo{db: db}
}

func (r *PostgresRepo) SaveVOD(ctx context.Context, vod *models.VOD) error {
	query := `
		INSERT INTO vods (id, channel_id, title, duration, s3_url, created_at)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	_, err := r.db.Exec(ctx, query, vod.ID, vod.ChannelID, vod.Title, vod.Duration, vod.S3URL, vod.CreatedAt)
	return err
}

func (r *PostgresRepo) GetVODsByChannel(ctx context.Context, channelID string, limit, offset int) ([]models.VOD, error) {
	query := `
		SELECT id, channel_id, title, duration, s3_url, created_at
		FROM vods
		WHERE channel_id = $1
		ORDER BY created_at DESC
		LIMIT $2 OFFSET $3
	`
	rows, err := r.db.Query(ctx, query, channelID, limit, offset)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var vods []models.VOD
	for rows.Next() {
		var v models.VOD
		if err := rows.Scan(&v.ID, &v.ChannelID, &v.Title, &v.Duration, &v.S3URL, &v.CreatedAt); err != nil {
			return nil, err
		}
		vods = append(vods, v)
	}
	return vods, nil
}

func (r *PostgresRepo) GetVOD(ctx context.Context, id string) (*models.VOD, error) {
	query := `
		SELECT id, channel_id, title, duration, s3_url, created_at
		FROM vods
		WHERE id = $1
	`
	var v models.VOD
	err := r.db.QueryRow(ctx, query, id).Scan(&v.ID, &v.ChannelID, &v.Title, &v.Duration, &v.S3URL, &v.CreatedAt)
	if err != nil {
		return nil, err
	}
	return &v, nil
}
