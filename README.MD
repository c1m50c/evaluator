# **evaluator**

![Build Status](https://img.shields.io/github/workflow/status/c1m50c/evaluator/Build?style=for-the-badge)
![Lines of Code](https://img.shields.io/tokei/lines/github/c1m50c/evaluator?style=for-the-badge)
![GitHub License](https://img.shields.io/github/license/c1m50c/evaluator?color=blue&style=for-the-badge)


Shell based application that performs lexical analysis, parsing, and evaluation to determine an output.


## **Running & Building**
<details>
<summary><strong>Running</strong></summary>
You can run the application with `cargo` by running `cargo run`, you can add the `--release` tag for a non-debug build.

```bash
$ cd evaluator
$ cargo run

evaluator:$ 5 + 3
>>> 8
```
</details>

<details>
<summary><strong>Building</strong></summary>
You can build a stand-alone application by running `cargo build`, same as running you can add the `--release` tag for a fully optimized executable.

```bash
$ cd evaluator
$ cargo build --release
...
$ ./target/release/evaluator # An example of running the application.

evaluator:$ 72 + 4 + -4
>>> 72
```
</details>


## **License**
<a href="https://github.com/c1m50c/evaluator/blob/main/LICENSE">MIT License</a>