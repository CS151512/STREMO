from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    app_name: str = "ML Spam Classifier"
    app_version: str = "2.0.0"
    model_name: str = "valhalla/distilbart-mnli-12-1"
    spam_threshold: float = 0.6
    port: int = 8000

    class Config:
        env_file = ".env"

settings = Settings()
