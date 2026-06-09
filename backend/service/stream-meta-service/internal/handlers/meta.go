package handlers

import (
	"net/http"
	"strconv"

	"stremo/stream-meta-service/internal/models"
	"stremo/stream-meta-service/internal/service"

	"github.com/gin-gonic/gin"
)

type MetaHandler struct {
	svc *service.MetaService
}

func NewMetaHandler(svc *service.MetaService) *MetaHandler {
	return &MetaHandler{svc: svc}
}

func (h *MetaHandler) GetLiveDirectory(c *gin.Context) {
	category := c.Query("category")
	limit, _ := strconv.Atoi(c.DefaultQuery("limit", "20"))
	offset, _ := strconv.Atoi(c.DefaultQuery("offset", "0"))

	directory, err := h.svc.GetLiveDirectory(c.Request.Context(), category, limit, offset)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch live directory"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"streams": directory})
}

func (h *MetaHandler) UpdateMeta(c *gin.Context) {
	channelID := c.Param("channel_id")

	var req models.UpdateMetaRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request payload"})
		return
	}

	if err := h.svc.UpdateMeta(c.Request.Context(), channelID, req); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update metadata"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Metadata updated successfully"})
}

func (h *MetaHandler) VerifyKey(c *gin.Context) {
	var req struct {
		StreamKey string `json:"stream_key" binding:"required"`
	}

	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "stream_key is required"})
		return
	}

	channelID, err := h.svc.VerifyKey(c.Request.Context(), req.StreamKey)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid stream key"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"channel_id": channelID})
}
