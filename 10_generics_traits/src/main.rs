//Struct with generic type
struct Point<T,U>{
    x: T,
    y: U,
}

//下面这个语法规则很有意思。
impl<T,U> Point<T,U>{
    fn new(x:T, y: U) -> Point<T,U>{
        Point{
            x,
            y,
        }
    }
}

//可以给具体的类型实现方法。
impl Point<i32, i32>{
    fn printing(&self){
        println!("i32 version. x: {}, y: {}", self.x, self.y);           
    }

    //不能使用 new， 和上面的泛型 new 重名。
    fn new_1(x: i32, y: i32) -> Point<i32, i32>{
        println!("new_1 for i32 version.");
        Point{
            x,
            y,
        }
    }
}

impl Point<f32, f32>{
    fn printing(&self){
        println!("f32 version. x: {:.1}, y: {:.2}", self.x, self.y);           
    }
}

//function with generic type
fn add_point<T, U>(p1 : &Point<T,U>, p2: &Point<T,U>) -> Point<T,U>{
    unimplemented!();
}

fn main() {
    let p1 = Point::new(1.0 as f32,2.0 as f32);
    p1.printing();

    let origin = Point::new_1(0, 0);
    origin.printing();

    //static dispatch
    add_point(&origin, &origin);
    add_point(&p1, &p1);
}
