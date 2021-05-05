use ethnum::U256;

pub fn addmod(x: &U256, y: &U256, m: &U256) -> U256 {
    let (out, carry) = x.overflowing_add(*y);
    // In textbook 14.27, says addmod is add and an extra step: subtract m iff x+y>=m
    if carry || m <= &out {
        out % m
    } else {
        out
    }
}

pub fn normalize(a: &U256, p: &U256) -> U256 {
    if a < &U256::new(0) {
        p - ((a.overflowing_neg().0) % p)
    } else {
        a % p
    }
}

pub fn mod_pow(base: &U256, exp: &U256, modulus: &U256) -> U256 {
    if modulus == &1 {
        return U256::new(0);
    }
    let mut result = U256::new(1);
    let mut base = *base;
    let mut exp = *exp;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mulmod(&result, &base, &modulus);
        }
        exp /= 2;
        base = mulmod(&base, &base, &modulus)
    }
    result
}

pub fn mulmod(a: &U256, b: &U256, m: &U256) -> U256 {
    let mut a = *a;
    let mut b = *b;
    let m = *m;

    let mut res = U256::ZERO;
    let mut temp_b;

    /* Only needed if b may be >= m */
    if b >= m {
        if m > U256::MAX / 2 {
            b -= m;
        } else {
            b %= m;
        }
    }

    while a != U256::new(0) {
        if a & U256::new(1) != U256::ZERO {
            /* Add b to res, modulo m, without overflow */
            if b >= m.wrapping_sub(res)
            /* Equiv to if (res + b >= m), without overflow */
            {
                res = res.wrapping_sub(m);
            }
            res = res.wrapping_add(b);
        }
        a = a / 2;

        /* Double b, modulo m */
        temp_b = b;
        if b >= m.wrapping_sub(b)
        /* Equiv to if (2 * b >= m), without overflow */
        {
            temp_b = temp_b.wrapping_sub(m);
        }
        b = b.wrapping_add(temp_b);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addmod_under_10() {
        let a = U256::new(1);
        let b = U256::new(2);
        let m = U256::new(10);
        assert_eq!(U256::new(3), addmod(&a, &b, &m));
    }

    #[test]
    fn test_addmod_over_10() {
        let a = U256::new(9);
        let b = U256::new(9);
        let m = U256::new(10);
        assert_eq!(U256::new(8), addmod(&a, &b, &m));
    }

    #[test]
    fn test_addmod_overflow() {
        let a = U256::from_str_radix(
            "21663839004416932945382355908790599225266501822907911457504978515578255421292",
            10,
        )
        .unwrap();
        let m = U256::from_str_radix(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617",
            10,
        )
        .unwrap();
        let b = m - a + 1;
        assert_eq!(U256::new(1), addmod(&a, &b, &m));
    }

    #[test]
    fn test_pow_mod() {
        let b = U256::from_str_radix(
            "7120861356467848435263064379192047478074060781135320967663101236819528304087",
            10,
        )
        .unwrap();
        let e = U256::from_str_radix("5", 10).unwrap();
        let m = U256::from_str_radix(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617",
            10,
        )
        .unwrap();
        let expected = U256::from_str_radix(
            "10745147226606258107230518846003648962088186972186954460246329063381405757050",
            10,
        )
        .unwrap();

        assert_eq!(mod_pow(&b, &e, &m), expected);
    }
}
