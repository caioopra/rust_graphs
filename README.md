# Rust Graph Library

This is a comprehensive Rust library for creating, manipulating, and analyzing graphs. In addition to basic graph data structures, the library implements a variety of graph algorithms and methods. It also includes functionality to read and write graph data from and to files in specific formats.

## File Format
Graphs should be represented in the file in the following format:

### Vertices
- Start a line with `*vertices <amount of vertices>`.
- The following lines should represent vertices by `<index> <label>`.

### Edges
- Start a line with `*arcs` or `*edges`.
- The edges will be represented by `<u_index> <v_index> <weight (float)>`.

## Example File
```example.net
*vertices 3
1 A
2 B
3 C
*edges
1 2 0.5
2 3 1.0
```

## Usage

### Loading a Graph from a File

To load a graph from a file, use the `read_from_file` function. The function takes the full path to the file starting from the root of the project and returns a `Result` with the `Graph` if the file was correctly parsed, else a `&str` with the error message.

### Example

```rust
fn main() {
    match Graph::read_from_file("full/path/to/data.net") {
        Ok(graph) => {
            // Use the graph
        },
        Err(e) => {
            eprintln!("Failed to read graph: {}", e);
        }
    }
}
```