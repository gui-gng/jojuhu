package services

import (
	"context"

	"github.com/google/uuid"
	"github.com/gui-gng/jojuhu/message-service/models"
	"go.opentelemetry.io/otel"
)

func (data *Config) List(ctx context.Context, senderUserId uuid.UUID, receiverUserId uuid.UUID) ([]models.Message, error) {
	var messages []models.Message
	data.db.Find(&messages, models.Message{SenderUserId: senderUserId, ReceiverUserId: receiverUserId})
	return messages, nil
}

func (data *Config) Send(ctx context.Context, senderUserId uuid.UUID, receiverUserId uuid.UUID, msg string) (*models.Message, error) {
	_, span := otel.Tracer("message-service").Start(ctx, "Send")
	defer span.End()


	message := models.Message{
		SenderUserId:     senderUserId,
		ReceiverUserId: receiverUserId,
		Msg: msg,
	}
	r := data.db.Create(&message)

	if r.Error != nil {
		return nil, r.Error
	}

	return &message, nil
}


