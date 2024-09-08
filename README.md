# EasyC
A superset of C for correct and concise code with high level features
Written on-top of https://github.com/vickenty/lang-c

## Features
- Sum type enums with 'if let' statement for value extraction
- Match statements as syntactic sugar for switch statements (avoids fallthrough)
- Defer statements for cleanup code
- If expressions
   (currently only implemented for assign operations)
- For loop syntactic sugar
   (currently only implemented for forward iteration)

## Initial build
```bash
make
```
## Run example
````bash
cargo run --bin dump ./main.c | astyle
````

## How to use
```bash
cargo run --bin dump /path/to/file.c
```
Additionally pass through 'astyle' command line C formatter
```bash
cargo run --bin dump /path/to/file.c | astyle
```

Write the code to a file
```bash
cargo run --bin dump /path/to/file.c > /path/to/output.c
```



### Current issues
- Due to the implementation of the underlying C parser, preprocessor statements must be stripped (and reinserted afterwards) (preferably using a script)
) before running the code through the EasyC parser.
- Other preprocessor directives such as #define are also not supported.
- Span position offset issues on certain characters
