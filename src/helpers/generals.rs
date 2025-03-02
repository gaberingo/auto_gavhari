use crate::models::general::llm::Message;

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) {
    let ai_function_str = ai_func(func_input);
}

#[cfg(test)]
mod tests {
    use super::*;
}
