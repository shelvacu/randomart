// Copyright (c) 2019, Shelvacu
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

//use rand::Rng;
use palette::Color;
use palette::rgb::Rgb;
use palette::encoding::Srgb as SrgbEncoding;
use palette::encoding::Linear;

type F = f64;
type FColor = Color<SrgbEncoding, F>;
type FLinRgb = Rgb<Linear<SrgbEncoding>,F>;
type FSrgbRgb = Rgb<SrgbEncoding,F>;
//type U8SrgbRgb = Rgb<SrgbEncoding,u8>;

//let color: Color = Srgb::new(0.8, 0.2, 0.1).into_linear().into();
//let lighter = color.lighten(0.1);
//let desaturated = color.desaturate(0.5);
//https://crates.io/crates/palette

/*
trait A

trait Arity0 {
    fn make_func(&self) -> Box<Fn(F,F) -> FColor>;
}
trait Arity1 {
    fn make_func(&self) -> Box<Fn(F,F,FColor) -> FColor>;
}
trait Arity2 {
    fn make_func(&self) -> Box<Fn(F,F,FColor,FColor) -> FColor>;
}
trait Arity3 {
    fn make_func(&self) -> Box<Fn(F,F,FColor,FColor,FColor) -> FColor>;
}

struct VariableX();
impl Arity0 for VariableX {
    fn make_func(&self) -> Box<Fn(F,F) -> FColor> {
        Box::new(|x,y| FColor::linear_rgb(x,x,x))
    }
}

struct VariableY();
impl Arity0 for VariableY {
    fn make_func(&self) -> Box<Fn(F,F) -> FColor> {
        Box::new(|x,y| FColor::linear_rgb(y,y,y))
    }
}
*/
//struct Constant

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

fn get_red(a: FColor) -> F {
    let a_rgb:FLinRgb = a.into();
    a_rgb.red
}

mod the_func;
//fn the_func(x: F, y: F) -> FColor { map_combine3(map_over(map_over(map_over(FColor::linear_rgb(0.6675707,0.25308847,0.56201494), well), well), well), map_over(map_over(palette::Mix::mix(&FColor::linear_rgb(0.043018043,0.9635114,0.19234258),&FColor::linear_rgb(0.29156852,0.711145,0.13020879),get_red(FColor::linear_rgb(y,y,y))), well), well), map_combine3(map_combine3(map_over(FColor::linear_rgb(x,x,x), well), map_over(FColor::linear_rgb(x,x,x), tent), map_combine3(FColor::linear_rgb(x,x,x), FColor::linear_rgb(x,x,x), FColor::linear_rgb(y,y,y), |level, e1, e2| if level < 0.41037297 { e1 } else { e2 }), |level, e1, e2| if level < 0.41948104 { e1 } else { e2 }), map_combine(map_over(FColor::linear_rgb(y,y,y), well),map_combine(FColor::linear_rgb(0.6310579,0.7069581,0.1531983),FColor::linear_rgb(0.88433623,0.88487315,0.9737127),|a,b| a*b),|a,b| a*b), map_combine(palette::Mix::mix(&FColor::linear_rgb(y,y,y),&FColor::linear_rgb(x,x,x),0.5),map_over(FColor::linear_rgb(x,x,x), well),|a,b| a*b), |level, e1, e2| if level < 0.27216887 { e1 } else { e2 }), |level, e1, e2| if level < 0.8572339 { e1 } else { e2 }) }

fn make_array<T:Default>(size:usize) -> Vec<T> {
    let mut res = Vec::<T>::with_capacity(size);
    for _ in 0..size {
        res.push(Default::default());
    }
    return res;
}

fn main() {
    let width = 1000usize;
    let height = 1000usize;
    let byte_depth = 3usize;
    let mut buf = make_array::<u8>(width*height*byte_depth);
    
    for x in 0..width {
        for y in 0..height {
            let zt1x = ( (x as F) / (width  as F) ) + ( 1.0 / (2.0*(width  as F)) );
            let zt1y = ( (y as F) / (height as F) ) + ( 1.0 / (2.0*(height as F)) );
            //println!("{},{}",zt1x,zt1y);
            let res = the_func::the_func(zt1x, zt1y);
            let components_tpl = FSrgbRgb::from(res).into_format::<u8>().into_components();
            let components_slice = vec![components_tpl.0,components_tpl.1,components_tpl.2];
            let start_idx = 3*x + 3*width*y;
            buf[start_idx..start_idx+3].copy_from_slice(&components_slice);
        }
    }
    lodepng::encode24_file("out.png", &buf, width, height).unwrap();
    //println!("{}", generate(&mut rng, 4));
    //let a = FColor::linear_rgb(1.0f32,0.0f32,1.0f32);
    //let b = FColor::linear_rgb(0.0f32,1.0f32,0.0f32);
    //let avg = average_rgb(a,b);
    //println!("{:?}",FSrgbRgb::from(avg).into_format::<u8>());
    //print_type_of(&a);
    //let () = a;
    //println!("a = {:?} and b = {:?}", a, b);
    //println!("avg({:?},{:?}) = {:?}",a,b,average_rgb(a,b));
}

fn well(x:F) -> F {
    F::from(1u8) - F::from(2u8) / (F::from(1u8) + x*x).powi(8)
}

fn tent(x:F) -> F {
    F::from(1u8) - F::from(2u8) * x.abs()
}

/*///dont use. Just use palette::Mix::mix
fn average_rgb_weighted(a:FColor, b:FColor, weight:F) -> FColor {
    let w = num::clamp(weight, 0.0, 1.0);
    let iw = 1.0 - w;
    let a_rgb:FLinRgb = a.into();
    let b_rgb:FLinRgb = b.into();
    let res = FLinRgb::new(
        a_rgb.red   * w + iw * b_rgb.red,
        a_rgb.green * w + iw * b_rgb.green,
        a_rgb.blue  * w + iw * b_rgb.blue,
    );

    return res.into()
    
}

fn average_rgb(a:FColor, b:FColor) -> FColor {
    return average_rgb_weighted(a,b,0.5)
}
*/
