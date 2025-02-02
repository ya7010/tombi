# Document Tree

This crate is a library for representing the tree structure of documents using AST.

```text
ast::Root -> document_tree::DocumentTree -> document::Document
```

In the process of converting to document_tree::DocumentTree,
syntax errors such as duplicate keys and different types of data assigned to the same key are detected.
