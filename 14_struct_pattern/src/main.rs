use struct_pattern::Student;
/*
1. 介绍了如何使用 Default Trait 来为结构体提供默认值。
2. 常见的 new 方法，如果出错则返回 Err，这时可以使用 unwrap_or_default 方法来获取默认值。

*/
fn main() {
    let s1 = Student::new("Eason".to_string(), 18);
    println!("Student: {:?}", s1.unwrap());

    let s2 = Student::new("Invalid123".to_string(), 12);
    //如果 Student::new 返回 Err，则使用默认值。
    println!("Student: {:?}", s2.unwrap_or_default());

    //下面列举了两个使用 Default Trait 的例子。
    let d = Student::default();
    println!("Default Student: {:?}", d);

    let d2: Student = Default::default(); // 因为前面指明了类型，所以这里可以不写。
    println!("Another way to get default Student: {:?}", d2);

    //可以用 default 方法来创建一个默认的实例，然后修改其中的某些字段。
    //这是 Default::default() 方法的一个常见用法。但是需要成员变量是 public 的。
    let s3 = Student {
        age: 20,
        ..Default::default()
    };
    println!("Student with age 20: {:?}", s3);
}
