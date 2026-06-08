package kafka

import (
	"context"
	"encoding/json"
	"log"

	"stremo/vod-manager-service/internal/models"

	"github.com/segmentio/kafka-go"
)

type Producer struct {
	writer *kafka.Writer
}

func NewProducer(brokers []string, topic string) *Producer {
	w := &kafka.Writer{
		Addr:     kafka.TCP(brokers...),
		Topic:    topic,
		Balancer: &kafka.LeastBytes{},
	}
	return &Producer{writer: w}
}

func (p *Producer) PublishClipEvent(ctx context.Context, event models.ClipEvent) error {
	payload, err := json.Marshal(event)
	if err != nil {
		return err
	}

	err = p.writer.WriteMessages(ctx,
		kafka.Message{
			Key:   []byte(event.ChannelID),
			Value: payload,
		},
	)
	if err != nil {
		log.Printf("Failed to publish message to Kafka: %v", err)
		return err
	}

	return nil
}

func (p *Producer) Close() error {
	return p.writer.Close()
}
