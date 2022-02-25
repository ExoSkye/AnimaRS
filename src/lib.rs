#![no_std]

pub mod common;
pub mod engine;
pub mod backends;
pub mod dummy_backend;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
