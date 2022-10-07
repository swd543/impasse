extern crate core;

use std::fmt::{Debug, Display, Error, Formatter, Write};
use std::iter::Product;
use std::ops;
use std::ops::{Add, Index, Mul, Neg, Sub};

#[derive(Debug, Clone)]
enum State {
    WC = 2,
    WW = 1,
    BB = -1,
    BC = -2,
    __ = 0
}

struct Board {
    state: Vec<Vec<State>>
}

impl Board{
    fn new()->Board{
        Board{
            state: vec![vec![State::__, State::WC, State::__, State::BB, State::__, State::WC, State::__, State::BB],
                        vec![State::BB, State::__, State::WC, State::__, State::BB, State::__, State::WC, State::__],
                        vec![State::__; 8],
                        vec![State::__; 8],
                        vec![State::__; 8],
                        vec![State::__; 8],
                        vec![State::__, State::BC, State::__, State::WW, State::__, State::BB, State::__, State::WW],
                        vec![State::WW, State::__, State::BB, State::__, State::WW, State::__, State::BB, State::__]]
        }
    }
    fn valid_moves(&self, position: Coord){

    }
}

impl Display for Board{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut r = Ok(());
        for x in self.state.iter() {
            match r {
                Ok(())=>{
                    r = write!(f, "\n{:?}", x)
                }
                _ => {
                    return r
                }
            }
        }
        r
    }
}


// impl Index<Coord> for Board{
//     type Output = State;
//
//     fn index(&self, index: Coord) -> &State {
//         // if index.x < 0 || index.x >= 8{ //TODO:
//         //     return Err(("Invalid index"));
//         // }
//         // if index.y <0 || index.y>=8{
//         //
//         // } { }
//         &self.state[index.x as usize][index.y as usize]
//     }
// }

impl Index<Coord> for Board{
    type Output = Result<State, String>;

    fn index(&self, index: Coord) -> &Self::Output {
        &Ok(&self.state[0][0])
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord{
    x: isize,
    y: isize
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, o: Coord) -> Self::Output {
        Coord{
            x: self.x+o.x,
            y: self.y+o.y
        }
    }
}

impl Neg for Coord{
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Coord{
            x: -self.x,
            y: -self.y
        }
    }
}

impl Mul<isize> for Coord {
    type Output = Coord;

    fn mul(self, o: isize) -> Self::Output {
        Coord{
            x: self.x*o,
            y: self.y*o
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

const N: Coord = Coord{ x: 0, y: -1 };
const S: Coord = Coord{ x: 0, y: 1 };
const E: Coord = Coord{ x: -1, y: 0 };
const W: Coord = Coord{ x: 1, y: 0 };

fn main() {
    let mut board = Board::new();
    println!("Hello, {:}", board);
    board.valid_moves(Coord{ x: 0, y: 0 });
}
