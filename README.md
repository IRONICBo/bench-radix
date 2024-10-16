# bench-radix

Test radix index for radix attention(sglang) for exisiting repos.

## Env

- Rust: cargo 1.74.0 (ecb9851af 2023-10-18)
- Machine: 6.5.0-21-generic #21~22.04.1-Ubuntu
- Python: 3.10

Generate 3000 prompts in `/data`.

```bash
$ python3 gen.py
```

## Case

Construct a 3000 string as a prompt in radix tree and then blockid i.e. 0-2999, the metrics are as follows:
- actual memory consumption is:
    - insert 3000 prompt strings
- average latency of insert data:
    - Insert 10 strings of data on top of 3000 strings of data, average latency
- average latency for match success data:
    - 3000 data base casserole, match 10 data, average latency
- 100 concurrent reads of 10 pieces of data by 100 concurrent programs
- 100 concurrent writes of 10 pieces of data by 100 concurrent programs

## References libs

- https://crates.io/crates/rax
- https://github.com/michaelsproul/rust_radix_trie
- https://github.com/XiangpengHao/congee

