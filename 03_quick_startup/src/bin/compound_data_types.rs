fn main() {
    //固定长度的数组
    let fixed_str = "Fixed String value";

    //如果要修改，必须添加 mut。
    let mut flexible_str = String::from("Flexible String value");
    flexible_str.push_str("pushed string.");

    //数组是固定长度的
    let mut array_1 = [1, 2, 3, 4, 5, 6];
    //添加了 mut ， 可以对元素进行修改
    array_1[0] = 10;

    //一种初始化方式
    let mut array_2 = [1; 5];
    array_2[0] = 2;
    println!("Array 2: {:?}", array_2);

    //vectors， 和 array 一样，元素类型必须一致。 但是 vectors 长度可变。
    let mut vector_1 = vec![1, 2, 3, 4, 5];
    vector_1.push(6);
    println!("Vector 1: {:?}", vector_1);

    //元组
    let mut my_info = ("salary", 4000, "age", 30);
    println!("My info: {:?}", my_info);
    my_info.1 = 5000;
    println!("my_info.1: {}", my_info.1);

    //可以利用 pattern matching 解构元组
    let (_, salary_value, _, age_value) = my_info;
    println!("Salary: {}, Age: {}", salary_value, age_value);

    //unit type
    let unit = ();
    println!("Unit: {:?}", unit);
}
