require('dotenv').config();

const express = require('express');
const httpProxy = require('http-proxy');
const app = express();
const proxy = httpProxy.createProxyServer({
  target: {
    socketPath: '/var/run/docker.sock'
  }
});

const API_KEY = process.env.API_KEY;

// Log incoming requests
app.use((req, res, next) => {
  const clientIP = req.headers['x-forwarded-for'] ? req.headers['x-forwarded-for'].split(',')[0].trim() : req.ip;
  const path = req.url.split('?')[0];
  console.log(`INFO: Proxying request: ${req.method} ${path} from ${clientIP}`);
  next();
});

// Middleware to add API key
app.use((req, res, next) => {
  if (API_KEY) {
    req.headers['x-api-key'] = API_KEY;
    console.log('INFO: Added API key authentication header');
  }
  next();
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'healthy' });
});

// Proxy requests to the Docker socket
app.all('/*', (req, res) => {
  proxy.web(req, res, (err) => {
    console.error(`ERROR: Proxy error: ${err.message}`);
    res.status(500).send('Proxy error');
  });
});

// Start the server
const PORT = process.env.PORT || 3277;
app.listen(PORT, () => {
  console.log(`🚀 Starting Docker Socket Proxy on 0.0.0.0:${PORT}`);
});