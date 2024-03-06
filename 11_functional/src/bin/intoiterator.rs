// Problem 3: Complete the into_iter function definition. 
// Also add the type for the associated type item

struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8; // this needs to be fixed 
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        /* The function needs to be completed */ 
        vec![self.r, self.g, self.b].into_iter()
    }
}

fn main() {
    let p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    let mut p = p.into_iter();
    println!("{:?}", p);
    p.next(); //vector iterator is mutable, next 会 remove 元素。
    println!("{:?}", p);
}