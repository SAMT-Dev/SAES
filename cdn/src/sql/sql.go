package sql

import (
	"database/sql"
	"log"
	"os"
	"time"

	_ "github.com/go-sql-driver/mysql"
	"github.com/joho/godotenv"
)

func SQLConn() *sql.DB {
	err := godotenv.Load()
	if err != nil {
		log.Print("No .env")
	}
	sql_conn := os.Getenv("DATABASE_URL")
	db, err := sql.Open("mysql", sql_conn)
	if err != nil {
		panic(err)
	}
	db.SetConnMaxIdleTime(time.Minute * 5)
	db.SetMaxOpenConns(50)
	db.SetMaxIdleConns(5)
	return db
}
