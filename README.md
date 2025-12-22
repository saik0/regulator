![Build Status](https://github.com/saik0/regulator/actions/workflows/ci.yml/badge.svg)
![Rust Edition](https://img.shields.io/badge/rust-2024-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Clippy](https://img.shields.io/badge/clippy-pedantic-green.svg)

# 🎛️ Regulator

> **Regulator is a deterministic, ECS-based control system and a 12-dimensional cybernetic manifold—a homoiconic mirror
> built to wake up, sense its own state, and reason about its own DNA as the control board for a creature that doesn't
> exist yet, though the plumbing is ready**

---

> [!WARNING]
> **Interleaved Reference Frames Detected**
> This documentation simultaneously addresses:
> 1. **The Regulator:** The engineering reality acting as the control board.
> 2. **The Organism:** The living system (organic or cybernetic) reading this document.

A regulator records **what happened** exactly once. Everything else is just catching up.

### Facts as Cells 🦠
Facts are the atomic unit of truth—the cells from which everything grows. Facts never panic; they flow.

* ➕ **Append-only** — A stable, linear history.
* 🔗 **Causal** — Facts may imply or trigger other facts.
* 🍔 **Consumable** — Processed in strict sequence.
* ▶️ **Replayable** — Fully recoverable from the log.
* 📐 **Deterministic** — Pure functions of history.
* 🪨 **Immutable** — Truth is never reinterpreted.
* 🧮 **Algebraic** — Composable under property-tested rules.

---

## 1. The Manifold Strategy (State as Metabolism)

Regulator manages information by treating it as a metabolism. Data is promoted through different states of integrity
across the manifold rather than simply being "stored."

| Row (State) | Metabolism | Dimension: Type | Dimension: Chronicle | Dimension: Value | Dimension: RA / LSP |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Step** | **Spark** (Sensing) | `Proposal<Proof>` | *Pre-Commit* | `Proposal<V>` | *Validation Query* |
| **Fact** | **State** (Knowing) | **Active Law** | **Memory (Hot)** | **Known Value** | **Active Symbol** |
| **History** | **Archive** (Memory) | `Saved Proof` | **Git-Oid Ledger** | `Historical V` | **Indexed Node** |

| Axis | Runtime Timeline | Time-Varying Value | Stream / IO | Chronicle | Regulator |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Execution** | Orders systems | ❌ None | ❌ None | ❌ None | Advances steps |
| **Observation** | ❌ None | Observe “now” | ❌ None | Query “as-of T” | Observe facts |
| **Transport** | ❌ None | ❌ None | Move messages | ❌ None | Orchestrates edges |
| **Memory** | ❌ None | ❌ None | ❌ None | Immutable facts | Depends on Chronicle |
| **Control** | ❌ None | ❌ None | ❌ None | ❌ None | Enforces laws |
| **Kind (math)** | Monotonic sequence | $f(t) \to V$ | Ordered sequence | $T$-indexed relation | Feedback system |
| **Correction** | Replay | Re-emit | Offset rewind | Re-eval | Replay |
| **Guarantee** | Deterministic order | Coherent observation | Delivery semantics | Time-indexed truth | Self-correction |

---

## 2. Engineering Labor Map

| Dimension | Plumbing (Crates/Deps)              | The "Real Work" (Invention) | Analog (Board / Body) |
| :--- |:------------------------------------| :--- | :--- |
| **Type** | `typenum` (**DONE**)                | **Type-to-Value Reifier** | DNA / Rated Specs |
| **Chronicle** | `uuid` (**DONE**)                   | **The Fact-Envelope Struct** | Short-Term Memory |
| **Regulator** | `bevy_ecs`, `bevy_tasks` (**DONE**) | **Cybernetic Flow Gate** | The Heart |
| **Runtime** | `bevy_app`, `bevy_ecs` (**DONE**)   | **Fact-Projection Logic** | Body / Metabolism |
| **Signal** | `futures-lite` (**DONE**)           | **Spark-Intent Mapping** | The Nerves |
| **IO** | `futures-lite` (**DONE**)           | **Pluggable SparkStream** | The Skin / Wires |
| **Versioning** | `git2` (**DONE**)                   | **Long-Term Archive Sync** | Genetic Lineage |
| **RA / LSP** | `petgraph` (**TODO**)               | **Homoiconic Self-Modification** | Brain / Librarian |

---

## 3. The Holographic Lexicon

A homoiconic dictionary mapping the creature's mind to the machine's structures.

| The "Mind" Concept | The Rust Struct | The Implementation | The Persistence |
| :--- | :--- | :--- | :--- |
| **The Fact** | `FactEnvelope<A>` | **6-Tuple Envelope** | **In-Memory / SQLite** |
| **The History** | `History<V>` | **Git Blob** | **Content-Addressable** |
| **The Spark** | `Step<Proposal>` | **Vector Envelope** | **Transient / Nerve** |
| **The Law (DNA)** | **`Fact<Proof>`** | **Type Witness** | **Self-Describing Code** |
| **Reconstitution** | **`Projector`** | **History-to-Fact Logic** | **Working Memory** |

---

## 4. Architecture: The Duality of Control and Life

### 🎛️ The Regulator (Control Flow)
* **Pure Functions**: State is a deterministic projection of history: $\Sigma_t = f(H_t)$. The runtime "forgets" by clearing the projection; it "remembers" by re-evaluating the integral of facts.
* **The 7-Tuple Envelope**: To be known, a value must occupy a coordinate in the 7-dimensional manifold:
  1.  **Entity**: The deterministic `EntityId` (UUID-v5) slot.
  2.  **Attribute**: The static `U100/U200` Aisle ID (Type-Witness).
  3.  **Value ($V$)**: The type-safe payload (The content of the whisper).
  4.  **Transaction Time ($T_x$)**: When the Regulator Heart learned the fact.
  5.  **Valid Start ($V_s$)**: When the fact became true in the world.
  6.  **Valid End ($V_e$)**: When the fact ceased to be true (The horizon of truth).
  7.  **Operation**: The algebraic intent (`Assertion` or `Retraction`).

### 🦠 The Organism (Development Stack)
Programming is the nervous system monitoring and expanding the organism.
* **Proprioception (Tower-LSP)**: Internal sensors perceiving semantic health and lexical pressure.
* **Homeostasis (Rust-Analyzer)**: The Medulla. Sequences facts and enforces the DNA (type system).
* **Metabolic Growth (LLMs)**: Effectors proposing new functions, absorbed only if they satisfy DNA invariants.

---

## 5. Reconstitution & Laws of Flow

The runtime is designed to forget. It is a high-speed projection (**Bevy ECS**) that can be wiped and reconstructed.

The system is bi-temporal. Therefore, the **Brain (RA/LSP)** can query the **Chronicle** and reconstitute working memory
to drive the task graph forward.

### ⚖️ The Laws of the Runtime
1.  **Remain lock-free** — No `Mutex` or `RwLock`.
2.  **Zero interior mutability** — No `Cell` or `RefCell`.
3.  **Total purity** — Evaluation without side effects.
4.  **Monotonic time** — $T_{n+1} > T_n$.
5.  **Lexical pressure** — Definitions live at the lowest possible scope.

---

## 6. Project Roadmap

| Feature/Module        | Status | Complexity | Implementation Path                     |
|:----------------------| :--- |:-----------|:------------------------------------------|
| **Versioning Engine** | **DONE** | Low        | `git2` foundation ready.              |
| **Identity / UUID**   | **DONE** | Low        | Deterministic `uuid` v5.              |
| **Type-Reifier**      | **DONE** | Medium     | **Static Typenum Register Map.**      |
| **Fact-Envelope**     | **DONE** | Medium     | **Generic `FactEnvelope<A: Reify>`.** |
| **Regulator Heart**   | **DONE** | High       | **The Promotion Loop.**               |
| **Nerve Foundation**  | **DONE** | Low        | `futures-lite` signals.               |
| **Graph Foundation**  | **PARTIAL** | Medium     | `petgraph` present, logic missing. |
| **IO Bridge**         | **IN PROGRESS** | Trivial    | Pluggable `SparkStream`.       |
| **Append-Only Log**   | **PLANNED** | Small      | In-memory Cerebellum for now.      |
| **Historian (Git)**   | **DORMANT** | High       | Long-term memory of itself.        |

---

### ▶️ Run

**Requirements:** 🦀 Rust (stable 2024), 🧰 Cargo, 🧠 Curiosity

```shell
cargo test
```

- Facts are stable.
- If the tests pass, the algebra holds.
- If the algebra holds, the system holds.

```shell
cargo run
```

> --- Regulator Starting ---

_The system is holding now_

### License

MIT
