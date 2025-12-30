
pub fn longest_common_prefix(words: &[String]) -> String {
    if words.is_empty() {
        return String::new();
    }

    let mut prefix = words[0].clone();

    for word in &words[1..] {
        while !word.starts_with(&prefix) {
            prefix.pop();

            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    prefix
}

pub fn tokenize_input(input : String) -> Vec<String> {
    let mut tokens : Vec<String> = Vec::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut backslash = false;
    let mut curr = String::new();

    for c in input.chars() {
        if backslash {
            if in_double_quotes && c != '\\' && c != '"' {
                curr.push('\\');
            }
            curr.push(c);
            backslash = false;
            continue;
        }

        match c {
            '\\' if !in_single_quotes => {
                backslash = true;
            },
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            },
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !curr.is_empty() {
                    tokens.push(curr.clone());
                    curr.clear();
                }
            },
            '\"' if !in_single_quotes && !backslash => {
                in_double_quotes = !in_double_quotes;
            },
            _ => curr.push(c),
        }
    } 

    if !curr.is_empty() {
        tokens.push(curr.clone());
    }

    tokens
}

pub fn split_commands(tokens : Vec<String>) -> Vec<Vec<String>> {

    let mut commands = Vec::new();
    let mut current = Vec::new();

    for token in tokens {
        if token == "|"  {
            commands.push(current);
            current = Vec::new();
        }else {
            current.push(token);
        }
    }

    if !current.is_empty()  {
        commands.push(current);
    }

    commands
}