package handlers

import (
	"net/http"
	"strconv"

	"stremo/vod-manager-service/internal/models"
	"stremo/vod-manager-service/internal/service"

	"github.com/gin-gonic/gin"
)

type VODHandler struct {
	svc *service.VODService
}

func NewVODHandler(svc *service.VODService) *VODHandler {
	return &VODHandler{svc: svc}
}

func (h *VODHandler) GetVODs(c *gin.Context) {
	channelID := c.Query("channel_id")
	if channelID == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "channel_id is required"})
		return
	}

	limit, _ := strconv.Atoi(c.DefaultQuery("limit", "20"))
	offset, _ := strconv.Atoi(c.DefaultQuery("offset", "0"))

	vods, err := h.svc.GetVODs(c.Request.Context(), channelID, limit, offset)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to fetch VODs"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"vods": vods})
}

func (h *VODHandler) CreateClip(c *gin.Context) {
	var req models.ClipRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request payload"})
		return
	}

	userID, exists := c.Get("user_id")
	if !exists {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Unauthorized"})
		return
	}

	err := h.svc.CreateClip(c.Request.Context(), req, userID.(string))
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusAccepted, gin.H{"message": "Clip creation task has been scheduled"})
}
