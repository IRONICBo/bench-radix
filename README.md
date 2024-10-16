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

## Bench

1. Radix Trie

```bash
words size: 3100
trie insert             time:   [5.4861 ms 5.6228 ms 5.7743 ms]
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) high mild
  15 (15.00%) high severe

trie get                time:   [4.4201 ms 4.4354 ms 4.4564 ms]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

trie remove             time:   [9.6145 ms 9.6707 ms 9.7462 ms]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
```

2. Congee

```bash
```

## References libs

- https://crates.io/crates/rax
- https://github.com/michaelsproul/rust_radix_trie
- https://github.com/XiangpengHao/congee


