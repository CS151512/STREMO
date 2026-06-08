package handlers

import (
	"net/http"
	"stremo/user-profile-service/internal/models"
	"stremo/user-profile-service/internal/service"

	"github.com/gin-gonic/gin"
)

type ProfileHandler struct {
	svc *service.ProfileService
}

func NewProfileHandler(svc *service.ProfileService) *ProfileHandler {
	return &ProfileHandler{svc: svc}
}

func (h *ProfileHandler) GetProfile(c *gin.Context) {
	id := c.Param("id")

	profile, err := h.svc.GetProfile(c.Request.Context(), id)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Profile not found"})
		return
	}

	c.JSON(http.StatusOK, profile)
}

func (h *ProfileHandler) UpdateProfile(c *gin.Context) {
	id := c.Param("id")

	var req models.UpdateProfileRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request payload"})
		return
	}

	profile, err := h.svc.UpdateProfile(c.Request.Context(), id, req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, profile)
}

func (h *ProfileHandler) UploadAvatar(c *gin.Context) {
	id := c.Param("id")

	file, header, err := c.Request.FormFile("avatar")
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Failed to parse form file"})
		return
	}
	defer file.Close()

	contentType := header.Header.Get("Content-Type")

	avatarURL, err := h.svc.UploadAvatar(c.Request.Context(), id, file, header.Size, contentType)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to upload avatar: " + err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"avatar_url": avatarURL})
}
