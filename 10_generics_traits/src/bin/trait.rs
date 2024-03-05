use std::fmt::Debug;

trait Drawable{
    fn draw(&self);
}
trait Shape : Drawable{
    fn area(&self) -> f32;
    fn describe(&self) -> String;

    fn print_area(&self){
        println!("{}, area is {}.",self.describe(),self.area());
    }
}

//结构体是可以嵌套，来表达复杂的概念关系。
#[derive(Debug)]
struct NameColor{
    name : String, 
    color: String,
}

impl NameColor{
    fn to_string(&self) -> String{
        format!("{} {}",self.color, self.name)
    }
}

#[derive(Debug)]
struct Circle{
    common: NameColor,
    radius: f32,
}

#[derive(Debug)]
struct  Rectangle{
    common: NameColor,
    width: f32,
    height: f32,
}

impl Shape for Circle{
    fn area(&self) -> f32{
        std::f32::consts::PI * self.radius * self.radius
    }
    fn describe(&self) -> String{
        self.common.to_string()
    }
}

//必须在单独的 implementation 中实现 super trait 的方法。
impl Drawable for Circle{
    fn draw(&self){
        println!("Drawing a circle.");
    }
}

impl Shape for Rectangle{
    fn area(&self) -> f32{
        self.width * self.height
    }

    fn describe(&self) -> String{
        self.common.to_string()
    }
}

impl Drawable for Rectangle{
    fn draw(&self){
        println!("Drawing a rectangle.");
    }
}

/*
这个函数接受一个 &dyn Shape 类型的参数，这意味着它可以接受任何实现了 Shape trait 的对象的引用。
这种方式的优点是灵活性高，可以处理多种类型的对象。缺点是运行时性能稍差，因为需要在运行时查找正确的方法。
 */
fn dynamic_print_area1(shape: &dyn Shape){
    println!("dynamic_print_area1: area is {:.2}.",shape.area());
}

// fn dynamic_print_area3(shape: dyn Shape){
//     println!("dynamic_print_area2: area is {:.2}.",shape.area());
// }
//如果直接使用 dyn Shape ，上面的代码会报错，因为 dyn Shape 是一个 trait 对象，它是一个动态大小的类型，不能直接使用。
fn dynamic_print_area2(shape: Box<dyn Shape>){
    println!("dynamic_print_area2: area is {:.2}.",shape.area());
}

//只能返回一种类型，在编译期间就确定了具体的类型
// fn get_shape() -> impl Shape{
//     if true {
//         Circle{
//             common: NameColor{
//                 name: "Circle".to_string(),
//                 color: "Red".to_string(),
//             },
//             radius: 1.0,
//         }
//     }else{
//         Rectangle{
//             common: NameColor{
//                 name: "Rectangle".to_string(),
//                 color: "Blue".to_string(),
//             },
//             width: 2.0,
//             height: 3.0,
//         }
    
//     }
// }

//通过 Box ， 可以返回不同的类型。
fn get_shape() -> Box<dyn Shape>{
    if true {
        Box::new(Circle{
            common: NameColor{
                name: "Circle".to_string(),
                color: "Red".to_string(),
            },
            radius: 1.0,
        })
    }else{
        Box::new(Rectangle{
            common: NameColor{
                name: "Rectangle".to_string(),
                color: "Blue".to_string(),
            },
            width: 2.0,
            height: 3.0,
        })
    }
}


/*
对比几种不同的 Trait Bound 方式。这些都是 static dispatch 的方式，即在编译时就确定了调用的方法， 会为每种类型生成一个新的函数。
*/
fn static_print_area2<T : Shape + Debug>(shape: &T){
    println!("print_area2: area is {}.",shape.area());
}

//下面是上面在编译期间展开的代码
// fn static_print_area2_circle(shape: &Circle){
//     println!("print_area2: area is {}.",shape.area());
// }

// fn static_print_area2_rec(shape: &Rectangle){
//     println!("print_area2: area is {}.",shape.area());
// }

fn static_print_area3(obj: &impl Shape){
    println!("print_area3: area is {}.",obj.area());
}

fn static_print_area4<T>(shape: &T) where T: Shape + Debug{
    println!("print_area4: area is {}.",shape.area());
}

fn main(){
    let circle = Circle{
        common: NameColor{
            name: "Circle".to_string(),
            color: "Red".to_string(),
        },
        radius: 1.0,
    };
    circle.draw();
    circle.print_area();
    static_print_area2(&circle);
    dynamic_print_area1(&circle);

    println!("\n===================\n");

    let rectangle = Rectangle{
        common: NameColor{
            name: "Rectangle".to_string(),
            color: "Blue".to_string(),
        },
        width: 2.0,
        height: 3.0,
    };
    rectangle.draw();
    rectangle.print_area();
    dynamic_print_area1(&rectangle);
    static_print_area2(&rectangle);
    static_print_area3(&rectangle);
    static_print_area4(&rectangle);

    dynamic_print_area2(Box::new(rectangle));

}