# Rust Roadmap

```
██████╗ ██╗   ██╗ ██████╗████████╗██████╗  ██████╗  █████╗ ██████╗ ███╗   ███╗ █████╗ ██████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔══██╗██╔═══██╗██╔══██╗██╔══██╗████╗ ████║██╔══██╗██╔══██╗
██████╔╝██║   ██║███████╗   ██║   ██████╔╝██║   ██║███████║██║  ██║██╔████╔██║███████║██████╔╝
██╔══██╗██║   ██║╚════██║   ██║   ██╔══██╗██║   ██║██╔══██║██║  ██║██║╚██╔╝██║██╔══██║██╔═══╝
██║  ██║╚██████╔╝██████╔╝   ██║   ██║  ██║╚██████╔╝██║  ██║██████╔╝██║ ╚═╝ ██║██║  ██║██║
╚═╝  ╚═╝ ╚═════╝ ╚═════╝   ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝
```

---

## ◆ PULSE

Every Rust learner has asked the same question: *what do I learn next?*
Rust Roadmap answers it with a map - 27 curated sections from
fundamentals to WebAssembly, embedded systems, and game development,
drawn as an interactive fishbone where dependencies point the way.
Click a topic and a drawer opens with the best of what exists to learn
it: official docs, books, videos, courses, crates. The layout never
moves beneath you, and the data is compiled, not stored - a broken
link breaks the build, never the learner.

| 27 sections ▣ | Fishbone ▣ | Resources ▣ | Compile-checked ▣ |
|---|---|---|---|

*The map - data, layout, drawer - is sealed.*

> Built with Rust 2024 + Leptos 0.8, drawn by a deterministic fishbone
> algorithm, shipped to GitHub Pages by one workflow.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One target, one tool, one command.

```
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install trunk
⟫ trunk serve --open
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080).

The release artifact: `⟫ trunk build --release` - optimized output in
`dist/`, deployed to [GitHub Pages](https://suradet-ps.github.io/rust-roadmap/).

<details>
<summary>Prerequisites</summary>

- [Rust](https://www.rust-lang.org/) toolchain (edition 2024)
- [Trunk](https://trunkrs.dev/) - installed above

</details>

---

## ◆ ANATOMY

Three layers, one rule: content never touches layout, layout never
touches rendering.

- **Defines** - `data/` is the source of truth: 27 modular section
  directories, each a Rust module holding its topics, dependencies,
  and content. The compiler validates every reference - orphan topics
  and dead links are build errors, not page bugs.
- **Positions** - `layout/tree.rs` runs the fishbone algorithm on
  explicit `Placement` enums (`Center` / `Left` / `Right`): coordinates
  are derived, never force-directed - the map looks the same on every
  device, every visit.
- **Renders** - Leptos components draw nodes, edges, and the detail
  drawer; fine-grained reactivity re-renders only the node that
  changed, never the whole tree.
- **Explains** - each topic opens a drawer with descriptions and
  curated resources, tagged by kind: Official, Book, Article, Video,
  Course, Interactive, Crate, OpenSource, Community, Podcast,
  Newsletter.
- **Wears** - a premium dark theme on a CSS token system with Rust's
  orange and red - the map reads like the language it teaches.

---

## ◆ RITUALS

**The core ceremony** - the next topic:

1. Open the map. The fishbone shows the path from fundamentals to the
   far domains.
2. Find the topic you are on; its dependencies say what comes next.
3. Click it. The drawer slides in: the description, then the resources,
   each tagged by kind.
4. Learn, advance, repeat - the layout stays still while the learner
   moves.

**The ceremony of the compiled map** - the data is Rust `const`
structs, so the map cannot contain what the compiler rejects. Adding a
section means adding a module - and the build either accepts the map
or refuses it.

**The ceremony of the still layout** - no force-directed jitter, no
random placement, no map that rearranges itself under the cursor. The
fishbone is deterministic because learning is not a lottery.

---

## ◆ ECHOES

**Where this artifact is heading**

```
data     ▸ 27 modular sections, compile-time validated ────────────── ▸ sealed
layout   ▸ fishbone algorithm, explicit placement ─────────────────── ▸ sealed
drawer   ▸ resources by semantic badge ────────────────────────────── ▸ sealed
deliver  ▸ GitHub Pages, one deploy workflow ──────────────────────── ▸ sealed
```

**Raising the artifact** - adding a topic follows the documented
ritual: section module, `Topic` with correct `Placement`, `Dependency`
edges, `TopicContent` in `content.rs`, then `cargo build` as the
verdict. Gates: `cargo fmt --all -- --check`, `cargo clippy
--all-targets -- -D warnings`, `cargo test --verbose`. Open an issue
first to discuss a change.

**Status** - CI gates every push and deploys to GitHub Pages.
[Watch the gates](.github/workflows).

---

```
  ─────────────────────────────────────────
   A roadmap that moves under you
   is a roadmap that lies.
  ─────────────────────────────────────────
```

Licensed under the [MIT License](LICENSE).