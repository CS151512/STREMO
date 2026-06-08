package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	Port      string
	RedisURL  string
	SMTPHost  string
	SMTPPort  string
	SMTPUser  string
	SMTPPass  string
	FromEmail string
}

func Load() *Config {
	_ = godotenv.Load()

	return &Config{
		Port:      getEnv("PORT", "8085"),
		RedisURL:  getEnv("REDIS_URL", "redis://localhost:6379/0"),
		SMTPHost:  getEnv("SMTP_HOST", "smtp.mailgun.org"),
		SMTPPort:  getEnv("SMTP_PORT", "587"),
		SMTPUser:  getEnv("SMTP_USER", "postmaster@yourdomain.com"),
		SMTPPass:  getEnv("SMTP_PASS", "your-smtp-password"),
		FromEmail: getEnv("FROM_EMAIL", "noreply@stremo.com"),
	}
}

func getEnv(key, fallback string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	log.Printf("Environment variable %s not set. Using fallback: %s", key, fallback)
	return fallback
}
