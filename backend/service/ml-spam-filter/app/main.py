import logging
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.api.routes import get_classifier, router
from app.core.config import settings
from app.services.classifier import SpamClassifier

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
)
logger = logging.getLogger(__name__)

classifier_instance = SpamClassifier(
    model_name=settings.model_name, threshold=settings.spam_threshold
)


@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("Starting up ML Spam Filter service...")
    classifier_instance.load_model()
    yield
    logger.info("Shutting down ML Spam Filter service...")
    classifier_instance._classifier = None


app = FastAPI(title=settings.app_name, version=settings.app_version, lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.dependency_overrides[get_classifier] = lambda: classifier_instance
app.include_router(router, prefix="/v1")

if __name__ == "__main__":
    import uvicorn

    uvicorn.run("app.main:app", host="0.0.0.0", port=settings.port, reload=False)
