/*
Option 的作用， 利用 None 表示还未定义。
1. 如何初始化？
2. 如何获取值？
*/
struct Student {
    name: String,
    grade: Option<f32>,
}

fn get_grade(name: &String, students: &Vec<Student>) -> Option<f32> {
    for student in students {
        if student.name == *name {
            return student.grade;
        }
    }
    None // 这个 None 定义不是很清楚，是没找到学生，还是没有分数。 表述上比较模糊，下面用 Result 进行了区分，更加的明晰。
}

fn get_student_grade(name: &String, students: &Vec<Student>) -> Result<Option<f32>, String> {
    for student in students {
        if student.name == *name {
            return Ok(student.grade);
        }
    }
    Err("Student not found".to_string())
}

fn main() {
    let students = vec![
        Student {
            name: "John".to_string(),
            grade: Some(90.0),
        },
        Student {
            name: "Jane".to_string(),
            grade: None,
        },
        Student {
            name: "Doe".to_string(),
            grade: Some(80.0),
        },
    ];

    let grade = get_grade(&"Jane".to_string(), &students);
    match grade {
        Some(grade) => println!("John's grade is {}", grade),
        None => println!("John's grade is not available"),
    }

    let name = "Doe".to_string();
    if let Some(grade) = get_grade(&name, &students) {
        println!("{}'s grade is {}", name, grade);
    } else {
        println!("{}'s grade is not available", name);
    }

    let grade_result = get_student_grade(&"Jane".to_string(), &students);
    match grade_result {
        Ok(grade) => {
            if let Some(value) = grade {
                println!("Grade is :{}", value);
            } else {
                println!("Grade is not avaible!");
            }
        }
        Err(error_msg) => println!("{}", error_msg),
    }
}
