#[derive(Debug)]
struct Employee {
    name: String,
    age: u32,
    salary: f32,
}

#[derive(Debug)]
struct EmployeeDB {
    employees: Vec<Employee>,
}

impl Iterator for EmployeeDB {
    type Item = Employee;
    //会调用 remove 方法，所以self 必须是可变的。如何不可变也可以迭代？
    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > 0 {
            Some(self.employees.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    let mut employees = EmployeeDB {
        employees: vec![
            Employee {
                name: "John".to_string(),
                age: 25,
                salary: 1000.0,
            },
            Employee {
                name: "Jane".to_string(),
                age: 22,
                salary: 1200.0,
            },
            Employee {
                name: "Tom".to_string(),
                age: 30,
                salary: 1300.0,
            },
        ],
    };
    println!("Before: Employees {:#?}", employees);
    let employee = employees.next();
    println!("{:#?}", employee);
    println!("After: Employees {:#?}", employees);

    //for loop 会自动调用next方法，而且会处理Option的情况。
    for employee in employees {
        println!("{:#?}", employee);
    }
}
