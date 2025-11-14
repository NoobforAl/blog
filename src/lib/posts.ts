import fs from "fs";
import path from "path";
import matter from "gray-matter";
import { serialize } from "next-mdx-remote/serialize";
import type { MDXRemoteSerializeResult } from "next-mdx-remote";

const postsDirectory = path.join(process.cwd(), "content/blog");

export interface BlogPost {
    slug: string;
    title: string;
    date: string;
    updated?: string;
    excerpt?: string;
    tags?: string[];
    content: string;
    metaDescription?: string;
    metaKeywords?: string[];
    ogImage?: string;
    twitterImage?: string;
    author?: string;
}

export interface BlogPostWithContent extends BlogPost {
    mdxSource: MDXRemoteSerializeResult;
}

let postsCache: BlogPost[] | null = null;

export function invalidatePostsCache(): void {
    postsCache = null;
}

export function getAllPosts(): BlogPost[] {
    if (postsCache && process.env.NODE_ENV === 'production') {
        return postsCache;
    }

    try {
        const fileNames = fs.readdirSync(postsDirectory);
        const mdxFiles = fileNames.filter((name) => name.endsWith(".mdx"));

        const posts = mdxFiles.map((fileName) => {
            const slug = fileName.replace(/\.mdx$/, "");
            const fullPath = path.join(postsDirectory, fileName);
            const fileContents = fs.readFileSync(fullPath, "utf8");
            const { data } = matter(fileContents);

        return {
            slug,
            title: data.title || "Untitled",
            date: data.date || "",
            updated: data.updated || "",
            excerpt: data.excerpt || "",
            tags: data.tags || [],
            content: "",
            metaDescription: data.metaDescription || data.excerpt || "",
            metaKeywords: data.metaKeywords || data.tags || [],
            ogImage: data.ogImage || "",
            twitterImage: data.twitterImage || "",
            author: data.author || "",
        };
        });

        const sortedPosts = posts.sort(
            (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime()
        );
        
        postsCache = sortedPosts;
        return sortedPosts;
    } catch (error) {
        console.error("Error reading posts:", error);
        return [];
    }
}

export async function getPostBySlug(
    slug: string
): Promise<BlogPostWithContent | null> {
    try {
        const fullPath = path.join(postsDirectory, `${slug}.mdx`);

        if (!fs.existsSync(fullPath)) {
            return null;
        }

        const fileContents = fs.readFileSync(fullPath, "utf8");
        const { data, content } = matter(fileContents);

        const mdxSource = await serialize(content, {
            mdxOptions: {
                development: process.env.NODE_ENV === "development",
            },
            parseFrontmatter: false,
        });

        return {
            slug,
            title: data.title || "Untitled",
            date: data.date || "",
            updated: data.updated || "",
            excerpt: data.excerpt || "",
            tags: data.tags || [],
            content,
            mdxSource,
            metaDescription: data.metaDescription || data.excerpt || "",
            metaKeywords: data.metaKeywords || data.tags || [],
            ogImage: data.ogImage || "",
            twitterImage: data.twitterImage || "",
            author: data.author || "",
        };
    } catch (error) {
        console.error(`Error reading post ${slug}:`, error);
        return null;
    }
}

export function getAllPostSlugs(): string[] {
    try {
        const fileNames = fs.readdirSync(postsDirectory);
        const mdxFiles = fileNames.filter((name) => name.endsWith(".mdx"));
        return mdxFiles.map((name) => name.replace(/\.mdx$/, ""));
    } catch (error) {
        console.error("Error reading post slugs:", error);
        return [];
    }
}

export function getRelatedPosts(
    currentSlug: string,
    limit: number = 3
): BlogPost[] {
    const allPosts = getAllPosts();
    const currentPost = allPosts.find((post) => post.slug === currentSlug);

    if (!currentPost || !currentPost.tags) {
        return allPosts
            .filter((post) => post.slug !== currentSlug)
            .slice(0, limit);
    }

    const relatedPosts = allPosts
        .filter((post) => post.slug !== currentSlug)
        .filter(
            (post) =>
                post.tags &&
                post.tags.some((tag) => currentPost.tags!.includes(tag))
        )
        .slice(0, limit);

    if (relatedPosts.length < limit) {
        const remainingPosts = allPosts
            .filter((post) => post.slug !== currentSlug)
            .filter(
                (post) =>
                    !relatedPosts.some((related) => related.slug === post.slug)
            )
            .slice(0, limit - relatedPosts.length);

        relatedPosts.push(...remainingPosts);
    }

    return relatedPosts;
}
