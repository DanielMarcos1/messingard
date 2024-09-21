pub mod chapter_1 {
    // There is also #[derive(Debug)] and fmt::Display which consists in formatting
    // the output to debug purposes, applies to structs, enums, and tuples
    pub fn how_many_days() {
        println!("How many days in a year? {}", 365);
    }

    pub fn how_many_days_month() {
        println!("How many days in a month? {0} or {1}", 30, 31);
    }

    pub fn sentence() {
        println!("{part1} {part2} {part3}",
                part1="This",    
                part2="is a",
                part3="sentence.");
    }

    pub fn counting_bases() {
        println!("Counting on base 10: {}", 0123456789);
        println!("Counting on base 2(binary): {:b}", 0123456789);
        println!("Counting on base 8(octal): {:o}", 0123456789);
        println!("Counting on base 16(hexadecimal): {:x}", 0123456789);
    }

    pub fn social_isolation() {
        // This function adds space before the space value
        println!("{space:>10}", space="im_sick");
    }

    pub fn a_thousand_before() {
        println!("{number:0>4}", number=1);
    }

    pub fn a_thousand_after() {
        println!("{number:0<4}", number=1);
    }

    pub fn a_thousand() {
        println!("{number:0<width$}", number=1, width=4);
    
        let number = 1;
        let width = 4;
        println!("{number:0<width$}");
        // Two ways to show a thousand =D
    }
}