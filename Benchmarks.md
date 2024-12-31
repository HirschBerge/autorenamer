# Benchmarks
## Criteria

Note: `time` is an alias to hyperfine.

The Benchmark is performed on 50k empty files that match the "Episode \d+" regex. The preparation script simply deletes all files and regenerates them.
### My Previous ***shudders*** Bash script for renaming...
```bash
time --prepare '../benchmarks.sh' '~/.scripts/autorenamer.sh 3'
Benchmark 1: ~/.scripts/autorenamer.sh 3
  Time (mean ± σ):     53.377 s ±  0.427 s    [User: 18.885 s, System: 35.567 s]
  Range (min … max):   52.990 s … 54.423 s    10 runs
```

### Release v1.0.2


```bash
time --prepare './benchmarks.sh' 'autorenamer -p test/ -s 3'              
Benchmark 1: autorenamer -p test/ -s 3
  Time (mean ± σ):      2.183 s ±  0.091 s    [User: 1.573 s, System: 0.580 s]
  Range (min … max):    2.065 s …  2.338 s    10 runs
```
### Comparison between Bash and v1.0.2

```bash
time --prepare '../benchmarks.sh' '~/.scripts/autorenamer.sh 3' 'autorenamer -p ../test/ -s 3'
Benchmark 1: ~/.scripts/autorenamer.sh 3
  Time (mean ± σ):     53.858 s ±  0.583 s    [User: 19.074 s, System: 35.896 s]
  Range (min … max):   53.040 s … 55.017 s    10 runs
 
Benchmark 2: autorenamer -p ../test/ -s 3
  Time (mean ± σ):      2.629 s ±  0.146 s    [User: 1.786 s, System: 0.774 s]
  Range (min … max):    2.430 s …  2.922 s    10 runs
 
Summary
  autorenamer -p ../test/ -s 3 ran
   20.48 ± 1.16 times faster than ~/.scripts/autorenamer.sh 3
```
I guess you could say it's pretty quick.

### Using Divan to get best timing via cargo
```bash
Timer precision: 20 ns
auto             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_rename                │               │               │               │         │
   ├─ 1000       2.744 ms      │ 27.76 ms      │ 2.821 ms      │ 3.101 ms      │ 100     │ 100
   ├─ 10000      25.74 ms      │ 287.7 ms      │ 27.03 ms      │ 29.81 ms      │ 100     │ 100
   ├─ 20000      51.93 ms      │ 581.6 ms      │ 56.32 ms      │ 61.68 ms      │ 100     │ 100
   ├─ 30000      81.62 ms      │ 884.3 ms      │ 90.08 ms      │ 97.52 ms      │ 100     │ 100
   ├─ 40000      106.7 ms      │ 1.245 s       │ 112.7 ms      │ 124.1 ms      │ 100     │ 100
   ╰─ 50000      136.2 ms      │ 1.45 s        │ 148.9 ms      │ 161.3 ms      │ 100     │ 100
```
