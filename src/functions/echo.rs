pub fn echoing<'a>(args: impl Iterator<Item = &'a str>) {
    let joined: String = args.collect::<Vec<_>>().join(" ");
    let mut chars = joined.chars().peekable();
    let mut result = String::new();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('a') => result.push('\x07'), // bell
                Some('\\') => result.push('\\'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    println!("{}", result);
}
