//! # Logic Gate Simulator
//!
//! This module implements a simple interactive logic gate simulator that allows users
//! to create and test various types of digital logic gates.
//!
//! ## Available Gate Types
//!
//! - **AND Gate**: Outputs true only when both inputs are true
//! - **OR Gate**: Outputs true when at least one input is true
//! - **XOR Gate**: Outputs true when inputs are different
//! - **NAND Gate**: Outputs false only when both inputs are true
//! - **NOR Gate**: Outputs true only when both inputs are false
trait GateLogic {
    fn output(&self) -> bool;
}

struct AndGate {
    a: bool,
    b: bool,
}

impl GateLogic for AndGate {
    fn output(&self) -> bool {
        self.a && self.b
    }
}

struct OrGate {
    a: bool,
    b: bool,
}

impl GateLogic for OrGate {
    fn output(&self) -> bool {
        self.a || self.b
    }
}

struct XorGate {
    a: bool,
    b: bool,
}

impl GateLogic for XorGate {
    fn output(&self) -> bool {
        self.a ^ self.b
    }
}

struct NandGate {
    a: bool,
    b: bool,
}

impl GateLogic for NandGate {
    fn output(&self) -> bool {
        !(self.a && self.b)
    }
}

struct NorGate {
    a: bool,
    b: bool,
}

impl GateLogic for NorGate {
    fn output(&self) -> bool {
        !(self.a || self.b)
    }
}

fn create_gate(gate_type: &str, a: bool, b: bool) -> Option<Box<dyn GateLogic>> {
    match gate_type {
        "and" => Some(Box::new(AndGate { a, b })),
        "or" => Some(Box::new(OrGate { a, b })),
        "xor" => Some(Box::new(XorGate { a, b })),
        "nand" => Some(Box::new(NandGate { a, b })),
        "nor" => Some(Box::new(NorGate { a, b })),
        _ => None,
    }
}

fn prompt_for_gate() -> String {
    let mut input = String::new();
    loop {
        input.clear();

        println!("Enter the type of gate you want to create (and, or, xor, nand, nor): ");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Failed to read line: {}", e);
            continue;
        }

        match input.trim().to_lowercase().as_str() {
            "and" | "or" | "xor" | "nand" | "nor" => {
                return input.trim().to_lowercase().to_string()
            }
            _ => {
                eprintln!("Invalid gate type. Please enter and, or, xor, nand, or nor.");
                continue;
            }
        }
    }
}

fn prompt_for_input(prompt: &str) -> bool {
    let mut input = String::new();
    loop {
        input.clear();

        println!("{}", prompt);
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Failed to read line: {}", e);
            return false;
        }

        match input.trim() {
            "1" => return true,
            "0" => return false,
            _ => {
                eprintln!("Invalid input. Please enter 1 or 0.");
                continue;
            }
        }
    }
}

fn main() {
    let gate_type = prompt_for_gate();
    let input_a = prompt_for_input("Enter the value for input A (1 or 0): ");
    let input_b = prompt_for_input("Enter the value for input B (1 or 0): ");
    match create_gate(&gate_type, input_a, input_b) {
        Some(gate) => {
            println!("Result: {}", gate.output());
        }
        None => {
            eprintln!("Failed to create gate.");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_gate_returns_and_gate_for_and_input() {
        let gate = create_gate("and", true, false);
        assert!(gate.is_some());
        assert!(!gate.unwrap().output());

        let gate = create_gate("and", true, true);
        assert!(gate.is_some());
        assert!(gate.unwrap().output());
    }

    #[test]
    fn create_gate_returns_or_gate_for_or_input() {
        let gate = create_gate("or", false, false);
        assert!(gate.is_some());
        assert!(!gate.unwrap().output());

        let gate = create_gate("or", true, false);
        assert!(gate.is_some());
        assert!(gate.unwrap().output());
    }

    #[test]
    fn create_gate_returns_xor_gate_for_xor_input() {
        let gate = create_gate("xor", true, true);
        assert!(gate.is_some());
        assert!(!gate.unwrap().output());

        let gate = create_gate("xor", true, false);
        assert!(gate.is_some());
        assert!(gate.unwrap().output());
    }

    #[test]
    fn create_gate_returns_nand_gate_for_nand_input() {
        let gate = create_gate("nand", true, true);
        assert!(gate.is_some());
        assert!(!gate.unwrap().output());

        let gate = create_gate("nand", false, false);
        assert!(gate.is_some());
        assert!(gate.unwrap().output());
    }

    #[test]
    fn create_gate_returns_nor_gate_for_nor_input() {
        let gate = create_gate("nor", false, false);
        assert!(gate.is_some());
        assert!(gate.unwrap().output());

        let gate = create_gate("nor", true, false);
        assert!(gate.is_some());
        assert!(!gate.unwrap().output());
    }

    #[test]
    fn create_gate_returns_none_for_invalid_input() {
        assert!(create_gate("invalid", true, false).is_none());
        assert!(create_gate("", true, false).is_none());
        assert!(create_gate("AND", true, false).is_none()); // Case sensitive check
    }
    #[test]
    fn and_gate_output_returns_true_when_both_inputs_are_true() {
        let gate = AndGate { a: true, b: true };
        assert!(gate.output());
    }

    #[test]
    fn and_gate_output_returns_false_when_any_input_is_false() {
        let gate = AndGate { a: true, b: false };
        assert!(!gate.output());

        let gate = AndGate { a: false, b: true };
        assert!(!gate.output());

        let gate = AndGate { a: false, b: false };
        assert!(!gate.output());
    }

    #[test]
    fn or_gate_output_returns_true_when_any_input_is_true() {
        let gate = OrGate { a: true, b: false };
        assert!(gate.output());

        let gate = OrGate { a: false, b: true };
        assert!(gate.output());

        let gate = OrGate { a: true, b: true };
        assert!(gate.output());
    }

    #[test]
    fn or_gate_output_returns_false_when_both_inputs_are_false() {
        let gate = OrGate { a: false, b: false };
        assert!(!gate.output());
    }

    #[test]
    fn xor_gate_output_returns_true_when_inputs_are_different() {
        let gate = XorGate { a: true, b: false };
        assert!(gate.output());

        let gate = XorGate { a: false, b: true };
        assert!(gate.output());
    }

    #[test]
    fn xor_gate_output_returns_false_when_inputs_are_same() {
        let gate = XorGate { a: true, b: true };
        assert!(!gate.output());

        let gate = XorGate { a: false, b: false };
        assert!(!gate.output());
    }

    #[test]
    fn nand_gate_output_returns_false_when_both_inputs_are_true() {
        let gate = NandGate { a: true, b: true };
        assert!(!gate.output());
    }

    #[test]
    fn nand_gate_output_returns_true_when_any_input_is_false() {
        let gate = NandGate { a: true, b: false };
        assert!(gate.output());

        let gate = NandGate { a: false, b: true };
        assert!(gate.output());

        let gate = NandGate { a: false, b: false };
        assert!(gate.output());
    }

    #[test]
    fn nor_gate_output_returns_true_when_both_inputs_are_false() {
        let gate = NorGate { a: false, b: false };
        assert!(gate.output());
    }

    #[test]
    fn nor_gate_output_returns_false_when_any_input_is_true() {
        let gate = NorGate { a: true, b: false };
        assert!(!gate.output());

        let gate = NorGate { a: false, b: true };
        assert!(!gate.output());

        let gate = NorGate { a: true, b: true };
        assert!(!gate.output());
    }
}
