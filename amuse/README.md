# Amuse

Pure-rust implementations of the [μ-law](https://en.wikipedia.org/wiki/%CE%9C-law_algorithm) and [A-law](https://en.wikipedia.org/wiki/A-law_algorithm) companding algorithms as specified in [ITU-T Recommendation G.711](https://www.itu.int/rec/T-REC-G.711-198811-I/en).

The algorithms in `amuse` are drop-in compatible with the implementations in the [ITU-T Software Tool Library (G.191)](https://github.com/openitu/STL) ([here](https://github.com/openitu/STL/tree/dev/src/g711)), but `amuse` doesn't link against those implementations (which are covered under the [ITU-T General Public License](./reference/LICENSE.md)) unless the `g191` feature is enabled.

## License

The code contained in `amuse` is licensed under the [GNU Affero General Public License v3.0](../LICENSE.md), with the exception of the reference implementation in [`./reference`](./reference), which is licensed under the [ITU-T General Public License](./reference/LICENSE.md). When the `g191` feature is enabled, the code in the `g191` module is also licensed under the I-TUT GPL.
