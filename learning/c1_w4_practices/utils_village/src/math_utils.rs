

pub fn factorial_of_number(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut counter = n;
    let mut result = n;
    while counter > 1{
        result *= counter;
        counter -= 1;
    }

    result
}


// find inverse-of-y modulo x such that y*y^-1 mod x = 1
// problem: find a and b such that a*x + b*y = 1
// formula: a*A1 + b*A2 = A3 and a*B1 + b*B2 = B3 
// a*R1 + b*R2 = a*A1 - a*Q*B1 + b*A2 - b*Q*B2 = R3
// matrix of elements in modulo space: 
// dividend A3 and elements (A1, A2) 
// divisor B3 and elements (B1, B2)
// residual R3 and elements (R1, R2)
// divide A3 by B3 to get quotient Q and remainder R3
// in the next step, (B1, B2, B3) becomes dividend and (R1, R2, R3) becomes divisor
pub fn great_common_divisor_extended(x: u64, y: u64) -> (u64, Option<u64>) {
    let mut a3 = x;
    let mut b3 = y;

    if a3 < b3 {
        let temp = a3;
        a3 = b3;
        b3 = temp;
    }

    let mut a1 = 1; 
    let mut a2 = 0;
    let mut b1 = 0;
    let mut b2 = 1;

    while b3 != 0 && b3 != 1 {
        let q = a3 / b3;
        let r1 = a1 - q * b1;
        let r2 = a2 - q * b2;
        let r3 = a3 - q * b3;

        a1 = b1; a2 = b2; a3 = b3;
        b1 = r1; b2 = r2; b3 = r3;
    }

    if b3 == 0 
    {
        return (a3, None);
    }
    return (1, Some(b2));

}


pub fn is_prime_number(n: u64) -> bool {

    if n <= 1{
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}