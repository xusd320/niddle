# niddle

A super fast Node.js addon for HTML parsing and manipulation, written in Rust.

## Features

- High-performance DOM parsing and manipulation
- Exposes a simple JavaScript API via NAPI-RS
- Designed for both server-side and CLI HTML processing
- Written in Rust for speed and safety

## Installation

```bash
yarn add niddle
# or
npm install niddle
```

## Usage

```js
const { parse } = require('niddle');
const root = parse('<div id="foo" class="bar">hello <span>world</span></div>');

const div = root.select('div');
console.log(div.getAttribute('id')); // "foo"
console.log(div.text()); // "hello world"
div.setAttribute('title', 'my-title');
console.log(div.outerHtml()); // <div id="foo" class="bar" title="my-title">hello <span>world</span></div>
```

## API Documentation

### `parse(html: string): NodeRepr`

Parses an HTML string and returns a `NodeRepr` instance representing the root node.

#### Parameters

- `html` (string): The HTML content to parse.

#### Returns

- `NodeRepr`: The parsed root node.

---

### `NodeRepr` Class

Represents a DOM node and provides various manipulation methods.

#### Methods

- **append(content: string | NodeRepr): void**
  - Appends a new node or HTML string as a child.
- **appendSequence(nodes: NodeRepr[]): void**
  - Appends multiple nodes.
- **clone(): NodeRepr**
  - Clones the current node (not including descendants).
- **cloneRecursive(): NodeRepr**
  - Clones the node and all descendants.
- **getAttribute(name: string): string**
  - Gets an attribute value by name.
- **getAttributes(): Record<string, string>**
  - Gets all attributes as a key-value object.
- **getChildren(): NodeRepr[]**
  - Returns all child nodes.
- **innerHtml(): string**
  - Gets the HTML content of all descendants.
- **insertAfter(node: NodeRepr): void**
  - Inserts the current node after the specified node.
- **insertBefore(node: NodeRepr): void**
  - Inserts the current node before the specified node.
- **insertSequenceAfter(nodes: NodeRepr[]): void**
  - Inserts multiple nodes after the current node.
- **insertSequenceBefore(nodes: NodeRepr[]): void**
  - Inserts multiple nodes before the current node.
- **outerHtml(): string**
  - Gets the HTML content including the node itself.
- **prepend(content: string | NodeRepr): void**
  - Prepends a new node or HTML string as a child.
- **prependSequence(nodes: NodeRepr[]): void**
  - Prepends multiple nodes.
- **remove(): void**
  - Removes the node from the DOM.
- **removeAllAttributes(): void**
  - Removes all attributes from the node.
- **removeAttribute(name: string): void**
  - Removes an attribute by name.
- **select(selectors: string): NodeRepr**
  - Selects the first node matching the selector.
- **selectAll(selectors: string): NodeRepr[]**
  - Selects all nodes matching the selector.
- **setAttribute(name: string, value: string): void**
  - Sets an attribute value.
- **setAttributes(attrs: Record<string, string>): void**
  - Sets multiple attributes.
- **text(): string**
  - Gets the text content of the node.


## Contributing

```bash
yarn install
yarn build
yarn test
```

## Benchmark

```bash
cargo benchmark
yarn benchmark
```

---

For more usage examples and advanced API, see the source code and benchmarks in the repository.
