package src

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"saes-cdn/src/utils"
)

func ServeHandler(w http.ResponseWriter, r *http.Request) {
	dir := os.Getenv("PARENT_IMAGE_DIR")
	imgid := r.URL.Query().Get("id")
	if imgid == "" {
		fmt.Fprintf(w, "No img ID provided")
		return
	}
	var filename string
	var tmp int8
	err := utils.GlobalSQL.QueryRow("SELECT filename,tmp FROM images WHERE id = ?", imgid).Scan(&filename, &tmp)
	if err != nil {
		log.Printf("Error: %v", err)
		http.Error(w, "IMG running by that ID has not been found", http.StatusNotFound)
		return
	}
	directory := dir
	if tmp == 1 {
		directory += "/tmp"
	}
	http.ServeFile(w, r, directory+"/"+filename)

}
