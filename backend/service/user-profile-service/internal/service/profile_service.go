package service

import (
	"context"
	"errors"
	"io"
	"stremo/user-profile-service/internal/models"
	"stremo/user-profile-service/internal/repository"
)

type ProfileService struct {
	db    *repository.PostgresRepo
	cache *repository.RedisRepo
	s3    *repository.MinioRepo
}

type Option func(*ProfileService)

func WithPostgres(db *repository.PostgresRepo) Option {
	return func(s *ProfileService) {
		s.db = db
	}
}

func WithRedis(cache *repository.RedisRepo) Option {
	return func(s *ProfileService) {
		s.cache = cache
	}
}

func WithMinio(s3 *repository.MinioRepo) Option {
	return func(s *ProfileService) {
		s.s3 = s3
	}
}

func NewProfileService(opts ...Option) (*ProfileService, error) {
	s := &ProfileService{}

	for _, opt := range opts {
		opt(s)
	}

	if s.db == nil {
		return nil, errors.New("postgres repository is required")
	}
	if s.cache == nil {
		return nil, errors.New("redis repository is required")
	}
	if s.s3 == nil {
		return nil, errors.New("minio repository is required")
	}

	return s, nil
}

func (s *ProfileService) GetProfile(ctx context.Context, id string) (*models.Profile, error) {
	if p, err := s.cache.GetCache(ctx, id); err == nil && p != nil {
		return p, nil
	}

	p, err := s.db.GetProfile(ctx, id)
	if err != nil {
		return nil, err
	}

	_ = s.cache.SetCache(ctx, p)
	return p, nil
}

func (s *ProfileService) UpdateProfile(ctx context.Context, id string, req models.UpdateProfileRequest) (*models.Profile, error) {
	p, err := s.db.GetProfile(ctx, id)
	if err != nil {
		return nil, err
	}

	p.DisplayName = req.DisplayName
	p.Bio = req.Bio

	err = s.db.UpdateProfile(ctx, p)
	if err != nil {
		return nil, err
	}

	_ = s.cache.InvalidateCache(ctx, id)
	return p, nil
}

func (s *ProfileService) UploadAvatar(ctx context.Context, id string, file io.Reader, size int64, contentType string) (string, error) {
	avatarURL, err := s.s3.UploadAvatar(ctx, id, file, size, contentType)
	if err != nil {
		return "", err
	}

	p, err := s.db.GetProfile(ctx, id)
	if err != nil {
		return "", err
	}
	p.AvatarURL = avatarURL

	if err := s.db.UpdateProfile(ctx, p); err != nil {
		return "", err
	}

	_ = s.cache.InvalidateCache(ctx, id)
	return avatarURL, nil
}
