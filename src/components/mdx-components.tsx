import type { MDXComponents } from 'mdx/types'
import type { ImgHTMLAttributes } from 'react'
import MDXImage from './MDXImage'

export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    img: (props) => {
      const { src, alt, width, height, className } = props as ImgHTMLAttributes<HTMLImageElement>;
      
      if (!src) return null;
      
      // Ensure src is a string (convert if it's a Blob URL)
      const srcString = typeof src === 'string' ? src : src.toString();
      
      return (
        <MDXImage
          src={srcString}
          alt={alt || ''}
          width={width ? parseInt(width.toString()) : undefined}
          height={height ? parseInt(height.toString()) : undefined}
          className={className}
        />
      );
    },
    a: ({ children, ...props }) => (
      <a className="text-[#1ea6d5] hover:text-[#1ea9d8] transition-colors duration-200 underline" {...props}>
        {children}
      </a>
    ),
    h1: ({ children, ...props }) => (
      <h1 className="text-4xl font-bold text-white mb-6 mt-8 first:mt-0" {...props}>
        {children}
      </h1>
    ),
    h2: ({ children, ...props }) => (
      <h2 className="text-3xl font-semibold text-white mb-4 mt-6" {...props}>
        {children}
      </h2>
    ),
    h3: ({ children, ...props }) => (
      <h3 className="text-xl font-semibold text-white mb-2 mt-6" {...props}>
        {children}
      </h3>
    ),
    h4: ({ children, ...props }) => (
      <h4 className="text-lg font-medium text-[#1ea6d5] mb-1 mt-4" {...props}>
        {children}
      </h4>
    ),
    p: ({ children, ...props }) => (
      <p className="text-[#e5e7eb] mb-3 text-base leading-relaxed" {...props}>
        {children}
      </p>
    ),
    hr: ({ ...props }) => (
      <hr className="my-6 border-[#374151]" {...props} />
    ),
    ul: ({ children, ...props }) => (
      <ul className="text-[#e5e7eb] mb-4 space-y-1.5 list-disc list-inside ml-4" {...props}>
        {children}
      </ul>
    ),
    ol: ({ children, ...props }) => (
      <ol className="text-[#e5e7eb] mb-4 space-y-1.5 list-decimal list-inside ml-4" {...props}>
        {children}
      </ol>
    ),
    li: ({ children, ...props }) => (
      <li className="text-[#e5e7eb] mb-1 text-sm leading-relaxed" {...props}>
        {children}
      </li>
    ),
    code: ({ children, ...props }) => (
      <code className="px-2 py-1 bg-[#0f0f0f] text-[#1da1cf] rounded text-sm font-mono" {...props}>
        {children}
      </code>
    ),
    pre: ({ children, ...props }) => (
      <pre className="bg-[#0f0f0f] border border-[#374151] rounded-lg p-4 mb-6 overflow-x-auto" {...props}>
        <code className="text-sm text-[#e5e7eb]">{children}</code>
      </pre>
    ),
    blockquote: ({ children, ...props }) => (
      <blockquote className="border-l-4 border-[#1ea6d5] pl-4 italic text-[#9ca3af] mb-4 my-4" {...props}>
        {children}
      </blockquote>
    ),
    ...components,
  }
}

