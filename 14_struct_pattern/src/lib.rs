#[derive(Debug)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u16,
}

impl Student {
    pub fn new(name: String, age: u16) -> Result<Student, String> {
        if name.chars().all(|x| matches!(x, 'a'..='z' | 'A'..='Z')) {
            Ok(Student { id: 0, name, age })
        } else {
            Err("name must be alphabetic".to_string())
        }
    }
}

/*
可以选择自己实现 Default Trait， 或者使用 derive(Default) 来自动生成默认实现。
derive(Default) 会自动生成一个默认实现，所有字段都会使用默认值。
字段的默认值是指，如果字段是数字类型，则为 0，如果字段是字符串类型，则为 ""，如果字段是 bool 类型，则为 false。
*/
impl Default for Student {
    fn default() -> Self {
        // 这里可以不写 Self，直接写 Student 也可以。
        Self {
            id: 0,
            name: "Default".to_string(),
            age: 0,
        }
    }
}
