import Link from "next/link";
import { getAllPosts } from "@/lib/posts";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Home",
  description: "A developer blog sharing knowledge and insights. I started this blog to deepen my knowledge and slow things down in a world moving at a frightening pace.",
  keywords: ["blog", "developer", "programming", "web development", "technology", "software development"],
  alternates: {
    canonical: "/",
  },
};

export default function Home() {
    const posts = getAllPosts();

    return (
        <div className="container-responsive py-6 sm:py-8 md:py-12">
            <div className="max-w-7xl mx-auto px-4">
                {/* Intro Section */}
                <div className="text-center mb-12 sm:mb-16 md:mb-20">
                    <div className="max-w-2xl mx-auto space-y-4 text-base sm:text-lg text-[#9ca3af] leading-relaxed">
                        <p>
                            I started this blog to <strong className="text-[#1ea6d5] font-semibold">deepen</strong> my knowledge a little further.
                        </p>
                        <p>
                            I feel that everything is moving at a <strong className="text-[#1ea6d5] font-semibold">frightening pace</strong> these days, to the point where it&apos;s becoming <strong className="text-[#1ea6d5] font-semibold">overwhelming</strong>.
                        </p>
                        <p>
                            This blog is my way of <strong className="text-[#1ea6d5] font-semibold">slowing things down</strong>—<strong className="text-[#1ea6d5] font-semibold">my space</strong> to move at a <strong className="text-[#1ea6d5] font-semibold">gentler</strong>, more <strong className="text-[#1ea6d5] font-semibold">intentional</strong> rhythm.
                        </p>
                    </div>
                </div>

                {/* Blog Posts Section */}
                <section className="mb-12 sm:mb-16 md:mb-20">
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between mb-6 sm:mb-8 gap-3 sm:gap-4">
                    <h2 className="text-2xl sm:text-3xl md:text-4xl font-bold text-white">
                        Latest Posts
                    </h2>
                    <Link
                        href="/blog"
                        className="text-[#1ea6d5] hover:text-[#1ea9d8] transition-colors duration-200 font-medium text-sm sm:text-base self-start sm:self-center"
                    >
                        View all →
                    </Link>
                </div>

                {posts.length === 0 ? (
                    <div className="text-center py-12 sm:py-16 bg-[#1a1a1a] rounded-lg border border-[#374151]">
                        <p className="text-[#9ca3af] text-base sm:text-lg mb-2">
                            No blog posts yet.
                        </p>
                        <p className="text-[#6b7280] text-sm">
                            Check back soon for new content!
                        </p>
                    </div>
                ) : (
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
                        {posts.slice(0, 6).map((post) => (
                            <article
                                key={post.slug}
                                className="bg-[#1a1a1a] border border-[#374151] rounded-lg p-4 sm:p-6 hover:border-[#4b5563] hover:shadow-lg transition-all duration-200 group flex flex-col h-full"
                            >
                                <div className="mb-3 space-y-1">
                                    <time className="text-xs text-[#6b7280] block">
                                        Published: {new Date(post.date).toLocaleDateString("en-US", {
                                            year: "numeric",
                                            month: "short",
                                            day: "numeric",
                                        })}
                                    </time>
                                    {post.updated && (
                                        <time className="text-xs text-[#9ca3af] block">
                                            Updated: {new Date(post.updated).toLocaleDateString("en-US", {
                                                year: "numeric",
                                                month: "short",
                                                day: "numeric",
                                            })}
                                        </time>
                                    )}
                                </div>

                                <h3 className="text-lg sm:text-xl font-semibold mb-2 sm:mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                                    <Link href={`/blog/${post.slug}`}>
                                        {post.title}
                                    </Link>
                                </h3>

                                {post.excerpt && (
                                    <p className="text-[#9ca3af] text-sm mb-3 sm:mb-4 line-clamp-3 flex-grow">
                                        {post.excerpt}
                                    </p>
                                )}

                                {post.tags && post.tags.length > 0 && (
                                    <div className="flex flex-wrap gap-2 mb-3 sm:mb-4">
                                        {post.tags.slice(0, 3).map((tag) => (
                                            <span
                                                key={tag}
                                                className="px-2 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs rounded border border-[#374151]"
                                            >
                                                {tag}
                                            </span>
                                        ))}
                                    </div>
                                )}

                                <div className="flex items-center justify-start mt-auto pt-3 sm:pt-4 border-t border-[#374151]">
                                    <Link
                                        href={`/blog/${post.slug}`}
                                        className="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm font-medium transition-colors duration-200"
                                    >
                                        Read more →
                                    </Link>
                                </div>
                            </article>
                        ))}
                    </div>
                )}
                </section>
            </div>
        </div>
    );
}
