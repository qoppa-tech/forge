package main

import (
	"log"
	"net/http"
	"os"
	"time"
)

func handler() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		if _, err := w.Write([]byte(`{"status":"ok","service":"banking"}`)); err != nil {
			log.Printf("write health response: %v", err)
		}
	})
	return mux
}

func main() {
	addr := os.Getenv("BANKING_ADDR")
	if addr == "" {
		addr = "127.0.0.1:3002"
	}
	server := &http.Server{
		Addr:              addr,
		Handler:           handler(),
		ReadHeaderTimeout: 5 * time.Second,
		ReadTimeout:       10 * time.Second,
		WriteTimeout:      10 * time.Second,
		IdleTimeout:       60 * time.Second,
	}
	log.Fatal(server.ListenAndServe())
}
