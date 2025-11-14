import Link from 'next/link';
import { getAllPosts } from '@/lib/posts';

export const metadata = {
  title: 'Blog',
  description: 'Read the latest blog posts about programming, web development, and technology. Explore tutorials, insights, and knowledge sharing.',
  keywords: ['blog', 'programming', 'web development', 'tutorials', 'technology', 'coding'],
  alternates: {
    canonical: '/blog',
  },
};

export default function BlogPage() {
  const posts = getAllPosts();
  const baseUrl = process.env.NEXT_PUBLIC_SITE_URL || 'http://localhost:3000';

  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "Blog",
    name: "Developer Blog",
    description: "A developer blog sharing knowledge and insights about programming, web development, and technology.",
    url: `${baseUrl}/blog`,
    blogPost: posts.map((post) => ({
      "@type": "BlogPosting",
      headline: post.title,
      description: post.excerpt || post.title,
      url: `${baseUrl}/blog/${post.slug}`,
      datePublished: post.date,
      dateModified: post.updated || post.date,
      author: {
        "@type": "Person",
        name: "Developer",
      },
    })),
  };

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <div className="container-responsive py-8 sm:py-12">
      <div className="max-w-7xl mx-auto px-4">
        {posts.length === 0 ? (
          <div className="text-center py-16 sm:py-20 bg-[#1a1a1a] rounded-lg border border-[#374151]">
            <p className="text-[#9ca3af] text-lg mb-2">
              No blog posts yet.
            </p>
            <p className="text-[#6b7280] text-sm">
              Check back soon for new content!
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {posts.map((post) => (
            <article
              key={post.slug}
              className="bg-[#1a1a1a] border border-[#374151] rounded-lg p-6 hover:border-[#4b5563] hover:shadow-lg transition-all duration-200 group flex flex-col"
            >
              <div className="mb-3 space-y-1">
                <div className="text-xs text-[#6b7280] font-medium">
                  {new Date(post.date).toLocaleDateString('en-US', {
                    year: 'numeric',
                    month: 'short',
                    day: 'numeric',
                  })}
                </div>
                {post.updated && (
                  <div className="text-xs text-[#9ca3af]">
                    Updated: {new Date(post.updated).toLocaleDateString('en-US', {
                      year: 'numeric',
                      month: 'short',
                      day: 'numeric',
                    })}
                  </div>
                )}
              </div>
              
              <h2 className="text-xl font-semibold mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                <Link href={`/blog/${post.slug}`} className="hover:underline">
                  {post.title}
                </Link>
              </h2>
              
              {post.excerpt && (
                <p className="text-[#9ca3af] text-sm mb-4 leading-relaxed flex-grow">
                  {post.excerpt}
                </p>
              )}
              
              {post.tags && post.tags.length > 0 && (
                <div className="flex flex-wrap gap-2 mb-4">
                  {post.tags.slice(0, 3).map((tag) => (
                    <span
                      key={tag}
                      className="px-2.5 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs rounded-md border border-[#374151] hover:border-[#4b5563] transition-colors"
                    >
                      {tag}
                    </span>
                  ))}
                  {post.tags.length > 3 && (
                    <span className="px-2.5 py-1 bg-[#0f0f0f] text-[#6b7280] text-xs rounded-md border border-[#374151]">
                      +{post.tags.length - 3}
                    </span>
                  )}
                </div>
              )}
              
              <div className="flex items-center justify-start pt-4 border-t border-[#374151] mt-auto">
                <Link
                  href={`/blog/${post.slug}`}
                  className="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm font-medium transition-colors duration-200 inline-flex items-center gap-1"
                >
                  Read more
                  <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                  </svg>
                </Link>
              </div>
            </article>
          ))}
          </div>
        )}
      </div>
    </div>
    </>
  );
}
