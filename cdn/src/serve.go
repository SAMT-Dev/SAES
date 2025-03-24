package serve

import (
	"fmt"
	"net/http"
)

func ServeHandler(w http.ResponseWriter, r *http.Request) {
	imgid := r.URL.Query().Get("id")
	fmt.Fprintf(w,"Hello serve, %q", imgid)
}