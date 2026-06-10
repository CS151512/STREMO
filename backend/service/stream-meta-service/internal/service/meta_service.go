package service

import (
	"context"
	"errors"

	"stremo/stream-meta-service/internal/models"
	"stremo/stream-meta-service/internal/repository"
)

type MetaService struct {
	db    *repository.PostgresRepo
	cache *repository.RedisRepo
}

type Option func(*MetaService)

func WithPostgres(db *repository.PostgresRepo) Option {
	return func(s *MetaService) {
		s.db = db
	}
}

func WithRedis(cache *repository.RedisRepo) Option {
	return func(s *MetaService) {
		s.cache = cache
	}
}

func NewMetaService(opts ...Option) (*MetaService, error) {
	s := &MetaService{}

	for _, opt := range opts {
		opt(s)
	}

	if s.db == nil {
		return nil, errors.New("postgres repository is required")
	}
	if s.cache == nil {
		return nil, errors.New("redis repository is required")
	}

	return s, nil
}

func (s *MetaService) GetLiveDirectory(ctx context.Context,
	category string, limit, offset int) ([]models.LiveStreamDTO, error) {
	streams, err := s.db.GetLiveStreams(ctx, category, limit, offset)
	if err != nil {
		return nil, err
	}

	var directory []models.LiveStreamDTO
	for _, st := range streams {
		viewers, _ := s.cache.GetLiveViewerCount(ctx, st.ChannelID)

		directory = append(directory, models.LiveStreamDTO{
			ChannelID:    st.ChannelID,
			Title:        st.Title,
			Category:     st.Category,
			Tags:         st.Tags,
			ViewerCount:  viewers,
			ThumbnailURL: st.ThumbnailURL,
		})
	}

	return directory, nil
}

func (s *MetaService) UpdateMeta(ctx context.Context, channelID string, req models.UpdateMetaRequest) error {
	return s.db.UpdateMeta(ctx, channelID, req)
}

func (s *MetaService) VerifyKey(ctx context.Context, streamKey string) (string, error) {
	meta, err := s.db.VerifyStreamKey(ctx, streamKey)
	if err != nil {
		return "", err
	}
	return meta.ChannelID, nil
}
