
This crate estimates the FiveThirtyEight card-collecting riddler for [Sept. 7, 2018](https://fivethirtyeight.com/features/id-like-to-use-my-riddler-lifeline/). 

It uses the `rayon` crate to implement a parallel map. 

Example use:


``` 
# download and compile
git clone git@github.com:bdilday/bdrustfun.git
cd bdrustfun/cards_permutations/ 
cargo build --release
```

``` 
# run it and get usage
./target/release/cards_permutations --help

card_permutations 0.1
how many packs of cards to complete the set?

USAGE:
    cards_permutations --num_cards <num_cards> --num_iter <num_iter> --num_pool <num_pool>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --num_cards <num_cards>    number of cards per pack
    -n, --num_iter <num_iter>      number of simulation iterations
    -p, --num_pool <num_pool>      number of cards in set (the pool)
``` 

You can switch:

* number of cards per pack (`c`)

* number of cards in set (`p`)

* number of simulation iterations (`n`)

example, 

```
./target/release/cards_permutations -c 10 -p 100 -n 1000000

mean packs bought : 49.9248
standard dev      : 12.0053
std. error on mean: 0.0120
```