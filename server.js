const next = require('next');
const express = require('express');

// Production mode for server
const dev = false;
const app = next({ dev });
const handle = app.getRequestHandler();

app.prepare().then(() => {
    const server = express();

    // Handle all requests and pass them to Next.js
    server.all('*', (req, res) => {
        return handle(req, res);
    });

    // Start the server - cPanel will set PORT environment variable automatically
    const port = process.env.PORT;
    server.listen(port, (err) => {
        if (err) throw err;
        console.log(`> Next.js running on port ${port}`);
    });
});
