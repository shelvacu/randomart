# Randomart

![seed: "randomart"](https://raw.githubusercontent.com/shelvacu/randomart/master/randomart-250x250.png)

Randomart is a program to generate cool-looking images from a seed. The image above was generated with the seed "randomart".

## How to use

`RANDOMART_SEED="your_seed_here" cargo run --release`

Note that you must re-build if you want to make a different seed. The output will be a file named `out.png` in the current directory. All other options are modified by directly modifying the code (I know); See `ANTIALIAS`, `width`, and `height` in [src/main.rs](https://github.com/shelvacu/randomart/blob/master/src/main.rs).

## How it works

`build.rs` has a list of functions that take in N colors and spit out a color. Using `format!` it builds a string that is a rust program, and then writes that string to `src/the_func.rs`. `the_func` takes in `x` and `y` as floats in the range [0,1). `main.rs` runs `the_func` for every pixel and makes an image.

## Acknowledgements

This is based off of `randomart.py` by Andrej Bauer available at [http://math.andrej.com/2010/04/21/random-art-in-python/](http://math.andrej.com/2010/04/21/random-art-in-python/).
