#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(value) => {
                stack.push(*value);
            }
            // Operand
            _ => {
                // If there aren't at least two values on the stack, return None
                if stack.len() < 2 {
                    return None;
                }

                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());

                // Inverse the order of the operands becase the stack is LIFO
                match input {
                    CalculatorInput::Add => stack.push(b + a),
                    CalculatorInput::Subtract => stack.push(b - a),
                    CalculatorInput::Multiply => stack.push(b * a),
                    CalculatorInput::Divide => stack.push(b / a),
                    _ => return None,
                }
            }
        }
    }

    return if stack.len() == 1 {
        Some(stack.pop().unwrap())
    } else {
        None
    };
}
