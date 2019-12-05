fn main() {
    // It is a six-digit number.
    // The value is within the range given in your puzzle input.
    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the
    // same (like 111123 or 135679).  Other than the range rule, the following are true:

    // 111111 meets these criteria (double 11, never decreases).
    // 223450 does not meet these criteria (decreasing pair of digits 50).
    // 123789 does not meet these criteria (no double).
    let minn = 147981;
    let maxx = 692423;

    let mut matches = 0;
    for num in minn..=maxx {
        if possible(num) {
            matches += 1;
        }
    }

    println!("{}", matches);
}

fn possible(nn: usize) -> bool {
    let d1 = nn % 10;
    let d2 = nn % 100 / 10;
    let d3 = nn % 1_000 / 100;
    let d4 = nn % 10_000 / 1_000;
    let d5 = nn % 100_000 / 10_000;
    let d6 = nn % 1_000_000 / 100_000;

    let mut exactly_two_matching = false;
    if d1 == d2 && d2 != d3 ||
        d1 != d2 && d2 == d3 && d3 != d4 ||
        d2 != d3 && d3 == d4 && d4 != d5 ||
        d3 != d4 && d4 == d5 && d5 != d6 ||
        d4 != d5 && d5 == d6
    {
        exactly_two_matching = true;
    }


    let mut always_increasing = false;
    if d6 <= d5 && d5 <= d4 && d4 <= d3 && d3 <= d2 && d2 <= d1 {
        always_increasing = true;
    }

    return exactly_two_matching && always_increasing;
}
