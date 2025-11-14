import Image from 'next/image';

interface MDXImageProps {
  src: string;
  alt: string;
  width?: number;
  height?: number;
  className?: string;
}

export default function MDXImage({ 
  src, 
  alt, 
  width, 
  height, 
  className = '' 
}: MDXImageProps) {
  if (src.startsWith('http')) {
    const imageWidth = width || 800;
    const imageHeight = height || 450;
    return (
      <div className={`my-6 flex justify-center ${className}`}>
        <div className="relative w-full max-w-4xl rounded-lg border border-[#374151] overflow-hidden">
          <Image
            src={src}
            alt={alt}
            width={imageWidth}
            height={imageHeight}
            className="w-full h-auto"
            loading="lazy"
            unoptimized
          />
        </div>
      </div>
    );
  }

  const imageWidth = width || 800;
  const imageHeight = height || 450;

  return (
    <div className={`my-6 flex justify-center ${className}`}>
      <div className="relative w-full max-w-4xl rounded-lg border border-[#374151] overflow-hidden">
        <Image
          src={src}
          alt={alt}
          width={imageWidth}
          height={imageHeight}
          className="w-full h-auto"
          loading="lazy"
        />
      </div>
    </div>
  );
}

