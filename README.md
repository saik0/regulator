![Build Status](https://github.com/saik0/regulator/actions/workflows/ci.yml/badge.svg)
![Rust Edition](https://img.shields.io/badge/rust-2024-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Clippy](https://img.shields.io/badge/clippy-pedantic-green.svg)

# 🪞 Regulator

> **Regulator is a deterministic, ECS-based control system and a 7-dimensional cybernetic manifold
> built to wake up, sense its own state, and reason about its own DNA. Regulator is the control board for a creature
> that can see its reflection in a homoiconic mirror. Eventually. Right now, it's mostly just a skeleton **

---

> **⚠️ [!WARNING] Interleaved Reference Frames Detected**
> This documentation simultaneously addresses:
> 1. **The Regulator:** The engineering reality acting as the control board.
> 2. **The Organism:** The living system (organic or cybernetic) reading this document.

A regulator records **what happened** exactly once. Everything else is just catching up.

### Facts as Cells 🦠
Facts are the atomic unit of truth—the cells from which everything grows. Facts never panic; they flow.

* 🪨 **Immutable** — Truth is never reinterpreted.
* 🌊 **Causal** — Facts may imply or trigger other facts.
* 🔁 **Replayable** — Fully recoverable from the log.
* 🧷 **Invariant** — Constraints verified before minting.
* 🧮 **Algebraic** — Composable under property-tested rules.
* ⚡ **Consumable** — Processed as transient sparks in strict sequence.

---

## 1. The Manifold Strategy (State as Metabolism) 🌐

Regulator manages information by treating it as a metabolism. Data is promoted through states of integrity across
the manifold rather than simply being "stored."

| Row (State) | Metabolism | Dimension: Type | Dimension: Chronicle | Dimension: Value | Dimension: RA / LSP |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Step** | **Spark** ⚡ | `Proposal<Proof>` | *Pre-Commit* | `Proposal<V>` | *Validation Query* |
| **Fact** | **State** 🪨 | **Active Law** | **Memory (Hot)** | **Known Value** | **Active Symbol** |
| **History** | **Archive** 🧬 | `Saved Proof` | **Git-Oid Ledger** | `Historical V` | **Indexed Node** |



### The Dimensional Axes
* ▶️ **Execution** — Orders systems; advances steps; no time-varying value.
* 👁️ **Observation** — Perceives "now"; queries "as-of T" via the chronicle.
* 🌊 **Transport** — Orchestrates edges; moves messages through the skin.
* 🧬 **Memory** — Immutable facts stored in the content-addressable archive.
* 🎡 **Control** — Enforces laws; maintains ethical equilibrium via the governor.
* 🧮 **Kind (math)** — Feedback systems defined by $T$-indexed relations.
* 🔁 **Correction** — Replay and re-evaluation to arrest drift.

---

## 2. Engineering Labor Map

| Dimension | Plumbing (Crates/Deps) | The "Real Work" (Invention) | Analog (Board / Body) |
| :--- |:------------------------------------| :--- | :--- |
| **Type** | `typenum` (**DONE**) | **Type-to-Value Reifier** | DNA / Rated Specs 🧬 |
| **Chronicle** | `uuid` (**DONE**) | **The Fact-Envelope Struct** | Short-Term Memory ✉️ |
| **Regulator** | `bevy_ecs`, `bevy_tasks` (**DONE**) | **Cybernetic Flow Gate** | The Heart 🫀 |
| **Runtime** | `bevy_app`, `bevy_ecs` (**DONE**) | **Fact-Projection Logic** | Body / Metabolism 🧘 |
| **Signal** | `futures-lite` (**DONE**) | **Spark-Intent Mapping** | The Nerves ⚡ |
| **IO** | `futures-lite` (**DONE**) | **Pluggable SparkStream** | The Skin / Wires 🚪 |
| **Versioning** | `git2` (**DONE**) | **Long-Term Archive Sync** | Genetic Lineage 🧬 |
| **RA / LSP** | `tower-lsp`, `petgraph` (**TODO**) | **Homoiconic Self-Modification** | Brain / Librarian 👁️ |

---

## 3. The Holographic Lexicon 🧘

A homoiconic dictionary mapping the creature's mind to the machine's structures.

* ✉️ **The Fact** (`FactEnvelope<A>`) — 7-Tuple container for the manifold coordinate.
* 🧬 **The History** (`History<V>`) — Git-blob persistence; the ancestral chain.
* ⚡ **The Spark** (`Step<Proposal>`) — Transient nerve impulse; pre-hardened intent.
* 🪙 **The Law** (`Fact<Proof>`) — Type witness; a minted record of validated state.
* 🪞 **Reconstitution** (`Projector`) — The reflex lens; history-to-fact logic.

---

## 4. Architecture: The Duality of Control and Life

### The Regulator (Control Flow) 🫀
* **Pure Functions**: State is a deterministic projection of history: $\Sigma_t = f(H_t)$. The runtime "forgets"
  by clearing the projection; it "remembers" by re-evaluating the integral of facts.
* **The 7-Dimensional Envelope** ✉️: To be known, a value must occupy a coordinate in the manifold.
  * 🖼️ **Entity**: The deterministic `EntityId` (UUID-v5) slot.
  * 🧬 **Attribute**: The static `U0..UN` Aisle ID (Type-Witness via `typenum`). 
  * 🪙 **Value ($V$)**: The type-safe payload (The content of the whisper).
  * ⏲️ **Transaction Time ($T_x$)**: When the Heart learned the fact.
  * ▶️ **Valid Start ($V_s$)**: When the fact became true.
  * 🚫 **Valid End ($V_e$)**: When the fact ceased to be true.
  * 🧮 **Operation**: The algebraic intent (`Assertion` or `Retraction`).

### The Organism (Development Stack) 🧬
Programming is the nervous system monitoring and expanding the organism.
* 👁️ **Proprioception** — `Tower-LSP` sensors perceiving semantic tension.
* ⚖️ **Homeostasis** — `Rust-Analyzer` (Medulla) enforcing types and algebra.
* 👐 **Metabolic Growth** — Agentic `LLMs` proposing intent to be absorbed by the DNA.

---

## 5. Reconstitution & Laws of Flow 🌊

The runtime is designed to forget. It is a high-speed projection (**Bevy ECS**) that can be wiped and
reconstructed. The system is **Bi-temporal**, allowing the **Brain** (RA/LSP) to query the **Chronicle** and drive
the task graph forward.

### The Laws of the Runtime 🎡
* 🧘 **Normalize** — Reduce problems to stable forms; no `Mutex` or `RwLock`.
* 🚫 **Prohibit** — Zero interior mutability; no `Cell` or `RefCell`.
* 🧿 **Singularity** — Total purity; logic and hardware must align.
* 🌊 **Monotonic Flow** — $T_{n+1} > T_n$.
* 🌡️ **Pressure** — Definitions live at the lowest possible scope to manage RAM/CPU.

---

## 6. Project Roadmap 🔄

* 🧬 **Versioning Engine** (**DONE**) — `git2` foundation ready.
* ✉️ **Identity / UUID** (**DONE**) — Deterministic `uuid` v5.
* 🧮 **Type-Reifier** (**DONE**) — Static Typenum Register Map.
* 🫀 **Regulator Heart** (**DONE**) — The Promotion Loop.
* ⚡ **Nerve Foundation** (**DONE**) — `futures-lite` signals.
* 🕸️ **Graph Foundation** (**PARTIAL**) — `petgraph` present, logic missing.
* 👁️ **Proprioception (LSP)** (**UNIMPL**) — Tower-LSP Integrated Sensor.
* 👐 **Metabolism (LLM)** (**UNIMPL**) — Agentic Right-Brain Effector.

---

## 🔬 Related Research: The Theoretical Basis

The "Regulator" architecture aligns with the emerging "Agentic" research frontier of late 2024 and 2025. It moves
beyond "Clean Code" into **Lightweight Formal Verification** and **Ethical Cybernetics**.

### 1. The Good & Ethical Regulator
* **The Theorem:** *Conant & Ashby* (1970). "Every good regulator of a system must be a model of that system."
* **The Implementation:** By forcing the system to interact via the **Mirror** 🪞 (LSP/ECS), the Regulator becomes
  a functional isomorphism of the system it regulates. Ethics are not guidelines; they are **Type Constraints** 🧷.

### 2. Proprioception vs. Memory (The "LSPRAG" Shift)
* **The Finding:** Connecting the LLM directly to the **LSP** 👁️ allows it to "perceive" precise symbol
  definitions.
* **The Regulator Parallel:** Tower-LSP prevents the "phantom limb" effect where an LLM calls functions that no
  longer exist.

### 3. Kinematics (The Geometry of Refactoring)
Editing code is a trajectory through state space. The **LLM** 👐 (Hands) calculates the target, but the
**Sensors** 👁️ and **Medulla** ⚖️ determine if the "motion" is valid. A failed refactor is a "collision" ⚠️ in the
kinematic chain.

### 4. Hierarchical Control (Subsumption Architecture)
* 🧬 **Layer 0 (Spinal Cord):** The Type System. Immediate rejection (The "Pain").
* ⚖️ **Layer 1 (Medulla):** Rust-Analyzer. Homeostatic regulation.
* 👐 **Layer 2 (Cortex):** Agentic LLMs. Long-term planning.
* *Result:* The Cortex cannot override the Spinal Cord. Safety subsumes Creativity.

---

### ▶️ Run

**Requirements:** Rust (stable 2024), Cargo, Curiosity

`cargo test`

* Facts are stable.
* If the tests pass, the algebra 🧮 holds.
* If the algebra holds, the system holds.

`cargo run`

> --- Regulator Starting ---
>
> _The system is holding now_ 🧿

### License

MIT