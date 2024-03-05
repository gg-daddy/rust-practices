// Problem 1: Add the correct types for the associated type 'item' in the implemenation blocks
trait Container {
    type Item;

    fn add_item(&mut self, item: Self::Item);
    fn get_item(&self) -> Option<&Self::Item>;
}

struct VecContainerI32 {
    items: Vec<i32>,
}

//下面的关联类型绑定的是确定的类型。
impl Container for VecContainerI32 {
    type Item = i32; // This line needs a fix

    fn add_item(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn get_item(&self) -> Option<&Self::Item> {
        self.items.last()
    }
}

struct OptionContainer<T> {
    item: Option<T>,
}

//下面的关联类型绑定的是泛型类型。具体的类型在实例化的时候确定。
impl<T> Container for OptionContainer<T> {
    type Item = T; // This line needs a fix

    fn add_item(&mut self, item: T) {
        self.item = Some(item);
    }

    fn get_item(&self) -> Option<&T> {
        self.item.as_ref()
    }
}

fn main() {
    let mut vec_container = VecContainerI32 { items: Vec::new() };
    vec_container.add_item(42);
    vec_container.add_item(123);

    if let Some(last_item) = vec_container.get_item() {
        println!("Last item in VecContainer: {}", last_item);
    } else {
        println!("VecContainer is empty");
    }

    let mut option_container = OptionContainer { item: None };
    option_container.add_item("Hello, Rust!");

    if let Some(only_item) = option_container.get_item() {
        println!("Only item in OptionContainer: {}", only_item);
    } else {
        println!("OptionContainer is empty");
    }
}
