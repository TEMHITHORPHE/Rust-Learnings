pub fn is_perfect_square(a: isize, b: isize, c: isize) -> bool {
    (b.pow(2) - 4 * a * c) == 0
}

fn determinant(a: isize, b: isize, c: isize) -> isize {
    b.pow(2) - 4 * a * c
}

pub enum EquationRoots {
    NormalRoot(f64, f64),
    ComplexRoot(f64, f64),
}

pub fn find_roots(a: isize, b: isize, c: isize) -> EquationRoots {
    let determinant = determinant(a, b, c);
    let lhs = (-b / 2 * a) as f64;
    if determinant >= 0 {
        // Normal & Perfect Roots
        let rhs = (determinant as f64).sqrt() / 2.0 * (a as f64);
        let x1 = lhs + rhs;
        let x2 = lhs - rhs;
        return EquationRoots::NormalRoot(x1, x2);
    } else {
        // Complex Roots
        let rhs = (determinant / 2 * a) as f64;
        return EquationRoots::ComplexRoot(lhs, rhs);
    }
}
