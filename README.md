# bitboards

2D bitboards in Rust.

`bitboards` is a Rust library providing generic, compile-time generated 2D bitboards with efficient bitwise operations and optional CPU intrinsics (BMI2 when available).

It is designed for things like:

- Board games (chess-like games, tactical engines)
- Cellular automata
- Grid simulations
- ...
---

## Features

- Generic 2D bitboards (any width × height)
- Compile-time generation via procedural macro
- Automatic storage selection (`u16`, `u32`, `u64`, `u128`, or array backend)
- Precomputed masks (rows, columns, borders, etc.)
- Sliding ray generation
- Neighborhood masks (orthogonal and diagonal)
- `pext` / `pdep` acceleration (BMI2 when available)
- Zero runtime dimension cost (fully const-driven)
- Runtime bitboard (with less functionality tho)
---

## Getting Started

Add the dependencies (local path example):

```toml
[dependencies]
bitboard = { path = "../bitboard" }
bitboard_proc_macro = { path = "../bitboard_proc_macro" }
```

### Define a Bitboard
```rust
use bitboard_proc_macro::bitboard;

#[bitboard(width = 8, height = 8)]
pub struct Board8x8;
```
This generates:
 * A storage type
 * Constants:
   * WIDTH
   * HEIGHT
   * NB_SQUARES
   * Masks:
     * FULL
     * EMPTY
     * ROW_MASKS
     * COL_MASKS
     * Border masks
 * Bit manipulation methods
 * Neighbor and sliding ray helpers

### Basic Usage
```rust
let mut board = Board8x8::EMPTY;

// Set square (x, y)
board.set(3, 4);

// Check square
if board.get(3, 4) {
    println!("Bit is set!");
}

// Clear square
board.reset(3, 4);
```

---

### Masks

Row and column masks are generated automatically:

```rust
let first_row = Board8x8::ROW_MASKS[0];
let first_col = Board8x8::COL_MASKS[0];
```

Masks can be combined with bitwise operations:

```rust
let center = Board8x8::ROW_MASKS[3] & Board8x8::COL_MASKS[3];
```

---

### Neighbors

Generate orthogonal or diagonal neighbor masks:

```rust
let neighbors = board.neighbors();
```

Useful for:

- Flood fill
- Influence maps
- Movement rules
- Cellular automata

---

### Sliding Rays

Ray masks between squares are precomputed.

Useful for:

- Chess-like move generation
- Line-of-sight calculations
- Tactical engines

---

### Storage Strategy

Depending on board size:

- ≤ 16 bits   → `u16`
- ≤ 32 bits   → `u32`
- ≤ 64 bits   → `u64`
- ≤ 128 bits  → `u128`
- Larger      → `[u64; N]`

The storage type is selected automatically by the macro.

---

### Runtime Bitboard

A runtime-configurable bitboard is also available.

It offers:

- Dynamic width / height
- Basic bit manipulation
- Reduced functionality compared to compile-time boards

Use it when board dimensions are not known at compile time.

---

### Performance

- Uses native integer types when possible
- Falls back to array storage for larger boards
- Uses `pext` / `pdep` CPU intrinsics when available
- Most operations are inlined
- Compile-time boards have zero runtime dimension overhead

---

### License

This project is licensed under the GNU Lesser General Public License (LGPL).

You may redistribute and/or modify it under the terms of the GPL as published by the Free Software Foundation.

See the `LICENSE` file for details.
