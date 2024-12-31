# Benchmarks
## Disclaimers
These benchmarks are run on a system running btrfs. Running ext4 on a much weaker laptop provides faster times. Increasingly so the larger the benchmark is. I would not consider the difference to be noticable outside of benchmarking at large volumes.
## Criteria
  
  
The Benchmark is performed on 50k empty files that match the "Episode \d+" regex. The preparation script simply deletes all files and regenerates them.
### My Previous ***shudders*** Bash script for renaming...

| Command | Mean [s] | Min [s] | Max [s] |
|---|---|---|---|
| `~/.scripts/autorenamer.sh 3` | 53.377 ± 0.427 | 52.990 | 54.423 |

### Release v1.0.2

| Command | Mean [s] | Min [s] | Max [s] |
|---|---|---|---|
| `autorenamer -p test/ -s 3` | 2.183 ± 0.091 | 2.065 | 2.338 |

### Comparison between Bash and v1.0.2
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/autorenamer --season 69 --path ./test` | 1.779 ± 0.032 | 1.743 | 1.842 | 1.00 |
| `./slow.sh 69` | 89.713 ± 0.162 | 89.510 | 90.067 | 50.41 ± 0.92 |

I guess you could say it's pretty quick.

### Using Divan to get best timing via cargo
Timer precision: 20 ns
| runs | fastest   | slowest   | median   | mean    | samples | iters |
|---|---|---|---|---|---|---|
| 1000      | 2.744 ms  | 27.76 ms  | 2.821 ms  | 3.101 ms  | 100     | 100   |
| 10000     | 25.74 ms  | 287.7 ms  | 27.03 ms  | 29.81 ms  | 100     | 100   |
| 20000     | 51.93 ms  | 581.6 ms  | 56.32 ms  | 61.68 ms  | 100     | 100   |
| 30000     | 81.62 ms  | 884.3 ms  | 90.08 ms  | 97.52 ms  | 100     | 100   |
| 40000     | 106.7 ms  | 1.245 s   | 112.7 ms  | 124.1 ms  | 100     | 100   |
| 50000     | 136.2 ms  | 1.45 s    | 148.9 ms  | 161.3 ms  | 100     | 100   |
