package repository

import (
	"context"
	"encoding/json"
	"stremo/user-profile-service/internal/models"
	"time"

	"github.com/redis/go-redis/v9"
)

type RedisRepo struct {
	client *redis.Client
	ttl    time.Duration
}

func NewRedisRepo(client *redis.Client, ttl time.Duration) *RedisRepo {
	return &RedisRepo{
		client: client,
		ttl:    ttl,
	}
}

func (r *RedisRepo) GetCache(ctx context.Context, id string) (*models.Profile, error) {
	val, err := r.client.Get(ctx, "profile:"+id).Result()
	if err != nil {
		return nil, err
	}

	var p models.Profile
	if err := json.Unmarshal([]byte(val), &p); err != nil {
		return nil, err
	}
	return &p, nil
}

func (r *RedisRepo) SetCache(ctx context.Context, p *models.Profile) error {
	data, err := json.Marshal(p)
	if err != nil {
		return err
	}
	return r.client.Set(ctx, "profile:"+p.ID, data, r.ttl).Err()
}

func (r *RedisRepo) InvalidateCache(ctx context.Context, id string) error {
	return r.client.Del(ctx, "profile:"+id).Err()
}
