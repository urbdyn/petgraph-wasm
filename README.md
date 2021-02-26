# petgraph-wasm

[![NPM Version][npm-image]][npm-url]

A work in progress, selective WASM wrapper around the fantastic [petgraph](https://github.com/petgraph/petgraph) library in Rust.

This project aims to provide a direct port of the petgraph library to an NPM packages which preserves as much of the original API structure and design as possible.
For more details please check out the [thoroughly documented petgraph API](https://docs.rs/petgraph/0.5.1/petgraph/).

## Example

```typescript
// Typescript example

import {DiGraph, toposort} from 'petgraph-wasm'

// Create new directional graph
const g = new DiGraph()

// Add nodes to directional graph
const kno_index = g.addNode("Knoxville")
const vil_index = g.addNode("Vilnius")
const tai_index = g.addNode("Taipei")

// Connect them with edges
g.addEdge(kno_index,vil_index)
g.addEdge(kno_index,tai_index)
g.addEdge(vil_index,tai_index)

// Sort them
const sorted_g = toposort(g)

// Detect cycles
g.addEdge(tai_index,kno_index)
//   Will throw error!
toposort(g)
```

## Performance

You should always measure you're exact needs to know how this library will work for you.
But here's a few order of magnitude examples as run on a GCP VM with a 2.25GHz AMD EPYC CPU. 
There are probably overly "optomistic" in design as compared to real world needs.

| Action     | Nodes     | Edges      | Time          |
|------------|-----------|------------|---------------|
| `toposort` | 10,000    | 9,000      | ~2ms          |
| `toposort` | 10,000    | 90,000     | ~5ms          |
| `toposort` | 10,000    | 900,000    | ~20ms         |
| `toposort` | 100,000   | 99,000     | ~20ms         |
| `toposort` | 100,000   | 990,000    | ~70ms         |
| `toposort` | 100,000   | 900,000    | ~600ms        |
| `toposort` | 1,000,000 | 999,000    | ~350ms        |
| `toposort` | 1,000,000 | 9,990,000  | ~750ms        |
| `toposort` | 1,000,000 | 99,900,000 | out-of-memory |


## Development

To work on this you will need to install rust-up and wasm-pack.

```bash
# Build the npm package
wasm-pack build --target nodejs
# Test on node
wasm-pack test --node
# Create release build
./bin/ci.sh
# Try out benchmark of 100,000 nodes each with 15 edges
time ./example_js/benchmark.js 100000 15
```

[npm-image]: https://img.shields.io/npm/v/@urbdyn/petgraph-wasm.svg
[npm-url]: https://npmjs.org/package/@urbdyn/petgraph-wasm
