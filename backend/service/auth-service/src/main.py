import asyncio
import logging
from contextlib import asynccontextmanager
from typing import AsyncIterator

import uvicorn
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

from src import __version__
from src.core.config import get_settings
from src.core.exceptions import AppException

logger = logging.getLogger(__name__)
settings = get_settings()


# Lifespan — startup / shutdown of shared resources

@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncIterator[None]:
    """
    Initialise infrastructure clients on startup and cleanly shut them down.
    All resources are stored in app.state so routers and dependencies can
    reach them without importing globals.
    """
    from src.infrastructure.postgres.database import Database
    from src.infrastructure.redis.client import RedisClient
    from src.infrastructure.kafka.producer import KafkaProducer
    from src.infrastructure.kafka.consumer import KafkaConsumer

    logger.info("Starting %s v%s [%s]", settings.APP_NAME, __version__, settings.APP_ENV)

    # Postgres
    db = Database(dsn=str(settings.POSTGRES_DSN))
    await db.connect()
    app.state.db = db

    # Redis
    redis = RedisClient(dsn=str(settings.REDIS_DSN))
    await redis.connect()
    app.state.redis = redis

    # Kafka producer
    producer = KafkaProducer(bootstrap_servers=settings.KAFKA_BOOTSTRAP_SERVERS)
    await producer.start()
    app.state.kafka_producer = producer

    # Kafka consumer (runs as a background task)
    consumer = KafkaConsumer(
        bootstrap_servers=settings.KAFKA_BOOTSTRAP_SERVERS,
        topic=settings.KAFKA_TOPIC_USER_EVENTS,
        group_id=settings.KAFKA_CONSUMER_GROUP_ID,
    )
    consumer_task = asyncio.create_task(consumer.start(), name="kafka-consumer")
    app.state.kafka_consumer = consumer
    app.state.kafka_consumer_task = consumer_task

    logger.info("All infrastructure clients ready.")

    yield  # ── application is running ──

    logger.info("Shutting down %s…", settings.APP_NAME)

    consumer_task.cancel()
    try:
        await consumer_task
    except asyncio.CancelledError:
        pass

    await producer.stop()
    await redis.disconnect()
    await db.disconnect()

    logger.info("Shutdown complete.")


# FastAPI application factory
def create_app() -> FastAPI:
    app = FastAPI(
        title=settings.APP_NAME,
        version=__version__,
        docs_url="/docs" if not settings.is_production else None,
        redoc_url="/redoc" if not settings.is_production else None,
        lifespan=lifespan,
    )

    # ── Exception handlers ──────────────────────────────────────────────────

    @app.exception_handler(AppException)
    async def app_exception_handler(request: Request, exc: AppException) -> JSONResponse:
        return JSONResponse(
            status_code=exc.status_code,
            content={"detail": exc.detail},
        )

    # ── Routers ─────────────────────────────────────────────────────────────

    from src.api.http.routers import router as http_router
    app.include_router(http_router, prefix="/api/v1")

    # ── Health check ────────────────────────────────────────────────────────

    @app.get("/health", tags=["meta"], include_in_schema=False)
    async def health() -> dict:
        return {"status": "ok", "version": __version__}

    return app


# Entrypoint
app = create_app()


async def _serve_grpc() -> None:
    """Start the gRPC server alongside the HTTP server."""
    from src.api.grpc.server import GrpcServer
    server = GrpcServer(host=settings.GRPC_HOST, port=settings.GRPC_PORT)
    await server.start()
    logger.info("gRPC server listening on %s:%s", settings.GRPC_HOST, settings.GRPC_PORT)
    await server.wait_for_termination()


async def _serve_http() -> None:
    config = uvicorn.Config(
        app=app,
        host=settings.HTTP_HOST,
        port=settings.HTTP_PORT,
        log_level="debug" if settings.DEBUG else "info",
        loop="none",  # we manage the event loop ourselves
    )
    server = uvicorn.Server(config)
    await server.serve()


async def main() -> None:
    logging.basicConfig(
        level=logging.DEBUG if settings.DEBUG else logging.INFO,
        format="%(asctime)s  %(levelname)-8s  %(name)s — %(message)s",
    )
    await asyncio.gather(_serve_http(), _serve_grpc())


if __name__ == "__main__":
    asyncio.run(main())