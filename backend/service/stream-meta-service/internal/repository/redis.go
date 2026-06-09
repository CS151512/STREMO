package repository

import (
	"context"
	"fmt"
	"strconv"

	"github.com/redis/go-redis/v9"
)

type RedisRepo struct {
	client *redis.Client
}

func NewRedisRepo(client *redis.Client) *RedisRepo {
	return &RedisRepo{client: client}
}

func (r *RedisRepo) GetLiveViewerCount(ctx context.Context, channelID string) (int, error) {
	key := fmt.Sprintf("live_ccv:%s", channelID)
	val, err := r.client.Get(ctx, key).Result()

	if err == redis.Nil {
		return 0, nil
	} else if err != nil {
		return 0, err
	}

	count, _ := strconv.Atoi(val)
	return count, nil
}
