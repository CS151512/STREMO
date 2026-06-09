import logging
import time
from typing import Tuple

from transformers import pipeline

logger = logging.getLogger(__name__)


class SpamClassifier:
    def __init__(self, model_name: str, threshold: float):
        self.model_name = model_name
        self.threshold = threshold
        self._classifier = None
        self.candidate_labels = ["spam", "toxic", "advertisement", "normal chat"]

    def load_model(self):
        """Loads the HuggingFace model into memory. This should be called during startup."""
        logger.info(f"Loading ML Model ({self.model_name})... This might take a while.")
        self._classifier = pipeline("zero-shot-classification", model=self.model_name)
        logger.info("ML Model loaded successfully.")

    def is_loaded(self) -> bool:
        return self._classifier is not None

    def predict(self, text: str) -> Tuple[bool, float, str, float]:
        """
        Runs the zero-shot classification on the given text.
        Returns: (is_spam, confidence, reason, inference_time_ms)
        """
        if not self.is_loaded():
            raise RuntimeError("Model is not loaded. Call load_model() first.")

        start_time = time.time()

        result = self._classifier(text, self.candidate_labels)

        top_label = result["labels"][0]
        top_score = result["scores"][0]

        is_spam = False
        reason = ""

        if (
            top_label in ["spam", "toxic", "advertisement"]
            and top_score > self.threshold
        ):
            is_spam = True
            reason = top_label

        inference_time_ms = (time.time() - start_time) * 1000

        return is_spam, top_score, reason, inference_time_ms
