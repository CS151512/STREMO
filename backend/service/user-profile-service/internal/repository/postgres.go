package repository

import (
	"context"
	"stremo/user-profile-service/internal/models"

	"github.com/jackc/pgx/v5/pgxpool"
)

type PostgresRepo struct {
	db *pgxpool.Pool
}

func NewPostgresRepo(db *pgxpool.Pool) *PostgresRepo {
	return &PostgresRepo{db: db}
}

func (r *PostgresRepo) GetProfile(ctx context.Context, id string) (*models.Profile, error) {
	query := `
		SELECT id, username, display_name, bio, avatar_url, followers, created_at, updated_at
		FROM user_profiles
		WHERE id = $1
	`
	var p models.Profile
	err := r.db.QueryRow(ctx, query, id).Scan(
		&p.ID, &p.Username, &p.DisplayName, &p.Bio, &p.AvatarURL, &p.Followers, &p.CreatedAt, &p.UpdatedAt,
	)
	if err != nil {
		return nil, err
	}
	return &p, nil
}

func (r *PostgresRepo) UpdateProfile(ctx context.Context, p *models.Profile) error {
	query := `
		UPDATE user_profiles
		SET display_name = $1, bio = $2, avatar_url = $3, updated_at = NOW()
		WHERE id = $4
	`
	_, err := r.db.Exec(ctx, query, p.DisplayName, p.Bio, p.AvatarURL, p.ID)
	return err
}
