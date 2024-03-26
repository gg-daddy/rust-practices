use negative_impl::negative_impl;

struct ABC {}

#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}

// Sized 并不是 auto trait，所以不能使用 negative_impl.
//Sized 特性在 Rust 中有特殊的地位。它并不是一个 auto trait，但是它会被编译器自动应用到所有在编译时大小已知的类型上。
//当我们说一个类型 "在编译时大小已知"，我们是指编译器可以在编译时确定该类型的实例需要多少内存。
//例如，基本类型（如 i32，bool 等）和结构体（如果它们的所有字段的类型都是 Sized）都是 Sized。
//然而，有些类型的大小在编译时是未知的。例如，切片类型（如 &[T]）和 trait 对象（如 Box<dyn Trait>）就不是 Sized。
//这是因为这些类型的实例可以有不同的大小，这取决于运行时的情况。
//虽然 Sized 不是 auto trait，但编译器会自动为所有在编译时大小已知的类型实现 Sized 特性。

//non auto traits are not supported
// #[negative_impl]
// impl !Sized for ABC {}

// ?Sized 表示类型可能是 Sized 也可能是 !Sized 。
// 这是一个特殊的 trait bound，它可以用于泛型参数。
// 如果使用了 ?Sized， 它们必须通过某种指针（如引用、Box、Rc 等）来使用。
//所以在这个函数定义中，参数类型是 &T，而不是 T。但这并不意味着所有使用 ?Sized 的 T 都必须是引用类型，它们也可以是其他指针类型。
fn some_func<T: ?Sized>(_: &T) {}

// 下面 T 是假设是 Sized 的，这是一种语法糖，脱糖之后就是 <T: Sized> 。
fn some_func2<T>(_: T) {}

fn main() {}
