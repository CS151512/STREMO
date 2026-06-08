package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	Port         string
	DatabaseURL  string
	KafkaBrokers []string
	JWTSecret    string
}

func Load() *Config {
	_ = godotenv.Load()

	brokers := getEnv("KAFKA_BROKERS", "localhost:19092")

	return &Config{
		Port:         getEnv("PORT", "8084"),
		DatabaseURL:  getEnv("DATABASE_URL", "postgres://itter_admin:secret_password@localhost:5432/itterstream"),
		KafkaBrokers: []string{brokers},
		JWTSecret:    getEnv("JWT_SECRET", "super_secret_key_for_development"),
	}
}

func getEnv(key, fallback string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	log.Printf("Environment variable %s not set. Using fallback: %s", key, fallback)
	return fallback
}
