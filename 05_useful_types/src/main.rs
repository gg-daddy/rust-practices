struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

fn main() {
    /*
       Structs
    */
    let mut my_car = Car {
        owner: "John".to_string(),
        year: 2010,
        fuel_level: 0.5,
        price: 10000,
    };

    let year = my_car.year;
    println!("year: {}", my_car.year);

    {
        let owner = &mut my_car.owner;
    }

    my_car.owner = "Steven".to_string();
    println!("Owner: {}", my_car.owner);

    //使用 .. 语法来复制结构体的其他字段
    let car2 = Car {
        owner: "Bob".to_string(),
        ..my_car
    };
    println!("owner : {}", my_car.owner);

    /*
       Tuple Structs
    */
    let point_2D = (1, 2);
    let point_3D = (1, 2, 3);
    // 用元组结构体来表示， 相比直接使用 Tuple， Tuple Structs 的优势是可以给元素命名。
    struct Point2D(i32, i32);
    struct Point3D(i32, i32, i32);
    let mut point_2D = Point2D(1, 2);
    let point_3D = Point3D(1, 2, 3);

    println!("point_2D: {}", point_2D.0);
    point_2D.0 = 3;
    println!("point_2D: {}", point_2D.0);

    //Unit Struct
    struct ABC;

    //type alias
    type Test = Car;
    let test1 = Test {
        owner: "Test".to_string(),
        year: 2022,
        fuel_level: 0.75,
        price: 20000,
    };
    println!("test1: {}", test1.owner);
}
