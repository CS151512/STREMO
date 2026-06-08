package repository

import (
	"context"
	"encoding/json"

	"stremo/smtp-service/internal/models"

	"github.com/redis/go-redis/v9"
)

const QueueName = "email_queue"

type RedisQueue struct {
	client *redis.Client
}

func NewRedisQueue(client *redis.Client) *RedisQueue {
	return &RedisQueue{client: client}
}

func (q *RedisQueue) Enqueue(ctx context.Context, task models.EmailTask) error {
	payload, err := json.Marshal(task)
	if err != nil {
		return err
	}
	return q.client.LPush(ctx, QueueName, payload).Err()
}

func (q *RedisQueue) Dequeue(ctx context.Context) (*models.EmailTask, error) {
	result, err := q.client.BRPop(ctx, 0, QueueName).Result()
	if err != nil {
		return nil, err
	}

	var task models.EmailTask
	if err := json.Unmarshal([]byte(result[1]), &task); err != nil {
		return nil, err
	}

	return &task, nil
}

func (q *RedisQueue) Ping(ctx context.Context) error {
	return q.client.Ping(ctx).Err()
}
