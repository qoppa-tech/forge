package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

// Health proves process liveness, not bank connectivity or chain readiness.
func TestHealthOnly(t *testing.T) {
	mux := handler()
	for _, tc := range []struct {
		method string
		path   string
		status int
	}{
		{http.MethodGet, "/health", http.StatusOK},
		{http.MethodGet, "/missing", http.StatusNotFound},
		{http.MethodPost, "/health", http.StatusMethodNotAllowed},
	} {
		r := httptest.NewRecorder()
		mux.ServeHTTP(r, httptest.NewRequest(tc.method, tc.path, nil))
		if r.Code != tc.status {
			t.Fatalf("%s %s: got %d, want %d", tc.method, tc.path, r.Code, tc.status)
		}
		if tc.status == http.StatusOK {
			var body map[string]string
			if err := json.Unmarshal(r.Body.Bytes(), &body); err != nil {
				t.Fatal(err)
			}
			if body["status"] != "ok" || body["service"] != "banking" {
				t.Fatalf("unexpected health response: %v", body)
			}
			if r.Header().Get("Content-Type") != "application/json" {
				t.Fatal("health response must be JSON")
			}
		}
	}
}
