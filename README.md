# Blog

A **personal blog** focused on **software development**, built with Next.js, TypeScript, and MDX.

This blog serves as a space to share knowledge, insights, and experiences in software development, programming, and web technologies.

## Project Structure

```
blog/
├── content/
│   └── blog/          # Blog posts (MDX files)
├── src/
│   ├── app/           # Next.js App Router pages
│   ├── components/    # React components
│   └── lib/           # Utility functions
├── public/            # Static assets
├── server.js          # Custom Express server for cPanel
├── next.config.js     # Next.js configuration
└── package.json       # Project dependencies and scripts
```

## Blog Posts

Blog posts are stored as MDX files in the `content/blog/` directory. Each post is a markdown file with frontmatter metadata. See `content/README.md` for detailed MDX format specification.

## Getting Started

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm start` - Start production server
- `npm run lint` - Run ESLint
- `npm run type-check` - Run TypeScript type checking
- `npm run lint:fix` - Fix ESLint errors automatically

## Tech Stack

- **Next.js** - React framework with App Router
- **TypeScript** - Type-safe JavaScript
- **MDX** - Markdown with JSX support
- **Tailwind CSS** - Utility-first CSS framework
- **rehype-highlight** - Syntax highlighting for code blocks
- **Express** - Custom server for cPanel deployment

## Deployment

### cPanel Deployment

This project includes a custom Express server (`server.js`) for deploying on cPanel with Node.js App.

#### Key Files for cPanel

- **`server.js`** - Custom Express server that runs Next.js in production mode
- **`package.json`** - Contains all dependencies including `express`
- **`next.config.js`** - Next.js configuration (JavaScript format to avoid TypeScript runtime dependency)
- **`.next/`** - Build output folder (generated after `npm run build`)
- **`public/`** - Static assets folder

#### Quick Deployment Steps

1. **Build the project:**
   ```bash
   npm install
   npm run build
   ```

2. **Upload files to cPanel:**
   - Upload all project files including `server.js`, `package.json`, `.next/`, `public/`, and `node_modules/` (or run `npm install` on server)

3. **Configure Node.js App in cPanel:**
   - Go to "Node.js App" in cPanel
   - Set **Application startup file**: `server.js`
   - Set **Application mode**: `production`
   - Set **Node.js version**: `18` or higher
   - Enable **NPM install** (to install dependencies automatically)
   - cPanel will automatically set the `PORT` environment variable

4. **Start the application:**
   - Click "Start" in cPanel Node.js App
   - The server will start and handle all requests

#### Important Notes

- **Port**: cPanel automatically sets the `PORT` environment variable - no need to configure it in `server.js`
- **Express**: Required dependency for `server.js` - included in `package.json`
- **Build**: Always run `npm run build` before deploying
- **Environment Variables**: cPanel handles `NODE_ENV=production` and `PORT` automatically

#### Troubleshooting

- **Server won't start**: Check that `express` is installed (`npm install`) and `npm run build` has been run
- **404 errors**: Verify that `.next/` folder exists and build completed successfully
- **Static files not loading**: Ensure `public/` folder is uploaded correctly
- **"Cannot find module 'express'" error**: Run `npm install` on the server to install all dependencies
- **"Cannot find module 'typescript'" error**: This happens if `next.config.ts` is used. The project uses `next.config.js` to avoid this issue. If you see this error, ensure `next.config.js` exists (not `.ts`)
- **Dependencies not installed**: Make sure to run `npm install` on the server, or enable "NPM install" in cPanel Node.js App settings
