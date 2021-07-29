fn main() {
    let c = circle{r:1.0};
    println!("circle with r 1, area is: {}", c.area());
}


fn calc_area<T: CalcArea>(geometric: T) -> f64{
    geometric.area()
}

pub trait CalcArea{
    fn area(&self) -> f64;
}

struct triangle{
    side_a:f64,
    side_b:f64,
    side_c:f64
}

struct square{
    side:f64
}

struct circle{
    r:f64
}

impl CalcArea for square{
    fn area(&self) -> f64{
        self.side * self.side
    }
}

impl CalcArea for circle{
    fn area(&self) -> f64{
        self.r * self.r * core::f64::consts::PI
    }
}