package service

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"html/template"
	"log"
	"net/smtp"

	"stremo/smtp-service/internal/config"
	"stremo/smtp-service/internal/models"
	"stremo/smtp-service/internal/repository"
)

type SMTPService struct {
	queue *repository.RedisQueue
	cfg   *config.Config
}

type Option func(*SMTPService)

func WithRedisQueue(queue *repository.RedisQueue) Option {
	return func(s *SMTPService) {
		s.queue = queue
	}
}

func WithConfig(cfg *config.Config) Option {
	return func(s *SMTPService) {
		s.cfg = cfg
	}
}

func NewSMTPService(opts ...Option) (*SMTPService, error) {
	s := &SMTPService{}

	for _, opt := range opts {
		opt(s)
	}

	if s.queue == nil {
		return nil, errors.New("redis queue is required")
	}
	if s.cfg == nil {
		return nil, errors.New("config is required")
	}

	return s, nil
}

func (s *SMTPService) EnqueueEmail(ctx context.Context, task models.EmailTask) error {
	return s.queue.Enqueue(ctx, task)
}

func (s *SMTPService) StartWorker(ctx context.Context) {
	log.Println("Starting background Email Worker...")

	for {
		select {
		case <-ctx.Done():
			log.Println("Stopping Email Worker...")
			return
		default:
			task, err := s.queue.Dequeue(ctx)
			if err != nil {
				log.Printf("Error dequeueing email task: %v", err)
				continue
			}

			if err := s.processAndSendEmail(task); err != nil {
				log.Printf("Failed to send email to %s: %v", task.To, err)
			} else {
				log.Printf("Successfully sent email '%s' to %s", task.Subject, task.To)
			}
		}
	}
}

func (s *SMTPService) processAndSendEmail(task *models.EmailTask) error {
	tmplPath := fmt.Sprintf("templates/%s.html", task.TemplateName)
	tmpl, err := template.ParseFiles(tmplPath)
	if err != nil {
		return fmt.Errorf("failed to parse template: %w", err)
	}

	var body bytes.Buffer
	if err := tmpl.Execute(&body, task.TemplateData); err != nil {
		return fmt.Errorf("failed to execute template: %w", err)
	}

	msg := []byte(fmt.Sprintf(
		"To: %s\r\n"+
			"From: %s\r\n"+
			"Subject: %s\r\n"+
			"MIME-version: 1.0;\r\n"+
			"Content-Type: text/html; charset=\"UTF-8\";\r\n\r\n"+
			"%s",
		task.To, s.cfg.FromEmail, task.Subject, body.String(),
	))

	auth := smtp.PlainAuth("", s.cfg.SMTPUser, s.cfg.SMTPPass, s.cfg.SMTPHost)
	addr := fmt.Sprintf("%s:%s", s.cfg.SMTPHost, s.cfg.SMTPPort)

	err = smtp.SendMail(addr, auth, s.cfg.FromEmail, []string{task.To}, msg)
	if err != nil {
		return fmt.Errorf("smtp.SendMail failed: %w", err)
	}

	return nil
}
