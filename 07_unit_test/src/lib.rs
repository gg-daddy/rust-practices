mod shape{
    pub struct Circle{
        radius: f64
    }

    impl Circle{
        pub fn new(radius : f64) -> Circle{
            //默认情况下 cargo test 不会打印 output， 可以通过 cargo test -- --nocapture 来打印输出， 
            //或者通过 --show-output,Show captured stdout of successful tests 。
            println!("Successfully created a new circle!");
            Circle{
                radius
            }
        }

        pub  fn new_with_check(radius : f64) -> Result<Circle, String>{
           if radius > 0.0 {
            Ok(Circle{
                radius
            })
           }else{
                Err("Radius cannot be negative".to_string())
           }
        }

        pub fn new_with_panic(radius : f64) -> Circle{
            match radius {
                ..=0.0 => panic!("Radius cannot be negative"), //正常情况下不应该使用 panic!，此处只是学习测试。
                _ => Circle{
                    radius
                }
            }
        }

        pub fn contains(&self, other: &Circle) -> bool{
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod test{

    use super::shape::*;

    #[test]
    fn large_should_contain_small(){
        let large = Circle::new(10.0);
        let small = Circle::new(5.0);
        assert!(large.contains(&small));
        assert_eq!(large.contains(&small),true);
        assert_ne!(large.contains(&small),false, "here can define custom error messages.");
    }

    #[test]
    fn negative_radius() -> Result<(), String>{
        Circle::new_with_check(-1.0)?;
        Ok(())
    }

    #[test]
    #[should_panic]
    #[ignore]
    /*
    cargo test -- --ignored 是一个在 Rust 中运行测试的命令，它包含两部分：cargo test 和 -- --ignored。
    cargo test：这是 Rust 的包管理器 Cargo 提供的一个命令，用于运行项目中的所有测试。
    默认情况下，这个命令会运行所有没有被 #[ignore] 属性标记的测试。
    -- --ignored：这是传递给 cargo test 的一个参数。
    -- 符号表示后面的参数应该直接传递给测试框架，而不是被 Cargo 自身处理。
    --ignored 参数告诉测试框架，除了运行所有没有被忽略的测试，还要运行那些被 #[ignore] 属性标记的测试。
    所以，cargo test -- --ignored 命令的作用是运行项目中的所有测试，包括那些被 #[ignore] 属性标记的测试。
     */
    fn negative_raius_panic(){
        Circle::new_with_panic(-1.0);
    }
}