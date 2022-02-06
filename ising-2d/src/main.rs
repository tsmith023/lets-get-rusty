use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ops::Index;

#[derive(Debug)]
struct UnitCell<const N: usize> {
    field: i32,
    hopping: i32,
    spins: [i32; N],
}

impl <const N: usize>  Default for UnitCell<{ N }> {
    fn default() -> Self {
        UnitCell {
            field: 0,
            hopping: 1,
            spins: [1; N],
        }
    }
}

impl <const N: usize> Index<[usize; 2]> for UnitCell<{ N }> {
    type Output = i32;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let i = index[0];
        let j = index[1];
        &self.spins[(i * N) + j]
    }
}

impl <const N: usize> UnitCell<{ N }> {
    fn new(field: i32, hopping: i32, ratio: i32) -> Self {
        let mut spins = [-1; N];
        let ups_to_downs = (N as i32) / (1 + ratio);
        for n in 0..N {
            if n < ups_to_downs as usize {
                spins[n] = 1;
            }
            else {
                spins[n] = -1;
            }
        }
        let mut rng = thread_rng();
        spins.shuffle(&mut rng);
        UnitCell {
            field: 0,
            hopping: 1,
            spins: spins,
        }
    }

    fn lattice_site_contribution(&self, i: usize, j: usize) -> i32 {
        - self.hopping * (
            self[[i,j]] * self[[i,j+1]] + self[[i,j]] * self[[i,j-1]] + self[[i,j]] * self[[i+1,j]] + self[[i,j]] * self[[i-1,j]]
        ) - self.field * self[[i,j]]
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_unit_cell_new() {
    let unit_cell = UnitCell::<256>::new(0, 1, 1);
    assert_eq!(unit_cell.spins.len(), 256);
    let mut ups: Vec<i32> = Vec::new();
    let mut downs: Vec<i32> = Vec::new();
    for spin in unit_cell.spins{
        if spin == 1 {
            ups.push(spin)
        }
        else if spin == -1 {
            downs.push(spin)
        }
    }
    assert_eq!(ups.len(), 128);
    assert_eq!(downs.len(), 128);
}

#[test]
fn test_unit_cell_index() {
    let unit_cell = UnitCell::<256>::default();
    assert_eq!(unit_cell[[0,0]].abs(), 1); 
}

// #[test]
// fn test_unit_cell_lattice_site_contribution() {

// }