package routers

import (
	"time"

	"stremo/stream-meta-service/internal/handlers"
	"stremo/stream-meta-service/internal/middleware"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func SetupRouter(metaHandler *handlers.MetaHandler, jwtSecret string) *gin.Engine {
	r := gin.Default()

	r.Use(cors.New(cors.Config{
		AllowAllOrigins:  true,
		AllowMethods:     []string{"GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Length", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))

	api := r.Group("/api/v1")
	{
		api.GET("/streams/live", metaHandler.GetLiveDirectory)
		protected := api.Group("")
		protected.Use(middleware.RequireAuth(jwtSecret), middleware.RequireOwner())
		{
			protected.PUT("/streams/meta/:channel_id", metaHandler.UpdateMeta)
		}
	}
	internal := r.Group("/internal/v1")
	{
		internal.POST("/verify-key", metaHandler.VerifyKey)
	}

	return r
}
