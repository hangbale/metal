# write a javascript engine in rust

Because a full implementation is too difficult,
I start up this is project with some goals that are easy to implement
but still cover all the important techniques.

## goals
- buid in rust
- a subset of javascript grammar


## usage
```
cargo run path_of_js_file.js
```

## task board

### hand written lexer
- [x] a basic lexer
- [x] functions to check identifier, especially unicode chars and sequence
- [x] handle string literal
- [x] opt the get postion method
- [x] numeric literal
- [ ] refactoring

### hand written parser(top-down)
- [x] basis of parser and AST