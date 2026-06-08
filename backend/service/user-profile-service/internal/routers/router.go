package routers

import (
	"stremo/user-profile-service/internal/handlers"
	"stremo/user-profile-service/internal/middleware"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func SetupRouter(profileHandler *handlers.ProfileHandler, jwtSecret string) *gin.Engine {
	r := gin.Default()

	r.Use(cors.New(cors.Config{
		AllowAllOrigins:  true,
		AllowMethods:     []string{"GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Length", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))

	api := r.Group("/api/v1/profiles")
	{
		api.GET("/:id", profileHandler.GetProfile)

		protected := api.Group("")
		protected.Use(middleware.RequireAuth(jwtSecret), middleware.RequireOwner())
		{
			protected.PUT("/:id", profileHandler.UpdateProfile)
			protected.POST("/:id/avatar", profileHandler.UploadAvatar)
		}
	}

	return r
}
