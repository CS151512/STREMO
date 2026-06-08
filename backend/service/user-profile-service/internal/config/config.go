package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	Port        string
	DatabaseURL string
	RedisURL    string
	MinioURL    string
	MinioUser   string
	MinioPass   string
	MinioBucket string
	JWTSecret   string
}

func Load() *Config {
	_ = godotenv.Load()

	return &Config{
		Port:        getEnv("PORT", "8082"),
		DatabaseURL: getEnv("DATABASE_URL", "postgres://itter_admin:secret_password@localhost:5432/itterstream"),
		RedisURL:    getEnv("REDIS_URL", "redis://localhost:6379/0"),
		MinioURL:    getEnv("MINIO_URL", "localhost:9000"),
		MinioUser:   getEnv("MINIO_ROOT_USER", "minioadmin"),
		MinioPass:   getEnv("MINIO_ROOT_PASSWORD", "minioadmin"),
		MinioBucket: getEnv("MINIO_BUCKET_AVATARS", "avatars"),
		JWTSecret:   getEnv("JWT_SECRET", "super_secret_key_for_development"),
	}
}

func getEnv(key, fallback string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	log.Printf("Environment variable %s not set. Using fallback: %s", key, fallback)
	return fallback
}
