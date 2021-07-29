fn main() {
    let light = Color::GREEN;
    println!("GREEN time {}", light.on_time());
}

pub enum Color{
    RED,
    YELLOW,
    GREEN
}

pub trait Traffic_Light{
    fn on_time(&self) -> u32;
}

impl Traffic_Light for Color{
    fn on_time(&self) -> u32{
        match self{
            Color::RED => 10,
            Color::GREEN => 20,
            Color::YELLOW => 3
        }
    }
}
