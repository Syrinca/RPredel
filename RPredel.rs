fn main() {
    let a = 2.0; // Значение, к которому стремится x
    let limit = compute_limit(a);
    println!("The limit of the function as x approaches {} is: {}", a, limit);
}

fn compute_limit(a: f64) -> f64 {
    let h = 0.0001; // Очень маленькое число для приближения производной

    // Функция f(x), для которой нужно вычислить предел
    let f = |x: f64| (x * x - 1.0) / (x - 1.0);

    // Производная функции f(x)
    let df = |x: f64| (f(x + h) - f(x)) / h;

    // Вычисление предела с использованием метода Лопиталя
    if df(a).abs() < std::f64::EPSILON {
        return f(a);
    } else {
        return df(a);
    }
}