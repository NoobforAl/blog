import { notFound } from "next/navigation";
import Link from "next/link";
import { getPostBySlug, getAllPostSlugs, getRelatedPosts } from "@/lib/posts";
import { MDXRemote } from "next-mdx-remote/rsc";
import { useMDXComponents } from "@/components/mdx-components";
import rehypeHighlight from "rehype-highlight";
import type { Metadata } from "next";

interface BlogPostPageProps {
    params: Promise<{
        slug: string;
    }>;
}

export async function generateStaticParams() {
    const slugs = getAllPostSlugs();
    return slugs.map((slug) => ({
        slug,
    }));
}

export async function generateMetadata({
    params,
}: BlogPostPageProps): Promise<Metadata> {
    const { slug } = await params;
    const post = await getPostBySlug(slug);

    if (!post) {
        return {
            title: "Post Not Found",
        };
    }

    const metadataBase = process.env.NEXT_PUBLIC_SITE_URL || "http://localhost:3000";
    const postUrl = `${metadataBase}/blog/${slug}`;
    
    const description = post.metaDescription || post.excerpt || `Read ${post.title}`;
    const keywords = post.metaKeywords && post.metaKeywords.length > 0 ? post.metaKeywords : (post.tags || []);
    const author = post.author || "Developer";
    const ogImage = post.ogImage ? (post.ogImage.startsWith('http') ? post.ogImage : `${metadataBase}${post.ogImage}`) : undefined;
    const twitterImage = post.twitterImage ? (post.twitterImage.startsWith('http') ? post.twitterImage : `${metadataBase}${post.twitterImage}`) : ogImage;

    return {
        title: post.title,
        description,
        keywords,
        authors: [{ name: author }],
        openGraph: {
            title: post.title,
            description,
            type: "article",
            publishedTime: post.date,
            ...(post.updated && { modifiedTime: post.updated }),
            url: postUrl,
            siteName: "Developer Blog",
            tags: post.tags || [],
            ...(ogImage && { images: [{ url: ogImage }] }),
        },
        twitter: {
            card: "summary_large_image",
            title: post.title,
            description,
            ...(twitterImage && { images: [twitterImage] }),
        },
        alternates: {
            canonical: postUrl,
        },
    };
}

