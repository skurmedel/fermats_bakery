use rug::{Assign, Integer};

pub mod primality;

pub type BigInt = Integer;


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rug::{Integer, integer::Order};

    #[test]
    fn it_works() {
        let mut int = Integer::from_str("10213").expect("A value");
        assert_eq!(int, 10213);

        let d = Integer::from_digits(&[111u8], Order::Lsf);
        
        int.modulo_mut(&d);

        assert_eq!(int, 1);
    }
}
