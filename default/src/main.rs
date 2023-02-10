use std::default;

// #[derive(Default)]
struct Product {
    name: String,
    price: f64,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            name: "Default name".to_string(),
            price: 999.0,
        }
    }
}

#[derive(Default, Debug)]
enum Fruit {
    #[default]
    Apple,
    Banana,
    Orange,
}

fn main() {
    // base type default value
    let s = String::default();
    println!("String: {}", s);

    let int = i32::default();
    println!("Integer: {}", int);

    let str: &str = Default::default();
    println!("&str: {}", str);

    // struct default value
    let product = Product {
        name: "".to_string(),
        price: 0.0,
    };
    println!("Name: {}, Price ${}", product.name, product.price);

    let product: Product = Default::default();
    println!("Name: {}, Price ${}", product.name, product.price);

    let product = Product {
        name: "Coffee".to_string(),
        ..Default::default() // 注意不能加逗号
    };
    println!("Name: {}, Price ${}", product.name, product.price);

    // enum default
    let fruit = Fruit::default();
    println!("{:?}", fruit);

    let banana = Fruit::Banana;
    println!("{:?}", banana);
}
