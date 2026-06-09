package routers

import (
	"stremo/vod-manager-service/internal/handlers"
	"stremo/vod-manager-service/internal/middleware"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func SetupRouter(vodHandler *handlers.VODHandler, jwtSecret string) *gin.Engine {
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
		api.GET("/vods", vodHandler.GetVODs)
		protected := api.Group("")
		protected.Use(middleware.RequireAuth(jwtSecret))
		{
			protected.POST("/clips", vodHandler.CreateClip)
		}
	}

	return r
}
