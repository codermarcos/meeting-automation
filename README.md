# 👻 Meeting automation

This is a simple automation that enter in a meeting using chrome and [vimium]()!

**Motivation**:

- Have you ever found yourself programming while an important meeting was happening?
- Have you ever been in a meeting that was held much more frequently than necessary, but you need to be there?

This projects uses Rust with tokio, cron, chronos and enigo.

## 🏃 How to run it

> ⚠️ **Remember:** thats important to have the [chrome extension vimium]() installed!

To run it you just need run [the binary file]() using the following arguments:

- Cron expression
  - Example: `0 0 14 * * Mon`

- Meeting url with the correct profile at end
  - Example: `https://meet.google.com/cod-ofthe-meet`
  - Example for multiples users: `https://meet.google.com/cod-ofthe-meet?authuser=2`

```bash
gmta "<Cron expression>" "<Meeting url>"
```

> Example: `gmta "0 0 14 * * Mon" "https://meet.google.com/cod-ofthe-meet"`

## 🏗️ How to run from sources for development

```bash
cargo run "<Cron expression>" "<Meeting url>"
```

> Example: `cargo run "0 0 14 * * Mon" "https://meet.google.com/cod-ofthe-meet"`

## 🗺️ Roadmap

- [x] Enter in a meeting
- [x] Accept a scheduled date
- [ ] Accept multiples meetings
