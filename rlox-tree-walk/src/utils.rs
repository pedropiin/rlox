pub const FUNCTION_MODE: u8 = 1;
pub const METHOD_MODE: u8 = 2;

pub fn parse_escape_sequences(og_str: &str) -> String {
    let mut result = String::with_capacity(og_str.len());
    let mut chars = og_str.chars();

    while let Some(chr) = chars.next() {
        if chr == '\\' && let Some(next_chr) = chars.next() {
            match next_chr {
                'n'  => result.push('\n'),
                't'  => result.push('\t'),
                'r'  => result.push('\r'),
                '\\' => result.push('\\'),
                c  => {result.push('\\'); result.push(c); }
            }
        } else { result.push(chr); }
    }

    result
}

pub fn get_callable_kind(mode: u8) -> String {
    if mode == FUNCTION_MODE { "function".to_string() }
    else { "method".to_string() }
}