package services

import (
	"log"
	"os"

	"github.com/gui-gng/jojuhu/message-service/models"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Config struct {
	db *gorm.DB
}

func Init() Config{

	conn := ConnectToDB()
	if conn == nil {
		log.Panic("Can't connect to Postgres!")
	}

	buildDB(conn)

	data := Config{
		db: conn,
	}

	return data
}


func ConnectToDB() (*gorm.DB) {
	dsn := os.Getenv("PG_CONN")
	//CREATE CONNECTION TO POSTGRES
	db, err := gorm.Open(postgres.Open(dsn))

	if err != nil {
		log.Println("Can't connect to Postgres!")
		return nil
	}

	result := db.Exec("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")

    if result.Error != nil {
        // Handle error
    }

	return db
}

func buildDB(conn *gorm.DB) {
	conn.AutoMigrate(&models.Message{})
}