export default async function BlogPostPage({ params }: BlogPostPageProps) {
    const { slug } = await params;
    const post = await getPostBySlug(slug);

    if (!post) {
        notFound();
    }

    const relatedPosts = getRelatedPosts(slug);
    const metadataBase = process.env.NEXT_PUBLIC_SITE_URL || "http://localhost:3000";
    const postUrl = `${metadataBase}/blog/${slug}`;

    const description = post.metaDescription || post.excerpt || post.title;
    const author = post.author || "Developer";
    const ogImage = post.ogImage ? (post.ogImage.startsWith('http') ? post.ogImage : `${metadataBase}${post.ogImage}`) : `${metadataBase}/og-image.png`;
    
    const jsonLd = {
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        headline: post.title,
        description,
        image: ogImage,
        datePublished: post.date,
        dateModified: post.updated || post.date,
        author: {
            "@type": "Person",
            name: author,
        },
        publisher: {
            "@type": "Organization",
            name: "Developer Blog",
        },
        mainEntityOfPage: {
            "@type": "WebPage",
            "@id": postUrl,
        },
        keywords: (post.metaKeywords && post.metaKeywords.length > 0 ? post.metaKeywords : post.tags)?.join(", ") || "",
    };

    // eslint-disable-next-line react-hooks/rules-of-hooks
    const mdxComponents = useMDXComponents({});

    return (
        <>
            <script
                type="application/ld+json"
                dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
            />
            <div className="container-responsive py-8 sm:py-12">
            <div className="max-w-4xl mx-auto px-4">
                {/* Back to blog */}
                <div className="mb-6 sm:mb-8">
                    <Link
                        href="/blog"
                        className="inline-flex items-center text-[#9ca3af] hover:text-white transition-colors duration-200 text-sm"
                    >
                        <svg className="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                        </svg>
                        Back to Blog
                    </Link>
                </div>

                {/* Article Header */}
                <article className="mb-12 sm:mb-16">
                <header className="mb-8 sm:mb-12">
                    <h1 className="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-6 text-white leading-tight">
                        {post.title}
                    </h1>

                    <div className="flex flex-col gap-4 mb-6">
                        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                            <div className="space-y-1">
                                <time className="text-sm text-[#9ca3af] block">
                                    Published: {new Date(post.date).toLocaleDateString("en-US", {
                                        year: "numeric",
                                        month: "long",
                                        day: "numeric",
                                    })}
                                </time>
                                {post.updated && (
                                    <time className="text-sm text-[#6b7280] block">
                                        Updated: {new Date(post.updated).toLocaleDateString("en-US", {
                                            year: "numeric",
                                            month: "long",
                                            day: "numeric",
                                        })}
                                    </time>
                                )}
                            </div>

                            {post.tags && post.tags.length > 0 && (
                                <div className="flex flex-wrap gap-2">
                                    {post.tags.map((tag) => (
                                        <span
                                            key={tag}
                                            className="px-3 py-1 bg-[#1a1a1a] text-[#9ca3af] text-sm rounded-md border border-[#374151]"
                                        >
                                            {tag}
                                        </span>
                                    ))}
                                </div>
                            )}
                        </div>
                    </div>
                </header>

                {/* Article Content */}
                <div className="bg-[#1a1a1a] border border-[#374151] rounded-lg p-6 sm:p-8 lg:p-12">
                    <div className="prose prose-invert max-w-none prose-headings:text-white prose-p:text-[#e5e7eb] prose-a:text-[#1ea6d5] prose-a:no-underline hover:prose-a:underline prose-strong:text-white prose-code:text-[#1da1cf] prose-pre:bg-[#0f0f0f] prose-pre:border prose-pre:border-[#374151]">
                        <MDXRemote 
                            source={post.content} 
                            components={mdxComponents}
                            options={{
                                mdxOptions: {
                                    rehypePlugins: [rehypeHighlight],
                                },
                            }}
                        />
                    </div>
                </div>
            </article>

                {/* Related Posts */}
                {relatedPosts.length > 0 && (
                    <section className="mt-16 sm:mt-20 pt-8 sm:pt-12 border-t border-[#374151]">
                        <h2 className="text-2xl sm:text-3xl font-bold mb-8 text-white">
                            Related Posts
                        </h2>
                        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                            {relatedPosts.map((relatedPost) => (
                                <article
                                    key={relatedPost.slug}
                                    className="bg-[#1a1a1a] border border-[#374151] rounded-lg p-6 hover:border-[#4b5563] hover:shadow-lg transition-all duration-200 group"
                                >
                                    <h3 className="text-lg font-semibold mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                                        <Link href={`/blog/${relatedPost.slug}`}>
                                            {relatedPost.title}
                                        </Link>
                                    </h3>

                                    {relatedPost.excerpt && (
                                        <p className="text-[#9ca3af] text-sm mb-4 line-clamp-2">
                                            {relatedPost.excerpt}
                                        </p>
                                    )}

                                    <div className="flex items-center justify-between pt-4 border-t border-[#374151]">
                                        <time className="text-xs text-[#6b7280]">
                                            {new Date(relatedPost.date).toLocaleDateString("en-US", {
                                                year: "numeric",
                                                month: "short",
                                                day: "numeric",
                                            })}
                                        </time>
                                        <Link
                                            href={`/blog/${relatedPost.slug}`}
                                            className="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm font-medium transition-colors duration-200"
                                        >
                                            Read →
                                        </Link>
                                    </div>
                                </article>
                            ))}
                        </div>
                    </section>
                )}

                {/* Navigation */}
                <div className="mt-12 sm:mt-16 pt-8 sm:pt-12 border-t border-[#374151]">
                    <div className="flex justify-center">
                        <Link
                            href="/blog"
                            className="px-6 py-3 bg-[#1a1a1a] text-white border border-[#374151] rounded-lg font-medium hover:bg-[#262626] hover:border-[#4b5563] transition-all duration-200"
                        >
                            View All Posts
                        </Link>
                    </div>
                </div>
            </div>
        </div>
        </>
    );
}
