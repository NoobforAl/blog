/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: [
        './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
        './src/components/**/*.{js,ts,jsx,tsx,mdx}',
        './src/app/**/*.{js,ts,jsx,tsx,mdx}',
    ],
    theme: {
        extend: {
            screens: {
                'xs': '475px',
                'sm': '640px',
                'md': '768px',
                'lg': '1024px',
                'xl': '1280px',
                '2xl': '1536px',
            },
            spacing: {
                '18': '4.5rem',
                '88': '22rem',
                '128': '32rem',
            },
            fontSize: {
                'xs': ['0.75rem', { lineHeight: '1rem' }],
                'sm': ['0.875rem', { lineHeight: '1.25rem' }],
                'base': ['1rem', { lineHeight: '1.5rem' }],
                'lg': ['1.125rem', { lineHeight: '1.75rem' }],
                'xl': ['1.25rem', { lineHeight: '1.75rem' }],
                '2xl': ['1.5rem', { lineHeight: '2rem' }],
                '3xl': ['1.875rem', { lineHeight: '2.25rem' }],
                '4xl': ['2.25rem', { lineHeight: '2.5rem' }],
                '5xl': ['3rem', { lineHeight: '1' }],
                '6xl': ['3.75rem', { lineHeight: '1' }],
            },
            zIndex: {
                '60': '60',
                '70': '70',
                '80': '80',
                '90': '90',
                '100': '100',
            },
            colors: {
                'terminal': {
                    'green': '#10b981',
                    'amber': '#f59e0b',
                    'brown': '#a78b5b',
                    'cyan': '#06b6d4',
                    'magenta': '#8b5cf6',
                    'yellow': '#eab308',
                    'red': '#ef4444',
                    'blue': '#3b82f6',
                    'white': '#f3f4f6',
                    'gray': '#9ca3af',
                    'orange': '#f97316',
                    'purple': '#a855f7',
                },
                'hacker': {
                    'green': '#10b981',
                    'dark': '#0f0f0f',
                    'gray': '#1a1a1a',
                    'accent': '#06b6d4',
                    'warning': '#f59e0b',
                    'error': '#ef4444',
                    'soft': '#34d399',
                    'brown': '#d4af37',
                    'magenta': '#8b5cf6',
                },
            },
            fontFamily: {
                'mono': ['ui-monospace', 'SFMono-Regular', 'Monaco', 'Consolas', 'Liberation Mono', 'Courier New', 'monospace'],
            },
            animation: {
                'fade-in': 'fadeIn 0.5s ease-out',
                'fade-in-up': 'fadeInUp 0.6s ease-out',
                'slide-in-down': 'slideInDown 0.4s ease-out',
            },
            keyframes: {
                fadeIn: {
                    'from': { opacity: '0' },
                    'to': { opacity: '1' },
                },
                fadeInUp: {
                    'from': { opacity: '0', transform: 'translateY(20px)' },
                    'to': { opacity: '1', transform: 'translateY(0)' },
                },
                slideInDown: {
                    'from': { opacity: '0', transform: 'translateY(-10px)' },
                    'to': { opacity: '1', transform: 'translateY(0)' },
                },
            },
            typography: {
                DEFAULT: {
                    css: {
                        maxWidth: 'none',
                        color: 'inherit',
                        a: {
                            color: '#3b82f6',
                            textDecoration: 'underline',
                            fontWeight: '500',
                        },
                        '[class~="lead"]': {
                            color: 'inherit',
                        },
                        strong: {
                            color: 'inherit',
                            fontWeight: '600',
                        },
                        'ol[type="A"]': {
                            '--list-counter-style': 'upper-alpha',
                        },
                        'ol[type="a"]': {
                            '--list-counter-style': 'lower-alpha',
                        },
                        'ol[type="A" s]': {
                            '--list-counter-style': 'upper-alpha',
                        },
                        'ol[type="a" s]': {
                            '--list-counter-style': 'lower-alpha',
                        },
                        'ol[type="I"]': {
                            '--list-counter-style': 'upper-roman',
                        },
                        'ol[type="i"]': {
                            '--list-counter-style': 'lower-roman',
                        },
                        'ol[type="I" s]': {
                            '--list-counter-style': 'upper-roman',
                        },
                        'ol[type="i" s]': {
                            '--list-counter-style': 'lower-roman',
                        },
                        'ol[type="1"]': {
                            '--list-counter-style': 'decimal',
                        },
                        'ol > li': {
                            position: 'relative',
                        },
                        'ol > li::before': {
                            content: 'counter(list-item, var(--list-counter-style, decimal)) "."',
                            position: 'absolute',
                            fontWeight: '400',
                            color: 'inherit',
                        },
                        'ul > li': {
                            position: 'relative',
                        },
                        'ul > li::before': {
                            content: '""',
                            position: 'absolute',
                            backgroundColor: 'currentColor',
                            borderRadius: '50%',
                        },
                        blockquote: {
                            fontWeight: '500',
                            fontStyle: 'italic',
                            color: 'inherit',
                            borderLeftWidth: '0.25rem',
                            borderLeftColor: '#e5e7eb',
                            quotes: '"\\201C""\\201D""\\2018""\\2019"',
                            marginTop: '1.6em',
                            marginBottom: '1.6em',
                            paddingLeft: '1em',
                        },
                        h1: {
                            color: 'inherit',
                            fontWeight: '800',
                            fontSize: '2.25em',
                            marginTop: '0',
                            marginBottom: '0.8888889em',
                            lineHeight: '1.1111111',
                        },
                        h2: {
                            color: 'inherit',
                            fontWeight: '700',
                            fontSize: '1.5em',
                            marginTop: '2em',
                            marginBottom: '1em',
                            lineHeight: '1.3333333',
                        },
                        h3: {
                            color: 'inherit',
                            fontWeight: '600',
                            fontSize: '1.25em',
                            marginTop: '1.6em',
                            marginBottom: '0.6em',
                            lineHeight: '1.6',
                        },
                        h4: {
                            color: 'inherit',
                            fontWeight: '600',
                            marginTop: '1.5em',
                            marginBottom: '0.5em',
                            lineHeight: '1.5',
                        },
                        'img, svg, video, canvas, audio, iframe, embed, object': {
                            display: 'block',
                            verticalAlign: 'middle',
                        },
                        'img, video': {
                            maxWidth: '100%',
                            height: 'auto',
                        },
                        '[hidden]': {
                            display: 'none',
                        },
                        code: {
                            color: 'inherit',
                            fontWeight: '600',
                            fontSize: '0.875em',
                        },
                        'code::before': {
                            content: '"`"',
                        },
                        'code::after': {
                            content: '"`"',
                        },
                        pre: {
                            color: '#e5e7eb',
                            backgroundColor: '#1f2937',
                            overflowX: 'auto',
                            fontWeight: '400',
                            fontSize: '0.875em',
                            lineHeight: '1.7142857',
                            marginTop: '1.7142857em',
                            marginBottom: '1.7142857em',
                            borderRadius: '0.375rem',
                            paddingTop: '0.8571429em',
                            paddingRight: '1.1428571em',
                            paddingBottom: '0.8571429em',
                            paddingLeft: '1.1428571em',
                        },
                        'pre code': {
                            backgroundColor: 'transparent',
                            borderWidth: '0',
                            borderRadius: '0',
                            padding: '0',
                            fontWeight: 'inherit',
                            color: 'inherit',
                            fontSize: 'inherit',
                            fontFamily: 'inherit',
                            lineHeight: 'inherit',
                        },
                        'pre code::before': {
                            content: 'none',
                        },
                        'pre code::after': {
                            content: 'none',
                        },
                        table: {
                            width: '100%',
                            tableLayout: 'auto',
                            textAlign: 'left',
                            marginTop: '2em',
                            marginBottom: '2em',
                            fontSize: '0.875em',
                            lineHeight: '1.7142857',
                        },
                        thead: {
                            color: 'inherit',
                            fontWeight: '600',
                            borderBottomWidth: '1px',
                            borderBottomColor: '#374151',
                        },
                        'thead th': {
                            verticalAlign: 'bottom',
                            paddingRight: '0.5714286em',
                            paddingBottom: '0.5714286em',
                            paddingLeft: '0.5714286em',
                        },
                        'tbody tr': {
                            borderBottomWidth: '1px',
                            borderBottomColor: '#374151',
                        },
                        'tbody tr:last-child': {
                            borderBottomWidth: '0',
                        },
                        'tbody td': {
                            verticalAlign: 'top',
                            paddingTop: '0.5714286em',
                            paddingRight: '0.5714286em',
                            paddingBottom: '0.5714286em',
                            paddingLeft: '0.5714286em',
                        },
                    },
                },
                invert: {
                    css: {
                        a: {
                            color: '#60a5fa',
                        },
                        strong: {
                            color: 'inherit',
                        },
                        'ol[type="A"]': {
                            '--list-counter-style': 'upper-alpha',
                        },
                        'ol[type="a"]': {
                            '--list-counter-style': 'lower-alpha',
                        },
                        'ol[type="A" s]': {
                            '--list-counter-style': 'upper-alpha',
                        },
                        'ol[type="a" s]': {
                            '--list-counter-style': 'lower-alpha',
                        },
                        'ol[type="I"]': {
                            '--list-counter-style': 'upper-roman',
                        },
                        'ol[type="i"]': {
                            '--list-counter-style': 'lower-roman',
                        },
                        'ol[type="I" s]': {
                            '--list-counter-style': 'upper-roman',
                        },
                        'ol[type="i" s]': {
                            '--list-counter-style': 'lower-roman',
                        },
                        'ol[type="1"]': {
                            '--list-counter-style': 'decimal',
                        },
                        blockquote: {
                            borderLeftColor: '#4b5563',
                        },
                        h1: {
                            color: 'inherit',
                        },
                        h2: {
                            color: 'inherit',
                        },
                        h3: {
                            color: 'inherit',
                        },
                        h4: {
                            color: 'inherit',
                        },
                        'img, svg, video, canvas, audio, iframe, embed, object': {
                            display: 'block',
                            verticalAlign: 'middle',
                        },
                        'img, video': {
                            maxWidth: '100%',
                            height: 'auto',
                        },
                        '[hidden]': {
                            display: 'none',
                        },
                        code: {
                            color: 'inherit',
                        },
                        'pre': {
                            color: '#d1d5db',
                            backgroundColor: '#111827',
                        },
                        thead: {
                            color: 'inherit',
                            borderBottomColor: '#4b5563',
                        },
                        'tbody tr': {
                            borderBottomColor: '#4b5563',
                        },
                    },
                },
            },
        },
    },
    plugins: [
        // eslint-disable-next-line @typescript-eslint/no-require-imports
        require('@tailwindcss/typography'),
        function({ addUtilities, theme }) {
            addUtilities({
                '.terminal-green': {
                    color: theme('colors.terminal.green'),
                },
                '.terminal-amber': {
                    color: theme('colors.terminal.amber'),
                },
                '.terminal-cyan': {
                    color: theme('colors.terminal.cyan'),
                },
                '.terminal-magenta': {
                    color: theme('colors.terminal.magenta'),
                },
                '.terminal-yellow': {
                    color: theme('colors.terminal.yellow'),
                },
                '.terminal-red': {
                    color: theme('colors.terminal.red'),
                },
                '.terminal-blue': {
                    color: theme('colors.terminal.blue'),
                },
                '.terminal-white': {
                    color: theme('colors.terminal.white'),
                },
                '.terminal-gray': {
                    color: theme('colors.terminal.gray'),
                },
                '.terminal-orange': {
                    color: theme('colors.terminal.orange'),
                },
                '.terminal-purple': {
                    color: theme('colors.terminal.purple'),
                },
                '.hacker-text': {
                    color: theme('colors.hacker.green'),
                },
                '.hacker-accent': {
                    color: theme('colors.hacker.accent'),
                },
                '.hacker-warning': {
                    color: theme('colors.hacker.warning'),
                },
                '.hacker-error': {
                    color: theme('colors.hacker.error'),
                },
                '.hacker-bg': {
                    backgroundColor: theme('colors.hacker.dark'),
                },
                '.hacker-bg-gray': {
                    backgroundColor: theme('colors.hacker.gray'),
                },
                '.hacker-soft': {
                    color: theme('colors.hacker.soft'),
                },
                '.hacker-command': {
                    position: 'relative',
                    paddingLeft: '20px',
                },
                '.hacker-prompt': {
                    position: 'relative',
                    paddingLeft: '30px',
                },
                '.hacker-matrix': {
                    position: 'relative',
                    overflow: 'hidden',
                    backgroundColor: theme('colors.hacker.dark'),
                },
                '.hacker-terminal-soft': {
                    backgroundColor: '#1a1a1a',
                    border: '1px solid #374151',
                    color: theme('colors.text-primary'),
                    borderRadius: '0.5rem',
                    boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
                    transition: 'all 0.2s ease',
                    '&:hover': {
                        borderColor: '#4b5563',
                        boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
                        backgroundColor: '#262626',
                    },
                },
                '.hover-glow': {
                    transition: 'all 0.2s ease',
                    '&:hover': {
                        transform: 'translateY(-1px)',
                    },
                },
                '.z-nav': {
                    zIndex: '50',
                },
                '.container-responsive': {
                    width: '100%',
                    maxWidth: '1280px',
                    marginLeft: 'auto',
                    marginRight: 'auto',
                    paddingLeft: '1rem',
                    paddingRight: '1rem',
                    '@media (min-width: 640px)': {
                        paddingLeft: '1.5rem',
                        paddingRight: '1.5rem',
                    },
                    '@media (min-width: 1024px)': {
                        paddingLeft: '2rem',
                        paddingRight: '2rem',
                    },
                },
            });
        },
    ],
}
