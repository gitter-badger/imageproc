
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;
extern crate conv;
extern crate image;
extern crate itertools;

#[macro_use]
extern crate nalgebra;
extern crate num;
extern crate quickcheck;
extern crate rand;

#[macro_use]
pub mod utils;
pub mod affine;
pub mod contrast;
pub mod corners;
pub mod definitions;
pub mod drawing;
pub mod edges;
pub mod filter;
pub mod gradients;
pub mod haar;
pub mod hog;
pub mod stats;
pub mod integralimage;
pub mod localbinarypatterns;
pub mod map;
pub mod math;
pub mod multiarray;
pub mod noise;
pub mod pixelops;
pub mod rect;
pub mod regionlabelling;
pub mod suppress;
pub mod unionfind;
