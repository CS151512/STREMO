package routers

import (
	"stremo/vod-manager-service/internal/handlers"
	"stremo/vod-manager-service/internal/middleware"

	"github.com/gin-gonic/gin"
)

func SetupRouter(vodHandler *handlers.VODHandler, jwtSecret string) *gin.Engine {
	r := gin.Default()

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
