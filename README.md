![Build Status](https://github.com/saik0/regulator/actions/workflows/ci.yml/badge.svg)
![Rust Edition](https://img.shields.io/badge/rust-2024-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Clippy](https://img.shields.io/badge/clippy-pedantic-green.svg)

## 🎛️ Regulator

> **Regulator is a deterministic, ECS-based control system and a homoiconic mirror into itself.**

It records **what happened** exactly once. Everything else catches up.  
Built from ideas in logs, event sourcing, control systems, and version control.  
The system does very little and promises very much.

---

### 📜 Facts

- ➕ **Append-only** — a stable history
- 🔗 **Causal** — facts may imply other facts
- 🍔 **Consumable** — facts are consumed in sequence
- ▶️ **Replayable** — facts can be replayed from the log
- 📐 **Deterministic** — the same inputs produce the same results
- 🪨 **Immutable** — facts are never reinterpreted
- 🧮 **Algebraic** — facts compose under a small set of rules, enforced with property tests

Facts never panic 😱

---

### 🗺️ Landscape

What exists in this repository today:

- ✨ **Sparks** — end-to-end loop verification
- 📜 **Fact Log** — an append-only record of facts
- 🌡️ **Sensors** — instruments that emit measurements
- 🎚️ **Controller** — sequences facts; never decides truth
- 🔁 **Effectors** — apply facts; restartable; track offsets
- 🧮 **Property-based invariants** — facts validated by laws, not cases
- 🧵 **Single-threaded truth spine** — one canonical history

---

### 💥 Self-Correcting Control

**reference → measurement → error → correction**  
The system corrects by replay, not repair.

- 🔁 Effectors are restartable
- 🗑️ Everything else is disposable
- 🕰️ Truth is history
- 📐 Correction is always derived

---

### 📐 Axiomatic Growth

- 👀 **Readers** not writers
- 👂 **Listeners** not controllers
- 🧪 **Properties** not behavior

The only built-in projection is the mirror 🪞  
Growth happens at the edges ⛓️  
The core is only a prelude 🎼

---

### 🔮 Vague Future

- 🦀🧠 **Rust Brain**
- 🤖 **Autonomous agents**
- 🖥️ **Typed UIs**
- 🌍 **Derived worlds**

---

### 🧭 Constraints

- 👨🏻‍💻 Solo-built, curiosity-driven  
- 🧪 Experimental by design. This is my lab.
- 🚫 Not shipped to production. That would be scary!

---

### ▶️ Run

Requirements: 🦀 Rust (stable), 🧰 Cargo, 🧠 Curiosity

```shell
git clone <repo>
cd regulator
cargo test
```

 - Facts are stable.
 - If the tests pass, the algebra holds.  
 - If the algebra holds, the system holds.

```shell
cargo run
```

<pre>
--- Regulator Starting ---
</pre>

*Nothing to see here. The system is holding.*

---

### ⚖️ License

Regulator is a collaborative projection between human and machine intent.  
Released under the [MIT License](LICENSE).

Copyright (c) 2025 @saik0