# Blog Post Format (MDX)

This directory contains MDX files for blog posts. All blog files should be placed in the `blog/` folder.

**Quick Start**: Copy `template.mdx` to `blog/your-post-name.mdx` and start writing!

## File Structure

Each blog file must:
- Have a `.mdx` extension
- Be placed in the `content/blog/` directory
- The filename will be used as the `slug` (e.g., `my-post.mdx` → `/blog/my-post`)

## Frontmatter (Metadata)

Each file must start with a frontmatter section containing the post metadata:

```yaml
---
title: "Post Title"
date: "2024-01-15"
updated: "2024-02-20"  # Optional
excerpt: "Short summary of the post for display in post listings"
tags: ["Tag1", "Tag2", "Tag3"]  # Optional
# Optional SEO fields:
# metaDescription: "Custom description for SEO"
# metaKeywords: ["custom", "keywords"]
# ogImage: "/images/og-image.png"  # Path to Open Graph image
# twitterImage: "/images/twitter-image.png"  # Path to Twitter image
# author: "Author Name"  # If not provided, default will be used
---
```

### Required Fields

- **`title`**: Post title (string)
- **`date`**: Publication date in `YYYY-MM-DD` format (string)

### Optional Fields

- **`updated`**: Last update date in `YYYY-MM-DD` format (string)
- **`excerpt`**: Short summary of the post displayed in post listings (string)
- **`tags`**: Array of tags for categorizing the post (string[])
- **`metaDescription`**: Custom description for SEO (if not provided, `excerpt` will be used)
- **`metaKeywords`**: Custom keywords for SEO (if not provided, `tags` will be used)
- **`ogImage`**: Path to Open Graph image for social media
- **`twitterImage`**: Path to Twitter Card image
- **`author`**: Author name (if not provided, default will be used)

## Content

After the frontmatter, write your post content in Markdown/MDX format:

```mdx
---
title: "Post Title"
date: "2024-01-15"
excerpt: "Post summary"
tags: ["Next.js", "React"]
---

# Main Title

This is a paragraph.

## Subtitle

- List item 1
- List item 2

\`\`\`javascript
// Sample code
const example = "Hello World";
\`\`\`
```

## MDX Features

You can use all Markdown features:

- **Headings**: `# H1`, `## H2`, `### H3`, etc.
- **Emphasis**: `*italic*` or `_italic_`, `**bold**` or `__bold__`
- **Lists**: Ordered and unordered lists
- **Links**: `[link text](URL)`
- **Images**: `![alt text](image-url)` - See [Images](#images) section below
- **Code**: Inline code with \`backticks\` and code blocks with \`\`\`
- **Blockquote**: `> quote`
- **Tables**: Markdown tables

### Images

You can add images to your blog posts. Place your images in the `public/images/blog/` directory.

**Basic Markdown syntax:**
```markdown
![Alt text for image](/images/blog/your-image.png)
```

**With custom dimensions (using HTML in MDX):**
```html
<img src="/images/blog/your-image.png" alt="Alt text" width="800" height="450" />
```

**External images:**
```markdown
![External image](https://example.com/image.jpg)
```

**Notes:**
- Local images should be placed in `public/images/blog/`
- Use paths starting with `/images/blog/` for local images
- External images (starting with `http://` or `https://`) are also supported
- Images are automatically optimized using Next.js Image component
- All images are lazy-loaded for better performance

### Syntax Highlighting

Your code will be automatically syntax highlighted:

\`\`\`javascript
function hello() {
  console.log("Hello World");
}
\`\`\`

\`\`\`typescript
const greeting: string = "Hello TypeScript";
\`\`\`

\`\`\`python
def hello():
    print("Hello Python")
\`\`\`

## Complete Example

```mdx
---
title: "Getting Started with Next.js 14"
date: "2024-01-15"
updated: "2024-02-20"
excerpt: "Learn Next.js 14 from scratch"
tags: ["Next.js", "React", "Web Development"]
metaDescription: "Complete Next.js 14 tutorial with practical examples"
author: "Author Name"
---

# Getting Started with Next.js 14

This is a sample post that teaches how to use Next.js 14.

## Introduction

Next.js is a React framework that...

## Installation

To install Next.js:

\`\`\`bash
npx create-next-app@latest my-app
\`\`\`

## Conclusion

Using Next.js, you can...
```

## Important Notes

1. **Filename**: The filename is used as the slug, so avoid special characters and spaces
2. **Date**: Always write dates in `YYYY-MM-DD` format
3. **Excerpt**: Write an engaging summary that encourages readers to read the full post
4. **Tags**: Use relevant and meaningful tags
5. **Code**: Place your code in code blocks so it displays correctly

## Directory Structure

```
content/
├── template.mdx      # Template file - copy this to create new posts
└── blog/
    ├── my-first-post.mdx
    ├── my-second-post.mdx
    └── ...
```

All blog files must be placed in the `blog/` folder.

## Using the Template

To create a new blog post:

1. Copy `template.mdx` to `blog/your-post-name.mdx`
2. Update the frontmatter (title, date, excerpt, tags, etc.)
3. Replace the template content with your actual post content
4. Save the file

The template includes examples of all MDX features you can use in your posts.
