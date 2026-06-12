//! Resume / portfolio content for the home page.
//! Edit this file to update the personal info shown on the site.

pub const NAME: &str = "NoobforAl";
pub const ROLE: &str = "Software Developer";
pub const TAGLINE: &str =
    "I like the layers most people skip — C, Assembly, Rust, and the quiet machinery underneath the web.";
pub const GITHUB_URL: &str = "https://github.com/NoobforAl";

/// Shown inside the `$ cat about.md` terminal window.
pub const ABOUT: &[&str] = &[
    "I started this blog to deepen my knowledge a little further.",
    "I feel that everything is moving at a frightening pace these days, to the point where it's becoming overwhelming.",
    "This blog is my way of slowing things down — my space to move at a gentler, more intentional rhythm.",
];

pub struct SkillGroup {
    pub label: &'static str,
    pub items: &'static [&'static str],
}

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        label: "languages",
        items: &["Rust", "C", "Go", "Assembly", "TypeScript", "Python"],
    },
    SkillGroup {
        label: "systems",
        items: &["Linux", "WebAssembly", "Networking", "Git", "Docker"],
    },
    SkillGroup {
        label: "interests",
        items: &[
            "Low-level programming",
            "Reverse engineering",
            "Privacy tech",
            "Performance",
        ],
    },
];

pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub url: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "blog",
        description: "This site — a fully static blog written in Rust, compiled to WebAssembly with Yew, posts embedded at build time.",
        tech: &["Rust", "Yew", "WASM", "Tailwind"],
        url: "https://github.com/NoobforAl",
    },
    Project {
        name: "monerodd",
        description: "Tinkering with Monero internals — digging into how privacy-preserving money actually works under the hood.",
        tech: &["Privacy", "Cryptocurrency"],
        url: "https://github.com/NoobforAl",
    },
];
