# NoCrypt's Tauri Template

Template for making tauri app feels like native. 

![image](https://github.com/user-attachments/assets/805b80ca-0546-4459-a1f5-ebfa43fec2bb)

Features:

- Svelte v5 + Typescript
- TailwindCSS v4 + [shadcn-svelte](https://www.shadcn-svelte.com/) ready
- Window Controls that feels native (windows only)
- Delay startup to avoid white flashes
- Optimized app size bundle by adjusting Cargo profile
- Disable most of browser features ([tauri-plugin-prevent-default](https://github.com/ferreira-tb/tauri-plugin-prevent-default?tab=readme-ov-file#tauri-plugin-prevent-default))
- Disable all context-menu except for inputs and editable elements (for easier copy and paste)
- Disable tap highlight color

## How to use?

1. Use `degit` to clone the repo

    ```sh
    bun x degit https://github.com/NoCrypt/ncpt-template my-project
    cd my-project
    ```

2. Rename the project name in `package.json`, `Cargo.toml`, `tauri.conf.json`, and `main.rs`
3. Run `bun install` and `bun tauri dev`

## Why did I make this?

Sometimes I just want to create a Tauri App rapidly, but the default template from `create-tauri-app` feels half-baked and gives too much indication that it is a "browser-based" app. This template essentially hides that, along with a few optimizations, and a touch of shadcn theme to make it ready for developing the app logic out of the box.

## Quick Links

- [Tauri Docs](https://tauri.app/start/)
- [Tauri Rust Docs](https://docs.rs/tauri/latest/tauri/)
- [Svelte Docs](https://svelte.dev/docs/svelte/overview)
- [SvelteKit Docs](https://svelte.dev/docs/kit/introduction)
- [shadcn-svelte Docs](https://www.shadcn-svelte.com/)
- [Tailwindcss Docs](https://tailwindcss.com/)
