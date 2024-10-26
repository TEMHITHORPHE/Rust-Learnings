mod math_ops;
use math_ops::quadratic::{
    find_roots, is_perfect_square, EquationRoots::ComplexRoot, EquationRoots::NormalRoot,
};

fn main() {
    println!("Hello World");
    let (a, b, c) = (2, 9, 4);
    let is_perfect_square = is_perfect_square(a, b, c);
    println!("[is perfect square]: {}", is_perfect_square);

    let roots = find_roots(a, b, c);
    match roots {
        NormalRoot(x1, x2) => {
            println!(
                "[The equation {}x + {}x^2 + {} has normal roots]: x1:{}, x2:{}",
                a, b, c, x1, x2
            );
        }
        ComplexRoot(i, j) => {
            println!(
                "[The equation {}x + {}x^2 + {} has complex roots]: {}i +/- {}j",
                a, b, c, i, j
            );
        }
    }
}
