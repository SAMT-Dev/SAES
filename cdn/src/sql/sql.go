package sql

import (
	"database/sql"
	"os"
	"time"

	_ "github.com/go-sql-driver/mysql"
)

func SQLConn() *sql.DB {
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
