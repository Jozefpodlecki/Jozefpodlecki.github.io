# Portfolio


## Getting started

```bash
git clone https://github.com/Jozefpodlecki/Jozefpodlecki.github.io
cd Jozefpodlecki.github.io
npx @tailwindcss/cli -i ./styles/main.css -o ./styles/output.css
cargo build
trunk serve
```

```
npx @tailwindcss/cli -i ./styles/main.css -o ./styles/output.css --watch
```

## E2E

[Download chrome driver which matches your version](https://googlechromelabs.github.io/chrome-for-testing/#stable)

```bash
cd e2e
cargo build
cargo run
```

## Credits

The icons used in this project are sourced from the following sites:

- [Tabler Icons](https://tabler.io/icons)
- [Icons8 - Duolingo Icons](https://icons8.com/icons/set/duolingo)
- [Favicon Generator](https://favicon.io/favicon-generator/)