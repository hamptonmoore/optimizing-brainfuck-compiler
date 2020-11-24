#[derive(Debug)]
enum Instruction {
    ChangePtr(i8),
    ChangeVal(i8),
    Loop(Vec::<Instruction>),
    Print,
    Input,
    Noop
}

fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    println!("Input: {}", &program);
    let (ast, _unused) = generate_ast(&program, true);
    println!("AST: {:?}", &ast);
    let optimized_ast:Vec<Instruction>  = optimize_ast(ast);
    println!("Optimized AST: {:?}", &optimized_ast);

    let generated_javascript = generate_javascript_from_ast(optimized_ast, true);
    println!("Javascript: {}", generated_javascript);
}

fn generate_javascript_from_ast(ast: Vec<Instruction>, base: bool) -> String {
    let mut output = match base {
        true => String::from("let d=new Uint8Array(128), p=0;"),
        false => String::from("")
    };

    for instruction in ast {
        match &instruction {
            Instruction::ChangePtr(val) => output.push_str(&format!("p+={};", val)),
            Instruction::ChangeVal(val) => output.push_str(&format!("d[p]+={};", val)),
            Instruction::Loop(_instructions) =>{
                if let Instruction::Loop(instructions) = instruction {
                    if instructions.len() > 0 {
                        output.push_str(&format!("while(d[p]!=0){{{}}}", &generate_javascript_from_ast(instructions, false)));
                    }
                }
            },
            Instruction::Print => output.push_str(&format!("console.log(String.fromCharCode(d[p]));")),
            Instruction::Input => output.push_str(&format!("d[p]=prompt('Type a char').charCodeAt(0)||0;")),
            _ => {}
        }
    }

    return output;
}

fn optimize_ast(ast: Vec<Instruction>) -> Vec::<Instruction> {
    let mut new_ast: Vec::<Instruction> = vec![];
    let mut noop = Instruction::Noop;

    let mut changed = false;

    for instruction in ast {
        let last_instruction = match new_ast.last_mut() {
            Some(last_instruction) => last_instruction,
            None => &mut noop
        };

        match &instruction {
            Instruction::ChangeVal(val) => {
                if *val == 0 {
                    continue;
                }
                match last_instruction {
                    &mut Instruction::ChangeVal(ref mut p_val) => {
                        *p_val += val;
                        changed = true;
                    },
                    _ => {
                        new_ast.push(instruction);
                    }
                }
            },
            Instruction::Loop(_instructions) =>{
                if let Instruction::Loop(instructions) = instruction {
                    if instructions.len() > 0 {
                        new_ast.push(Instruction::Loop(optimize_ast(instructions)));
                    }
                }
            },
            Instruction::ChangePtr(val) => {
                match last_instruction {
                    &mut Instruction::ChangePtr(ref mut p_val) => {
                        *p_val += val;
                        changed = true;
                    },
                    _ => {
                        new_ast.push(instruction);
                    }
                }
            },Instruction::Noop => {},
            _ => {
                new_ast.push(instruction);
            }
        }
    }

    return if changed {
        optimize_ast(new_ast)
    } else {
        new_ast
    }
}

fn generate_ast(program: &str, base: bool) -> (Vec::<Instruction>, usize) {
    let mut ast: Vec::<Instruction> = Vec::new();

    let mut skip_till: usize = 0;

    for (index, char) in program.chars().enumerate() {
        if index < skip_till {
            continue;
        }

       match char {
           '+' => ast.push(Instruction::ChangeVal(1)),
           '-' => ast.push(Instruction::ChangeVal(-1)),
           '>' => ast.push(Instruction::ChangePtr(1)),
           '<' => ast.push(Instruction::ChangePtr(-1)),
           '.' => ast.push(Instruction::Print),
           ',' => ast.push(Instruction::Input),
           '[' => {
               let (loop_instructions, loop_end_index) = generate_ast(&program[index+1..], false);
               skip_till = loop_end_index + index;
               ast.push(Instruction::Loop(loop_instructions));
           },
           ']' => {
               if !base {
                   return (ast, index+2)
               }
           },
           _ => {
               ast.push(Instruction::Noop)
           }
       }
    }

    (ast, 0)
}
