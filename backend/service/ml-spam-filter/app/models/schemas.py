from pydantic import BaseModel, Field

class InferenceRequest(BaseModel):
    text: str = Field(..., description="The message text to analyze")
    user_id: str = Field(..., description="UUID of the user who sent the message")
    channel_id: str = Field(default="", description="Channel UUID (optional)")

class InferenceResponse(BaseModel):
    is_spam: bool = Field(..., description="True if the message is classified as spam/toxic")
    confidence: float = Field(..., description="Confidence score from 0.0 to 1.0")
    reason: str = Field(..., description="Reason for the classification (e.g., 'toxic', 'spam', 'advertisement')")
    inference_time_ms: float = Field(..., description="Time taken to run the inference in milliseconds")
