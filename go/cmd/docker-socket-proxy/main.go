package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"strings"
)

// DockerSocketPath is the default location for the Docker Unix socket.
const DockerSocketPath = "/var/run/docker.sock"

func main() {
	// 1. Configuration and Setup
	port := os.Getenv("PORT")
	if port == "" {
		port = "3277" // Default port, matching the Rust example
	}
	listenAddr := fmt.Sprintf(":%s", port)

	// 2. Create a custom Transport for the ReverseProxy.
	// This is the core difference: it replaces the default TCP dialing
	// with Unix socket dialing.
	dockerTransport := &http.Transport{
		Dial: func(network, addr string) (net.Conn, error) {
			// Ignore the 'addr' (which would be 'host:port') and always
			// connect to the Unix socket.
			return net.Dial("unix", DockerSocketPath)
		},
	}

	// The Docker daemon doesn't use a "host" or "scheme", so we use a dummy URL.
	// The path part of the original request will be appended to this.
	// The Host header will be set to "localhost" implicitly or can be overridden
	// in the ModifyResponse/Director functions if needed, but usually isn't
	// required for Unix socket communication.
	targetURL, _ := url.Parse("http://unix.sock.docker/")

	// 3. Create the Reverse Proxy handler.
	//
	// Note: we use `httputil.NewSingleHostReverseProxy` for broader stdlib
	// compatibility. Earlier we attempted to use `httputil.NewReverseProxy`,
	// but that symbol may not exist in some older Go stdlib versions. Using
	// `NewSingleHostReverseProxy` expresses our intent clearly: all requests
	// are forwarded to a single target (the Docker socket dummy host). If you
	// bump the Go version and want a different reverse-proxy behavior, update
	// this section accordingly.
	proxy := httputil.NewSingleHostReverseProxy(targetURL)
	proxy.Transport = dockerTransport

	// Optional: Custom Director to modify the request before forwarding.
	// This replaces the Rust code's manual header building.
	proxy.Director = func(req *http.Request) {
		// Determine client IP (prefer X-Forwarded-For if present).
		clientIP := func() string {
			if xf := req.Header.Get("X-Forwarded-For"); xf != "" {
				parts := strings.Split(xf, ",")
				return strings.TrimSpace(parts[0])
			}
			host, _, err := net.SplitHostPort(req.RemoteAddr)
			if err == nil {
				return host
			}
			return req.RemoteAddr
		}()

		// Log the incoming request including client IP
		log.Printf("INFO: Proxying request: %s %s from %s", req.Method, req.URL.Path, clientIP)

		// Overwrite the request Host header to match Docker's expectation (often ignored
		// for sockets, but good practice).
		req.Host = "localhost"

		// --- API Key Logic (Matching Rust's env::var("API_KEY")) ---
		if apiKey := os.Getenv("API_KEY"); apiKey != "" {
			req.Header.Set("x-api-key", apiKey)
			log.Println("INFO: Added API key authentication header")
		}

		// Update the URL to only contain the path needed by the Docker daemon
		req.URL.Scheme = targetURL.Scheme
		req.URL.Host = targetURL.Host
		req.URL.Path = req.URL.Path
	}

	// 4. Start the server.
	log.Printf("🚀 Starting Docker Socket Proxy on 0.0.0.0%s", listenAddr)

	// Listen and serve
	err := http.ListenAndServe(listenAddr, proxy)
	if err != nil {
		log.Fatalf("FATAL: Server failed: %v", err)
	}
}

// Note: The Go implementation automatically handles:
// - Reading the request body and writing it to the socket.
// - Reading the full HTTP response (headers, status, body) from the socket.
// - Correctly parsing Content-Length and Transfer-Encoding.
// - Forwarding all response headers and the body back to the original client.
// - Correctly handling binary data in the response body (no UTF-8 errors).
// - Error handling for socket connection and I/O.
