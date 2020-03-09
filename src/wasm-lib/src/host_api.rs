extern crate ndarray;

use crate::resource::{RESOURCES, AbiHandle};
use std::convert::{TryInto, From};

use wasm_bindgen::prelude::*;
use num_complex::{Complex64};

use ndarray::prelude::*;

extern "C" {
    #[link_name = "__log"]
    fn host_log(address: i32, length: i32);
}

pub fn log(msg: String) {
    unsafe {
        host_log(msg.as_ptr() as i32, msg.len() as i32);
    }
}

#[wasm_bindgen]
pub fn new() -> u32 {
    RESOURCES.push().try_into().unwrap()
}

lazy_static! {
    pub static ref A: Array2<Complex64> = {
        array![
            [Complex64::from(1.0), Complex64::from(0.0)],
            [Complex64::from(0.0), Complex64::from(1.0)],
        ]
    };
}

#[wasm_bindgen]
pub fn x(handle: u32) {
    RESOURCES.with(handle.try_into().unwrap(), |res| {
        res.data = A.dot(&res.data);
        log("called x".to_string());
    });
}

// NB: This is a silly placeholder used to test parts of the interface,
//     and should be removed soon.
#[wasm_bindgen]
pub fn render(input: &str) -> String {
    log(format!("Hello!"));

    let data = array![
        [[-1., 0., -2.], [1., 7., -3.]],
        [[1., 0., -3.], [1., 7., 5.]],
        [[1., 0., -3.], [1., 7., 5.]],
        [[2., 0., 2.], [1., 7., 2.]]
    ];

    log(format!("data = {}", data));

    return String::from(input);
}
