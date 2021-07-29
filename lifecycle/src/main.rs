fn main() {
    let a:&str = "abc";
    let b:&str = "12";
    let c = longest(a, b);
    println!("{}", c);
}

///让编译器知道传入参数与返回值之间生命周期的大小，例如：
/// 1. 若返回值引用的某个参数，那此参数一定不能活得比返回值短
/// 注意：若返回值不允许引用函数内的变量
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}


// fn shortest<'a>(x: &'a str, y: &'a str) -> str{
//     if x.len() < y.len(){
//         *x
//     }else{
//         *y
//     }
// }
