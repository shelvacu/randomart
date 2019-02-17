// Copyright (c) 2019, Shelvacu
// Based on "randomart.py" which is Copyright (c) 2010, Andrej Bauer, http://andrej.com/
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate num;
extern crate rand;
extern crate rand_chacha;
extern crate sha2;

use num::traits::FloatConst;
use std::fs;
use rand::{Rng,SeedableRng};
use sha2::digest::{FixedOutput,Input};
use std::convert::AsMut;

fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}

type F = f64;

fn generate<R: Rng + ?Sized>(rng:&mut R, layers:u8) -> String {
    if layers == 0 {
        match rng.gen_range(0, 3) {
            0 => format!(
                "FColor::linear_rgb({0},{0},{0})",
                "x"
            ),
            1 => format!(
                "FColor::linear_rgb({0},{0},{0})",
                "y"
            ),
            2 => format!(
                "FColor::linear_rgb({},{},{})",
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
            ),
            _ => panic!("This shouldn't happen"),
        }
    } else {
        match rng.gen_range(0, 7) {
            0 => format!( // "Sum" (not really a sum)
                "palette::Mix::mix(&{},&{},0.5)",
                generate(rng, layers - 1),
                generate(rng, layers - 1),
            ),
            1 => format!( // "Product"
                "map_combine({},{},|a,b| a*b)",
                generate(rng, layers - 1),
                generate(rng, layers - 1),
            ),
            // "Mod" seems dumb (always zero?) so I'm skipping it
            2 => format!( // "Well"
                "map_over({}, well)",
                generate(rng, layers - 1),
            ),
            3 => format!( // "Tent"
                "map_over({}, tent)",
                generate(rng, layers - 1),
            ),
            4 => format!( // "Sin"
                "map_over({}, |c| ({} + {} * c).sin())",
                generate(rng, layers - 1),
                rng.gen_range(F::from(0u8), F::PI()), // phase (supposedly)
                rng.gen_range(F::from(1u8), F::from(6u8)), // frequency (allegedly)
            ),
            5 => format!( // "Level"
                "map_combine3({}, {}, {}, |level, e1, e2| if level < {} {{ e1 }} else {{ e2 }})",
                generate(rng, layers - 1),
                generate(rng, layers - 1),
                generate(rng, layers - 1),
                rng.gen_range(F::from(0u8), F::from(1u8)),
            ),
            6 => format!( // "Mix" mix but with dynamic weight
                "palette::Mix::mix(&{},&{},get_red({}))",
                generate(rng, layers - 1),
                generate(rng, layers - 1),
                generate(rng, layers - 1),
            ),
            _ => panic!("This shouldn't happen"),
        }
    }
}


fn main() {
    let mut hasher = sha2::Sha256::default();
    let string_seed = String::from("phantom thieves");
    eprintln!("Seed: {:?}", string_seed);
    hasher.input(string_seed.into_bytes());
    let hash_res_generic_array = hasher.fixed_result();
    let hash_res_slice = hash_res_generic_array.as_slice();
    let hash_res_array:[u8; 32] = copy_into_array(hash_res_slice);
    //let mut rng = rand_chacha::ChaChaRng::seed_from_u64(9685424495u64); //TODO
    let mut rng = rand_chacha::ChaChaRng::from_seed(hash_res_array);
    let func_inner = generate(&mut rng, 10);

    let file_contents = format!(r#"
#[allow(unused_imports)]
use crate::{{F,FColor,map_over,map_combine,map_combine3,get_red,well,tent}};

pub fn the_func(x: F, y: F) -> FColor {{
    {}
}}
"#, func_inner);
    fs::write("src/the_func.rs", file_contents).unwrap();
}
