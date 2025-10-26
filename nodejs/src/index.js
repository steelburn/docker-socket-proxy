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
  console.log(`INFO: Proxying request: ${req.method} ${req.url} from ${clientIP}`);
  next();
});

// Middleware to check API key
app.use((req, res, next) => {
  const apiKey = req.headers['x-api-key'];
  if (!API_KEY || apiKey === API_KEY) {
    return next();
  }
  res.status(403).send('Forbidden: Invalid API Key');
});

// Proxy requests to the Docker socket
app.all('/*', (req, res) => {
  proxy.web(req, res, (err) => {
    console.error('Proxy error:', err);
    res.status(500).send('Proxy error');
  });
});

// Start the server
const PORT = 3277;
app.listen(PORT, () => {
  console.log(`Docker Socket Proxy running on http://localhost:${PORT}`);
});