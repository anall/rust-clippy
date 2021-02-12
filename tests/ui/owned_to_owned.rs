#![warn(clippy::owned_to_owned)]
#![allow(clippy::redundant_clone)]
use std::borrow::Borrow;
use std::path::PathBuf;

fn return_owned_from_ref(result : &[u32]) -> Vec<u32> {
    result.to_owned()
}

#[derive(Copy,Clone)]
struct Kitten {}
impl Kitten {}
impl Borrow<BorrowedKitten> for Kitten {
    fn borrow(&self) -> &BorrowedKitten {
        static VALUE: BorrowedKitten = BorrowedKitten {};
        &VALUE
    }
}

struct BorrowedKitten {}
impl ToOwned for BorrowedKitten {
    type Owned = Kitten;
    fn to_owned(&self) -> Kitten {
        Kitten {}
    }
}

fn main() {
    let result = vec![5];
    let _ = return_owned_from_ref(&result);
    let _ = result.to_owned();
    let _ = result.to_vec();

    let str = "hello world".to_string();
    let _ = str.to_owned();

    let kitten = Kitten{};
    let _ = kitten.to_owned();

    let borrowed = BorrowedKitten{};
    let _ = borrowed.to_owned();

    let pathbuf = PathBuf::new();
    let _ = pathbuf.to_owned();
    let _ = pathbuf.to_path_buf();
}
