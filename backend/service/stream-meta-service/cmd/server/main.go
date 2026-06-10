package main

import (
	"context"
	"log"

	"stremo/stream-meta-service/internal/config"
	"stremo/stream-meta-service/internal/handlers"
	"stremo/stream-meta-service/internal/repository"
	"stremo/stream-meta-service/internal/routers"
	"stremo/stream-meta-service/internal/service"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/redis/go-redis/v9"
)

func main() {
	log.Println("Starting Stream Meta Service...")

	cfg := config.Load()

	dbPool, err := pgxpool.New(context.Background(), cfg.DatabaseURL)
	if err != nil {
		log.Fatalf("Unable to connect to database: %v\n", err)
	}
	defer dbPool.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: cfg.RedisURL,
	})
	if err := rdb.Ping(context.Background()).Err(); err != nil {
		log.Fatalf("Unable to connect to Redis: %v\n", err)
	}
	defer rdb.Close()

	pgRepo := repository.NewPostgresRepo(dbPool)
	redisRepo := repository.NewRedisRepo(rdb)
	metaService, err := service.NewMetaService(
		service.WithPostgres(pgRepo),
		service.WithRedis(redisRepo),
	)
	if err != nil {
		log.Fatalf("Failed to initialize meta service: %v", err)
	}

	metaHandler := handlers.NewMetaHandler(metaService)

	router := routers.SetupRouter(metaHandler, cfg.JWTSecret)

	log.Printf("Server listening on port %s", cfg.Port)
	if err := router.Run(":" + cfg.Port); err != nil {
		log.Fatalf("Failed to run server: %v", err)
	}
}
