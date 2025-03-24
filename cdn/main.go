package main

import (
	"fmt"
	"log"
	"net/http"
	serve "saes-cdn/src"

	"github.com/joho/godotenv"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Print("No .env")
	}
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "SAES CDN Server written in Go")
	})
	http.HandleFunc("/get", serve.ServeHandler)
	log.Fatal(http.ListenAndServe(":3100", nil))
}
