# Fast SMT (Sparse merkle tree)

A zkVM-friendly, fast Sparse Merkle Tree (SMT) library in Rust, using TurboSHAKE128 as default hash function.

> [!NOTE]
> This library crate is a fork of <https://github.com/namada-net/sparse-merkle-tree>.

| size | proof size | update | get | merkle proof | verify proof |
| --- | --- | --- | --- | --- | --- |
| 2n + log(n) | log(n) | log(n) | log(n) | log(n) | log(n) |

Features:

- Generate / Verify multi-leaves merkle proof.
- Customizable hash function. Provides default hasher implementation using both TurboSHAKE128 and BLAKE3.
- Rust `no_std` support. zkVM friendly.

This article describes details of the tree [An optimized compacted sparse merkle tree](https://justjjy.com/An-optimized-compact-sparse-merkle-tree).

## Introduction

A sparse merkle tree is a perfectly balanced tree contains `2 ^ N` leaves:

``` txt
# N = 256 sparse merkle tree

height:
255                0
                /     \
254            0        1

.............................

           /   \          /  \
2         0     1        0    1
1        / \   / \      / \   / \
0       0   1 0  1 ... 0   1 0   1 
       0b00..00 0b00..01   ...   0b11..11
```

The above graph demonstrates a sparse merkle tree with `2 ^ 256` leaves, which can map every possible `H256` value into the leaves. The height of the tree is `256`, from top to bottom. At every intermediate node, we denote `0` for the left branch and denote `1` for the right branch. So we can get a 256-bit path, from root of the tree to any leaf. We use the path as the key for the corresponding leaves - the left most leaf's key is `0b00..00`, and the next leaf's key is `0b00..01`, while the the right most leaf's key is `0b11..11`.

We use a root of data-type `H256` and a hash-map of data-type `map[(usize, H256)] -> (H256, H256)` to represent a tree, the key of map is parent node and height, values are children nodes, an empty tree represented in an empty map plus a zero `H256` root.

To update a `key` with `value`, we walk down from `root`, push every non-zero sibling into `merkle_path` vector, since the tree height is `N = 256`, we need store 256 siblings. Then we reconstruct the tree from bottom to top: `map[(height, parent)] = merge(lhs, rhs)`, after doing it 256 times, we get the new Merkle `root`.

A sparse merkle tree contains few efficient nodes, and lot's of zero nodes, we can specialize the `merge` function for zero value. We redefine the `merge` function, only do the actual computing when `lhs` and `rhs` are both non-zero values, otherwise if one of them is zero, we just return another one as the parent.

``` rust
fn merge(lhs: H256, rhs: H256) -> H256 {
    if lhs.is_zero() {
        return rhs;
    } else if rhs.is_zero() {
        return lhs;
    }

    // Only do actual computing when lhs and rhs both are non-zero.
    merge_hash(lhs, rhs)
}
```

This optimized `merge` function still has one problem, `merge(x, zero)` equals to `merge(zero, x)`, which means the merkle `root` is broken, we can easily construct a conflicted merkle `root` from different leaves. To fix this, instead of updating `key` with just an `value` of type `H256`, we use `hash(prefix || key || value)` as the leaf value. So for different keys, no matter what the `value` is, the leaf hashes are always unique. Since all leaves have a unique hash, nodes at each height will fall in one of following category.

- Either merged by two different hashes.
- Or be merged by a hash and a zero-value; for a non-zero parent.

In either situation we get a unique hash at the parent's height. Until the root, if the tree is empty, we get zero, or if the tree is not empty, the root must merge from two hashes or a hash and a zero-value. Hence the the root hash is guaranteed to be unique.

## Prerequisites

Rust stable toolchain; see <https://rustup.rs> for installation guide. MSRV for this library crate is **1.88.0**.

```bash
# While developing this library, I was using
$ rustc --version
rustc 1.91.1 (ed61e7d7e 2025-11-07)
```

## Testing

For ensuring functional correctness of Sparse Merkle Tree operations, this library crate includes a comprehensive testing suite. Run all the tests by issuing following command from the root of the project repository.

```bash
make test
```

### Code Coverage

To generate a detailed code coverage report in HTML format, use [cargo-tarpaulin](https://github.com/xd009642/tarpaulin):

```bash
# Install cargo-tarpaulin, if not already installed
cargo install cargo-tarpaulin
make coverage
```

```bash
Coverage Results:
|| Tested/Total Lines:
|| benches/sparse_merkle_tree.rs: 0/20
|| src/blake3_hasher.rs: 7/7
|| src/default_store.rs: 8/32
|| src/error.rs: 0/7
|| src/h256.rs: 17/41
|| src/internal_key.rs: 25/69
|| src/lib.rs: 0/2
|| src/merge.rs: 9/17
|| src/merkle_proof.rs: 93/176
|| src/traits.rs: 0/6
|| src/tree.rs: 91/198
|| 
43.48% coverage, 250/575 lines covered
```

This will create an HTML coverage report at `tarpaulin-report.html` that you can open in your web browser to view detailed line-by-line coverage information for all source files.

## Benchmarking

For evaluating the performance of this library crate with different input sizes and hash functions, run following command.

```bash
make bench
```

> [!WARNING]
> When benchmarking, make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I find <https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md> helpful.

## Usage

To include `fast-sparse-merkle-tree` library crate in your Rust project, add it as a dependency in your `Cargo.toml`.

```toml
[dependencies]
fast-sparse-merkle-tree = "=0.1.1"
# or (minimal, just `turboshake` for faster hashing, no_std)
fast-sparse-merkle-tree = { version = "=0.1.1", default-features = false, features = ["turboshake"] }
```

See [smt_example.rs](./examples/sparse_merkle_tree.rs) example program which demonstrates main functionality of SMT. Run it with `$ make example`.
