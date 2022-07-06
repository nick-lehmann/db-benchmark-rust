# Benchmarks

## Variables 

- Type of data storage: Row, Column, PAX (tba)
- Type of retrieval: Scalar, Vectorised, Vectorised + strided access (tba)
- Number of rows: 16 * 100; 16 * 1000; 16 * 10,000
- Number of filters: 1, 2, 4, 8, 16
- Number of returned arguments: tba


## TODO

- [ ] Implement benchmark for 64bit integers
- [ ] `RowStore` expects a multiple of the `chunk_size`, which is 16 for AVX512 with 32bit integers
