# Garden Path

[![crates.io](https://img.shields.io/crates/v/garden-path.svg)](https://crates.io/crates/garden-path)
[![docs.rs](https://docs.rs/garden-path/badge.svg)](https://docs.rs/garden-path)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Decision tree pruning via garden metaphor — seeds, branches, pruning, grafting, and harvesting.**

---

## The Problem

Decision trees grow unboundedly. Without pruning, they overfit, become unreadable, and waste computation on low-value branches. Traditional pruning algorithms are mathematically sound but conceptually opaque — entropy, information gain, and Gini impurity are abstract metrics that don't map to intuitive mental models.

## Why This Exists

Garden Path reimagines decision tree management through a garden metaphor:
- **Seeds**: Initial decisions with scores and metadata
- **Branches**: Decision paths that grow from seeds
- **Pruning**: Remove low-value branches (like trimming a hedge)
- **Grafting**: Merge trees together (like grafting branches)
- **Harvesting**: Extract final decisions from the pruned tree

The metaphor makes decision tree operations intuitive and the API memorable.

## Architecture

```
  Seeds (initial decisions)
    │
    ▼
  ┌─────────┐
  │  Seed    │──── score > threshold? ──→ viable
  │Collection│                         → pruned
  └────┬────┘
       │ grow
       ▼
  ┌─────────┐
  │ Branch/ │──── prune low-value ──→ trimmed tree
  │ Tree    │──── graft other ──────→ merged tree
  └────┬────┘
       │ harvest
       ▼
  ┌─────────┐
  │ Decision│──── best score ──→ final choice
  └─────────┘
```

## Installation

```toml
[dependencies]
garden-path = "0.1"
```

## API Reference

### `Seed`

An initial decision with a score:

```rust
use garden_path::seed::Seed;

let s = Seed::new("classify_as_spam", 0.85);
assert!(s.is_viable(0.5));
assert!(!s.is_viable(0.9));

let mut s = Seed::new("route_a", 0.7);
s.add_metadata("source", "model_v2");
assert_eq!(s.get_metadata("source"), Some("model_v2"));
```

### `SeedCollection`

Manage and filter seeds:

```rust
use garden_path::seed::{Seed, SeedCollection};

let mut collection = SeedCollection::new();
collection.add(Seed::new("option_a", 0.9));
collection.add(Seed::new("option_b", 0.7));
collection.add(Seed::new("option_c", 0.3));

let viable = collection.viable(0.5); // only a and b
let best = collection.best(); // option_a with 0.9
let sorted = collection.sorted_by_score();
```

### Seed Operations

```rust
use garden_path::seed::Seed;

let a = Seed::new("strategy_a", 0.6);
let b = Seed::new("strategy_b", 0.8);
let merged = a.merge(&b);
// Label: "(strategy_a+strategy_b)", score: 0.7 (average)
```

## Usage Examples

### Example 1: Seed Selection

```rust
use garden_path::seed::*;

let mut seeds = SeedCollection::new();
seeds.add(Seed::new("neural_network", 0.92));
seeds.add(Seed::new("random_forest", 0.88));
seeds.add(Seed::new("logistic_regression", 0.75));
seeds.add(Seed::new("coin_flip", 0.5));

let viable = seeds.viable(0.7);
assert_eq!(viable.len(), 3);

let best = seeds.best().unwrap();
assert_eq!(best.label(), "neural_network");
```

### Example 2: Score Thresholding

```rust
use garden_path::seed::*;

let mut seeds = SeedCollection::new();
for i in 0..10 {
    seeds.add(Seed::new(&format!("opt_{}", i), i as f64 / 10.0));
}

let top = seeds.viable(0.7);
assert_eq!(top.len(), 3); // scores 0.7, 0.8, 0.9
```

### Example 3: Merging Strategies

```rust
use garden_path::seed::*;

let a = Seed::new("ensemble_a", 0.85);
let b = Seed::new("ensemble_b", 0.75);
let combined = a.merge(&b);
assert!((combined.score() - 0.8).abs() < 0.01);
```

## Modules

| Module | Description |
|--------|-------------|
| `seed` | Decision seeds and collections |
| Additional branch/prune/graft modules | Tree management |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests
4. Push and open a Pull Request
