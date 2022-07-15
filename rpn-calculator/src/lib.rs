use core::panic;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    match inputs.len() {
        0 | 2 => return None,
        1 => match inputs[0] {
            CalculatorInput::Value(num) => return Some(num),
            _ => return None,
        },
        _ => {
            let mut stack: Vec<i32> = Vec::new();

            for input in inputs {
                match input {
                    CalculatorInput::Value(num) => {
                        stack.push(*num);
                    }
                    _ => {
                        if stack.len() < 2 {
                            return None;
                        }

                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();

                        match input {
                            CalculatorInput::Add => stack.push(a + b),
                            CalculatorInput::Subtract => stack.push(a - b),
                            CalculatorInput::Divide => stack.push(a / b),
                            CalculatorInput::Multiply => stack.push(a * b),
                            _ => panic!("invalid data"),
                        }
                    }
                }
            }

            if stack.len() != 1 {
                return None;
            }

            return stack.pop();
        }
    }
}
