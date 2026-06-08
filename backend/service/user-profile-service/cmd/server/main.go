package main

import (
	"context"
	"log"
	"time"

	"stremo/user-profile-service/internal/config"
	"stremo/user-profile-service/internal/handlers"
	"stremo/user-profile-service/internal/repository"
	"stremo/user-profile-service/internal/routers"
	"stremo/user-profile-service/internal/service"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	"github.com/redis/go-redis/v9"
)

func main() {
	log.Println("Starting User Profile Service...")

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

	minioClient, err := minio.New(cfg.MinioURL, &minio.Options{
		Creds:  credentials.NewStaticV4(cfg.MinioUser, cfg.MinioPass, ""),
		Secure: false, // В проде true,но это явно не прод пока :)))))))))))))
	})
	if err != nil {
		log.Fatalf("Unable to connect to MinIO: %v\n", err)
	}

	pgRepo := repository.NewPostgresRepo(dbPool)
	redisRepo := repository.NewRedisRepo(rdb, 15*time.Minute)
	minioRepo := repository.NewMinioRepo(minioClient, cfg.MinioBucket)

	profileService, err := service.NewProfileService(
		service.WithPostgres(pgRepo),
		service.WithRedis(redisRepo),
		service.WithMinio(minioRepo),
	)
	if err != nil {
		log.Fatalf("Failed to initialize profile service: %v", err)
	}

	profileHandler := handlers.NewProfileHandler(profileService)
	router := routers.SetupRouter(profileHandler, cfg.JWTSecret)

	log.Printf("Server listening on port %s", cfg.Port)
	if err := router.Run(":" + cfg.Port); err != nil {
		log.Fatalf("Failed to run server: %v", err)
	}
}
