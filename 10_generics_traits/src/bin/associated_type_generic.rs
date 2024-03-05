/*
对比学习， Trait 中什么时候用 associated type， 什么时候用泛型。

*/

#[derive(Debug)]
struct Point{
    x: i32, 
    y: i32,
}

trait Addition<Rhs, Output>{
    //注意 Self::Rhs 是大写的 S 开头。
    fn add(self , other : Rhs) -> Output;
}

impl Addition<Point,Point> for Point{
    fn add(self , other : Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Addition<i32, Point> for Point{
    fn add(self , other : i32) -> Point {
        Point{
            x: self.x + other,
            y: self.y + other,
        }
    }
}

struct  Line {
    start: Point,
    end: Point,
}

impl Addition<Point, Line> for Point{
    fn add(self , other : Point) -> Line {
        Line{
            start: self,
            end: other,
        }
    }
}

fn main(){
    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 3, y: 4};

    let p3: Point = p1.add(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 3, y: 4};

    let p3: Line = p1.add(p2);
    println!("p3.x = {:?}, p3.y = {:?}", p3.start, p3.end);

    let p1 = Point{x: 1, y: 2};
    let p3 = p1.add(12);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}