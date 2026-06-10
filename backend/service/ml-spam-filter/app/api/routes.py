import logging

from fastapi import APIRouter, Depends, HTTPException

from app.core.config import settings
from app.models.schemas import InferenceRequest, InferenceResponse
from app.services.classifier import SpamClassifier

logger = logging.getLogger(__name__)
router = APIRouter()


def get_classifier():
    raise NotImplementedError()


@router.post("/predict", response_model=InferenceResponse)
async def predict_spam(
    request: InferenceRequest, classifier: SpamClassifier = Depends(get_classifier)
):
    try:
        is_spam, score, reason, inf_time = classifier.predict(request.text)

        logger.info(
            f"[Inference] User: {request.user_id} | "
            f"Spam: {is_spam} | Label: {reason} | Score: {score:.4f} | Time: {inf_time:.2f}ms"
        )

        return InferenceResponse(
            is_spam=is_spam, confidence=score, reason=reason, inference_time_ms=inf_time
        )
    except Exception as e:
        logger.error(f"Inference error: {e}")
        return InferenceResponse(
            is_spam=False,
            confidence=0.0,
            reason="error_fallback",
            inference_time_ms=0.0,
        )
