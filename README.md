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
└── mdx-components.tsx # MDX component configuration
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
