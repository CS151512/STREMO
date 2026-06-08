package main

import (
	"context"
	"log"

	"stremo/vod-manager-service/internal/config"
	"stremo/vod-manager-service/internal/handlers"
	"stremo/vod-manager-service/internal/kafka"
	"stremo/vod-manager-service/internal/repository"
	"stremo/vod-manager-service/internal/routers"
	"stremo/vod-manager-service/internal/service"

	"github.com/jackc/pgx/v5/pgxpool"
)

func main() {
	log.Println("Starting VOD Manager Service...")

	cfg := config.Load()
	dbPool, err := pgxpool.New(context.Background(), cfg.DatabaseURL)
	if err != nil {
		log.Fatalf("Unable to connect to database: %v\n", err)
	}
	defer dbPool.Close()

	producer := kafka.NewProducer(cfg.KafkaBrokers, "stream.clips.requested")
	defer producer.Close()

	pgRepo := repository.NewPostgresRepo(dbPool)

	vodService, err := service.NewVODService(
		service.WithPostgres(pgRepo),
		service.WithKafka(producer),
	)
	if err != nil {
		log.Fatalf("Failed to initialize vod service: %v", err)
	}

	vodHandler := handlers.NewVODHandler(vodService)
	router := routers.SetupRouter(vodHandler, cfg.JWTSecret)
	log.Printf("Server listening on port %s", cfg.Port)
	if err := router.Run(":" + cfg.Port); err != nil {
		log.Fatalf("Failed to run server: %v", err)
	}
}
