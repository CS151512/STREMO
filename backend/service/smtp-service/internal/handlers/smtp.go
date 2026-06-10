package handlers

import (
	"net/http"

	"stremo/smtp-service/internal/models"
	"stremo/smtp-service/internal/service"

	"github.com/gin-gonic/gin"
)

type SMTPHandler struct {
	svc *service.SMTPService
}

func NewSMTPHandler(svc *service.SMTPService) *SMTPHandler {
	return &SMTPHandler{svc: svc}
}

func (h *SMTPHandler) EnqueueMail(c *gin.Context) {
	var req models.EmailTask

	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request payload or missing required fields"})
		return
	}

	if err := h.svc.EnqueueEmail(c.Request.Context(), req); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to enqueue email task"})
		return
	}

	c.JSON(http.StatusAccepted, gin.H{"message": "Email task has been enqueued in Redis"})
}
