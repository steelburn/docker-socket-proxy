const express = require('express');
const httpProxy = require('http-proxy');
const app = express();
const proxy = httpProxy.createProxyServer({
  target: {
    socketPath: '/var/run/docker.sock'
  }
});

// Log incoming requests
app.use((req, res, next) => {
  console.log(`${req.method} ${req.url}`);
  next();
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