package main

import (
	"context"
	"log"

	"stremo/smtp-service/internal/config"
	"stremo/smtp-service/internal/handlers"
	"stremo/smtp-service/internal/repository"
	"stremo/smtp-service/internal/routers"
	"stremo/smtp-service/internal/service"

	"github.com/redis/go-redis/v9"
)

func main() {
	log.Println("Starting SMTP Service (Email Gateway)...")

	cfg := config.Load()

	rdb := redis.NewClient(&redis.Options{
		Addr: cfg.RedisURL,
	})
	if err := rdb.Ping(context.Background()).Err(); err != nil {
		log.Fatalf("Unable to connect to Redis: %v\n", err)
	}
	defer rdb.Close()

	redisQueue := repository.NewRedisQueue(rdb)

	smtpService, err := service.NewSMTPService(
		service.WithRedisQueue(redisQueue),
		service.WithConfig(cfg),
	)
	if err != nil {
		log.Fatalf("Failed to initialize smtp service: %v", err)
	}

	go smtpService.StartWorker(context.Background())

	smtpHandler := handlers.NewSMTPHandler(smtpService)
	router := routers.SetupRouter(smtpHandler)

	log.Printf("Internal API listening on port %s", cfg.Port)
	if err := router.Run(":" + cfg.Port); err != nil {
		log.Fatalf("Failed to run server: %v", err)
	}
}
