use num_format::ToFormattedString;

fn main() {
    println!("{}", format!("{}", 100));
    println!("{}", format!("{:010}", 5)); // 填充 0
    println!("{}", format!("{:#b}", 50)); // 二进制
    println!("{}", format!("{:#o}", 50)); // 八进制
    println!("{}", format!("{:#x}", 50)); // 十六进制

    println!("============");
    println!(
        "{}",
        1_000_000_000.to_formatted_string(&num_format::Locale::en)
    );
    println!(
        "{}",
        1_000_000_000.to_formatted_string(&num_format::Locale::fr)
    );
    println!(
        "{}",
        1_000_000_000.to_formatted_string(&num_format::Locale::en_IN)
    );

    println!("============");
    println!("{}", format!("{:.5}", 100.02)); // 保留 五位小说，四舍五入
    println!("{}", format!("{:.5}", 100.02781839));
    println!("{}", format!("{:<20}", "Rust is awesome"));
    println!("{}", format!("{:^20}", "Rust is awesome"));
    println!("{}", format!("{:>20}", "Rust is awesome"));
    println!("{}", format!("{:*^20}", "Rust is awesome"));
}
