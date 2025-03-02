use crate::models::general::llm::Message;

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    let msg = format!(
        "Function: {}
INSTRUCTION: You are a function printer. You ONLY print the result of the functions. 
Nothing else. No commentary. Here is the input to the function {}.
OUTPUT: Print out what the function will return",
        ai_function_str, func_input
    );

    Message {
        role: "System".to_string(),
        content: msg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "todo app");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "System".to_string());
    }
}
