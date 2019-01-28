
## inning sim

This code simulates an inning of baseball. In particular it computes the number of runs scored for a set of simulations and reports the mean and standard deviation.

### command line args

The command line args available are 

* `p0` probability for a bases-on-balls

* `p1` probability for a single

* `p2` probability for a double

* `p3` probability for a triple

* `p4` probability for a home run

The code is also parameterized by the probability for the runners to take an extra base on a single or double, but these aren't currently configurable.

### example use

``` {rust}
cargo build --release
./target/release/inning_sim  --p0 0.08 --p1 0.15 --p2 0.05 --p3 0.0 --p4 0.03 --num_iter 100000000

mean run          : 0.3944 (3.55)
standard dev      : 0.9104 (8.19)
std. error on mean: 0.0001
```

As a benchmark on an eight-core system, it does 100 million innings in about 3 seconds.

``` {rust}
$ time ./target/release/inning_sim  --p0 0.08 --p1 0.15 --p2 0.05 --p3 0.0 --p4 0.03 --num_iter 100000000

mean run          : 0.3944 (3.55)
standard dev      : 0.9104 (8.19)
std. error on mean: 0.0001

real	0m2.798s
user	0m20.032s
sys	0m0.204s
```