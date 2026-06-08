package service

import (
	"context"
	"errors"

	"stremo/vod-manager-service/internal/kafka"
	"stremo/vod-manager-service/internal/models"
	"stremo/vod-manager-service/internal/repository"

	"github.com/google/uuid"
)

type VODService struct {
	db       *repository.PostgresRepo
	producer *kafka.Producer
}

type Option func(*VODService)

func WithPostgres(db *repository.PostgresRepo) Option {
	return func(s *VODService) {
		s.db = db
	}
}

func WithKafka(producer *kafka.Producer) Option {
	return func(s *VODService) {
		s.producer = producer
	}
}

func NewVODService(opts ...Option) (*VODService, error) {
	s := &VODService{}

	for _, opt := range opts {
		opt(s)
	}

	if s.db == nil {
		return nil, errors.New("postgres repository is required")
	}
	if s.producer == nil {
		return nil, errors.New("kafka producer is required")
	}

	return s, nil
}

func (s *VODService) GetVODs(ctx context.Context, channelID string, limit, offset int) ([]models.VOD, error) {
	return s.db.GetVODsByChannel(ctx, channelID, limit, offset)
}

func (s *VODService) CreateClip(ctx context.Context, req models.ClipRequest, userID string) error {
	vod, err := s.db.GetVOD(ctx, req.VODID)
	if err != nil {
		return errors.New("vod not found")
	}
	if req.StartTime < 0 || req.EndTime > vod.Duration || req.StartTime >= req.EndTime {
		return errors.New("invalid clip time range")
	}

	event := models.ClipEvent{
		EventID:   uuid.New().String(),
		VODID:     vod.ID,
		ChannelID: vod.ChannelID,
		UserID:    userID,
		StartTime: req.StartTime,
		EndTime:   req.EndTime,
		Title:     req.Title,
	}

	return s.producer.PublishClipEvent(ctx, event)
}
