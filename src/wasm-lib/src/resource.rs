extern crate ndarray;
extern crate uuid;

use std::sync::{Mutex};

use num_complex::{Complex, Complex64};

use ndarray::prelude::*;
use uuid::Uuid;

lazy_static! {
    pub static ref RESOURCES: Mutex<Vec<Resource>> = {
        Mutex::new(Vec::new())
    };
}

pub trait AbiHandle {
    fn push(&self) -> usize;
    fn with<F, T>(&self, index: usize, func: F) -> Option<T>
    where F: FnOnce(&mut Resource) -> T;
}

impl AbiHandle for Mutex<Vec<Resource>> {
    fn push(&self) -> usize {
        let mut data = self.lock().unwrap();
        data.push(Resource::new());
        data.len() - 1
    }
    fn with<F, T>(&self, index: usize, func: F) -> Option<T>
    where F: FnOnce(&mut Resource) -> T {
        let locked = self.lock();
        match locked {
            Ok(mut data) => match data.get_mut(index) {
                Some(arg) => Some(func(arg)),
                None => None
            },
            Err(_) => None
        }
    }
}

pub struct Resource {
    pub id: Uuid,
    pub data: Array<Complex64, Ix2>
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            id: Uuid::new_v4(),
            data: array![[Complex::from(1.0)], [Complex::from(0.0)]]
        }
    }

    // We expose integer index into our vector of resources to the host
    // so as to not expose raw pointers with unsafe code.
    // To support that usecase, we want two associated functions:
    // - create a new Resource and push it
    // - get a borrowed reference to a Resource given its index
}
