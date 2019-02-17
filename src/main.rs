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

extern crate palette;
extern crate lodepng;

use palette::Color;
use palette::rgb::Rgb;
use palette::encoding::Srgb as SrgbEncoding;
use palette::encoding::Linear;

type F = f64;
type FColor = Color<SrgbEncoding, F>;
type FLinRgb = Rgb<Linear<SrgbEncoding>,F>;
type FSrgbRgb = Rgb<SrgbEncoding,F>;

fn map_over<Fc: Fn(F) -> F>(a: FColor, func: Fc) -> FColor {
    let a_rgb:FLinRgb = a.into();
    FColor::linear_rgb(
        func(a_rgb.red),
        func(a_rgb.green),
        func(a_rgb.blue)
    )
}

fn map_combine<Fc: Fn(F, F) -> F>(a: FColor, b: FColor, func: Fc) -> FColor {
    let a_rgb:FLinRgb = a.into();
    let b_rgb:FLinRgb = b.into();
    FColor::linear_rgb(
        func(a_rgb.red, b_rgb.red),
        func(a_rgb.green, b_rgb.green),
        func(a_rgb.blue, b_rgb.blue),
    )
}

fn map_combine3<Fc: Fn(F, F, F) -> F>(a: FColor, b: FColor, c: FColor, func: Fc) -> FColor {
    let a_rgb:FLinRgb = a.into();
    let b_rgb:FLinRgb = b.into();
    let c_rgb:FLinRgb = c.into();
    FColor::linear_rgb(
        func(a_rgb.red, b_rgb.red, c_rgb.red),
        func(a_rgb.green, b_rgb.green, c_rgb.green),
        func(a_rgb.blue, b_rgb.blue, c_rgb.blue),
    )
}

fn map_combine_n<Fc: Fn(&mut [F]) -> F>(colors: &[FColor], func: Fc) -> FColor {
    type FVec = Vec<F>;
    let mut reds = FVec::new();
    let mut greens = FVec::new();
    let mut blues = FVec::new();
    for color in colors {
        let color_rgb:FLinRgb = color.clone().into();
        reds.push(color_rgb.red);
        greens.push(color_rgb.green);
        blues.push(color_rgb.blue);
    }
    FColor::linear_rgb(
        func(&mut reds),
        func(&mut greens),
        func(&mut blues),
    )
}

fn get_red(a: FColor) -> F {
    let a_rgb:FLinRgb = a.into();
    a_rgb.red
}

mod the_func;

fn make_array<T:Default>(size:usize) -> Vec<T> {
    let mut res = Vec::<T>::with_capacity(size);
    for _ in 0..size {
        res.push(Default::default());
    }
    return res;
}

const ANTIALIAS:usize = 1;

fn main() {
    let width = 1000usize;
    let height = 1000usize;
    let byte_depth = 3usize;
    let mut buf = make_array::<u8>(width*height*byte_depth);

    //let antialias = 2;
    let mut results = [FColor::default(); ANTIALIAS*ANTIALIAS];
    
    for x in 0..width {
        for y in 0..height {
            let pixelwidth  = 1.0  / (width as F);
            let pixelheight = 1.0 / (height as F);
            //                   /
            let tlx = (x as F)  / (width as F);
            let tly = (y as F) / (height as F);
            for division_x in 0..ANTIALIAS {
                for division_y in 0..ANTIALIAS {
                    let inner_x = tlx + ( ((division_x+1) as F) * (pixelwidth /((ANTIALIAS+1) as F)) );
                    let inner_y = tly + ( ((division_y+1) as F) * (pixelheight/((ANTIALIAS+1) as F)) );
                    results[division_x*ANTIALIAS + division_y] = the_func::the_func(inner_x, inner_y);
                }
            }
            //let res = the_func::the_func(zt1x, zt1y);
            let res = map_combine_n(&results, |cs| cs.iter().sum::<F>() / cs.len() as F);
            let components_tpl = FSrgbRgb::from(res).into_format::<u8>().into_components();
            let components_slice = vec![components_tpl.0,components_tpl.1,components_tpl.2];
            let start_idx = 3*x + 3*width*y;
            buf[start_idx..start_idx+3].copy_from_slice(&components_slice);
        }
        print!("{},", x+1);
    }
    lodepng::encode24_file("out.png", &buf, width, height).unwrap();
    println!();
}

fn well(x:F) -> F {
    F::from(1u8) - F::from(2u8) / (F::from(1u8) + x*x).powi(8)
}

fn tent(x:F) -> F {
    F::from(1u8) - F::from(2u8) * x.abs()
}
