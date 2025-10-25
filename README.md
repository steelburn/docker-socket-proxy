# Docker Socket Proxy

Docker Socket Proxy is a Node.js application that proxies Docker REST API requests from a TCP port to the Unix socket `/var/run/docker.sock`. It includes API key authentication for added security.

## Features
- Proxies Docker REST API requests.
- API key authentication for secure access.
- Configurable via environment variables.

## Prerequisites
- Docker
- Node.js (v18 or higher)

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/steelburn/docker-socket-proxy.git
   cd docker-socket-proxy
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Create a `.env` file to configure the API key:
   ```bash
   cp .env.example .env
   ```
   Replace `your-secure-api-key` in the `.env` file with your desired API key.

4. Build and run the application using Docker Compose:
   ```bash
   docker-compose up --build
   ```

## Environment Variables
- `API_KEY`: The API key required for accessing the proxy. Set this in the `.env` file.

## Usage
- The application listens on port `3277` by default.
- Include the API key in the `x-api-key` header of your requests.

Example:
```bash
curl -H "x-api-key: your-secure-api-key" http://localhost:3277/containers/json
```

## License
This project is licensed under the MIT License.