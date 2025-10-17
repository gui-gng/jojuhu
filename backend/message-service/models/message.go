package models

import "github.com/google/uuid"



type Message struct {
	ID  			uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4()"`
	SenderUserId    uuid.UUID    `gorm:"type:uuid;"`
	ReceiverUserId  uuid.UUID    `gorm:"type:uuid;"`
	Msg 			string   
}
