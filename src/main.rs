use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use std::str::FromStr;

const BITS: u32 = 256;
const TWO_ADICITY: u32 = 32;

const INPUT_MODULUS: &str =
    "52435875175126190479447740508185965837690552500527637822603658699938581184513";
const INPUT_GENERATOR: &str = "7";

fn main() {
    let modulus = BigUint::from_str(INPUT_MODULUS).unwrap();
    let generator = BigUint::from_str(INPUT_GENERATOR).unwrap();

    let modulus_bits = modulus.bits();

    let modulus_minus_one_div_two = (&modulus - BigUint::one()) >> 1u32;

    let r = BigUint::from(2u32).modpow(&BigUint::from(BITS), &modulus);
    let r2 = (&r * &r) % &modulus;

    let t = (&modulus - BigUint::one()) >> TWO_ADICITY;
    let t_minus_one_div_two = (&t - BigUint::one()) >> 1u32;

    let two_adic_root_of_unity = generator.modpow(&t, &modulus);

    let inv = BigInt::from(2u128.pow(64))
        - mod_inverse(&modulus.to_bigint().unwrap(), &BigInt::from(2u128.pow(64)));

    println!("impl FftParameters for XXXXXXXXXX {{");
    println!("    type BigInt = BigInteger256;");
    println!();
    println!("    const TWO_ADICITY: u32 = {};", TWO_ADICITY);
    println!();
    println!("    const TWO_ADIC_ROOT_OF_UNITY: BigInteger256 = BigInteger256::new([");
    print_biguint(&to_montgomery(&two_adic_root_of_unity, &r, &modulus));
    println!("    ]);");
    println!("}}");
    println!();
    println!("impl FpParameters for XXXXXXXXXX {{");
    println!("    const MODULUS: BigInteger256 = BigInteger256::new([");
    print_biguint(&modulus);
    println!("    ]);");
    println!();
    println!("    const MODULUS_BITS: u32 = {};", modulus_bits);
    println!();
    println!("    const CAPACITY: u32 = Self::MODULUS_BITS - 1;");
    println!();
    println!("    const REPR_SHAVE_BITS: u32 = {};", 256 - modulus_bits);
    println!();
    println!("    const R: BigInteger256 = BigInteger256::new([");
    print_biguint(&r);
    println!("    ]);");
    println!();
    println!("    const R2: BigInteger256 = BigInteger256::new([");
    print_biguint(&r2);
    println!("    ]);");
    println!();
    println!("    const INV: u64 = {:#x};", &inv);
    println!();
    println!("    const GENERATOR: BigInteger256 = BigInteger256::new([");
    print_biguint(&to_montgomery(&generator, &r, &modulus));
    println!("    ]);");
    println!();
    println!("    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger256 = BigInteger256::new([");
    print_biguint(&modulus_minus_one_div_two);
    println!("    ]);");
    println!();
    println!("    const T: BigInteger256 = BigInteger256::new([");
    print_biguint(&t);
    println!("    ]);");
    println!();
    println!("    const T_MINUS_ONE_DIV_TWO: BigInteger256 = BigInteger256::new([");
    print_biguint(&t_minus_one_div_two);
    println!("    ]);");
    println!("}}");
}

fn mod_inverse(operand: &BigInt, modulus: &BigInt) -> BigInt {
    // Ported from:
    //   https://github.com/dignifiedquire/num-bigint/blob/56576b592fea6341b7e1711a1629e4cc1bfc419c/src/algorithms/mod_inverse.rs#L11
    let extended_gcd = operand.extended_gcd(modulus);
    if extended_gcd.gcd != BigInt::one() {
        panic!("GCD must be one");
    }

    if extended_gcd.x < BigInt::zero() {
        extended_gcd.x + modulus
    } else {
        extended_gcd.x
    }
}

fn to_montgomery(num: &BigUint, r: &BigUint, modulus: &BigUint) -> BigUint {
    (num * r) % modulus
}

fn print_biguint(num: &BigUint) {
    let elements = [
        num % 2u128.pow(64),
        (num >> 64u32) % 2u128.pow(64),
        (num >> (64u32 * 2)) % 2u128.pow(64),
        (num >> (64u32 * 3)) % 2u128.pow(64),
    ];

    println!(
        "        {:#x},\n        {:#x},\n        {:#x},\n        {:#x}",
        elements[0], elements[1], elements[2], elements[3],
    );
}
