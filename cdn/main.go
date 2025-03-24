package main

import (
	"fmt"
	"log"
	"net/http"
	serve "saes-cdn/src"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request){
		fmt.Fprintf(w,"SAES CDN Server written in Go")
	}) 
	http.HandleFunc("/get", serve.ServeHandler)
	log.Fatal(http.ListenAndServe(":3100",nil))
}