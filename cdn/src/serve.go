package src

import (
	"fmt"
	"net/http"
	"os"
)

func ServeHandler(w http.ResponseWriter, r *http.Request) {
	dir := os.Getenv("PARENT_IMAGE_DIR")
	imgid := r.URL.Query().Get("id")
	if imgid == "" {
		fmt.Fprintf(w, "No img ID provided")
		return
	}
	db := SQLConn()
	var filename string
	var tmp int8
	err := db.QueryRow("SELECT filename,tmp FROM images WHERE id = ?", imgid).Scan(&filename, &tmp)
	if err != nil {
		fmt.Printf("Error: %v", err)
		http.Error(w, "IMG running by that ID has not been found", http.StatusNotFound)
		return
	}
	directory := dir
	if tmp == 1 {
		directory += "/tmp"
	}
	http.ServeFile(w, r, directory+"/"+filename)

}
