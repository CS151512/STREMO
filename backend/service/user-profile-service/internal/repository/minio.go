package repository

import (
	"context"
	"fmt"
	"io"
	"log"

	"github.com/minio/minio-go/v7"
)

type MinioRepo struct {
	client *minio.Client
	bucket string
}

func NewMinioRepo(client *minio.Client, bucket string) *MinioRepo {
	ctx := context.Background()
	exists, err := client.BucketExists(ctx, bucket)
	if err == nil && !exists {
		err = client.MakeBucket(ctx, bucket, minio.MakeBucketOptions{})
		if err != nil {
			log.Printf("Failed to create minio bucket %s: %v", bucket, err)
		}
	}

	return &MinioRepo{
		client: client,
		bucket: bucket,
	}
}

func (r *MinioRepo) UploadAvatar(ctx context.Context, userID string,
	reader io.Reader, objectSize int64, contentType string) (string, error) {
	objectName := fmt.Sprintf("%s/avatar.webp", userID)

	_, err := r.client.PutObject(ctx, r.bucket,
		objectName, reader, objectSize, minio.PutObjectOptions{
			ContentType: contentType,
		})
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("/avatars/%s", objectName), nil
}
