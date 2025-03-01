# Portfolio

![rustc](https://img.shields.io/badge/rustc-1.84.0-blue.svg)
![wasm-bindgen](https://img.shields.io/badge/wasm--bindgen-0.2.0-blue.svg)

This is my personal portfolio website built with Rust (Yew), Tailwind CSS, and Trunk.

It showcases my work, projects, and skills.

## Getting Started  

To set up and run the project locally, follow these steps:

### Prerequisites  

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [Trunk](https://trunkrs.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Chrome Driver](https://googlechromelabs.github.io/chrome-for-testing/#stable) (for E2E testing)

### Installation  

```bash
git clone https://github.com/Jozefpodlecki/Jozefpodlecki.github.io
cd Jozefpodlecki.github.io
npx @tailwindcss/cli -i ./styles/main.css -o ./styles/output.css
cargo build
trunk serve
```

### Tailwind CSS  

To watch for changes in your CSS and rebuild automatically:

```bash
npx @tailwindcss/cli -i ./styles/main.css -o ./styles/output.css --watch
```

### Running End-to-End (E2E) Tests  

1. [Download the Chrome driver that matches your browser version](https://googlechromelabs.github.io/chrome-for-testing/#stable).
2. Run the tests:

```bash
cd e2e
cargo build
cargo run
```

### Credits  

The icons used in this project are sourced from the following sites:

- [Tabler Icons](https://tabler.io/icons)  
- [Icons8 - Duolingo Icons](https://icons8.com/icons/set/duolingo)  
- [Favicon Generator](https://favicon.io/favicon-generator/)  
