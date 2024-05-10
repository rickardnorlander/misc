
#![allow(unused_variables)]

use once_cell::sync::Lazy;
use regex::Regex;
use std::io;
use std::io::BufRead;

use std::collections::HashMap;

fn skip_space_and_newlines(s: &mut &str) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[ \n]+").unwrap());
    let m = RE.captures(s);
    if let Some(caps) = m {
        *s = &s[caps[0].len()..];
    }
}

fn parse_token(s: &mut &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\(|\)|'|[^ \n\(\)']+)").unwrap());
    let m = RE.captures(s);
    if let Some(caps) = m {
        *s = &s[caps[0].len()..];
        return caps[0].to_string();
    }
    panic!();
}

fn get_code() -> String {
    let fizzbuzz = r#"
    0 a ! 0 b ! 0 c !
    
    ((()) 'd !) 'resetd !
    (a 3 = (d fizz + 'd ! 0 'a !) if) 'addfizz !
    (b 5 = (d buzz + 'd ! 0 'b !) if) 'addbuzz !
    (d () = (d c + 'd !) if) 'fallback !

    (resetd addfizz addbuzz fallback d print) 'printone !

    (c 100 <) (a 1 + 'a ! b 1 + 'b ! c 1 + 'c ! printone) loop
"#;

    let factorial = r#"

    (dup 1 > (dup 1 - factorial *) if) 'factorial !
    5 factorial print
    "#;


    let avoiding_injection = r#"
    'hunter2 secretpassword !
    (How old are you?) print
    read 'age !
    (This variant would be vulnerable as it would evaluate age
        (Ah so you are ) age + print
    ) drop
    (Ah so you are ) 'age @ + print
    "#;

    let repl = r#"
    (true) (read eval depth 0 > (dup print) if) loop
    "#;

    return repl.to_string();
}

struct Context {
    variables: HashMap::<String, String>,
    stack: Vec::<String>,
}

fn eval(co: &mut Context, in_s: &str) {
    let mut s_x: &str = in_s;
    let s: &mut &str = &mut s_x;
    let mut quote = false;
    loop {
        skip_space_and_newlines(s);
        if s.is_empty() {
            break;
        }
        let token = parse_token(s);
        if quote {
            co.stack.push(token);
            quote = false;
            continue;
        }

        if let Some(v) = co.variables.get(&token) {
            let vcopy = v.clone();
            eval(co, &vcopy);
            continue;
        }
        match token.as_str() {
            "(" => {
                let mut curstr = String::new();
                let mut paren_level = 1;
                while let Some(c) = s.chars().next() {
                    *s = &s[c.len_utf8()..];
                    match c {
                        '(' => {
                            paren_level += 1;
                        }
                        ')' => {
                            paren_level -= 1;
                            if paren_level == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    curstr.push(c);
                }
                co.stack.push(curstr);
            },
            ")" => panic!(),
            "+" => {
                let b = co.stack.pop().unwrap();
                let a = co.stack.pop().unwrap();
                let d = b.parse::<f64>();
                let c = a.parse::<f64>();
                if c.is_ok() && d.is_ok() {
                    co.stack.push((c.unwrap()+d.unwrap()).to_string())
                } else {
                    co.stack.push(format!("{}{}", a, b));
                }}
            "*" => {let b = co.stack.pop().unwrap().parse::<f64>().unwrap(); let a = co.stack.pop().unwrap().parse::<f64>().unwrap(); co.stack.push((a * b).to_string())}
            "-" => {let b = co.stack.pop().unwrap().parse::<f64>().unwrap(); let a = co.stack.pop().unwrap().parse::<f64>().unwrap(); co.stack.push((a - b).to_string())}
            "/" => {let b = co.stack.pop().unwrap().parse::<f64>().unwrap(); let a = co.stack.pop().unwrap().parse::<f64>().unwrap(); co.stack.push((a / b).to_string())}
            ">" => {let b = co.stack.pop().unwrap().parse::<f64>().unwrap(); let a = co.stack.pop().unwrap().parse::<f64>().unwrap(); co.stack.push(if a > b {"yup".to_string()} else {"".to_string()})}
            "<" => {let b = co.stack.pop().unwrap().parse::<f64>().unwrap(); let a = co.stack.pop().unwrap().parse::<f64>().unwrap(); co.stack.push(if a < b {"yup".to_string()} else {"".to_string()})}
            "=" => {let b = co.stack.pop().unwrap(); let a = co.stack.pop().unwrap(); co.stack.push(if a == b {"yup".to_string()} else {"".to_string()})}
            "'" => {quote = true;}
            "read" => {let line = io::stdin().lock().lines().next().unwrap().unwrap(); co.stack.push(line);}
            "print" => {println!("{}",co.stack.pop().unwrap());}
            "dup" => {co.stack.push(co.stack.last().unwrap().clone());}
            "eval" => {let v = co.stack.pop().unwrap(); eval(co, &v);}
            "if" => {
                let body = co.stack.pop().unwrap();
                let condition = co.stack.pop().unwrap();
                if !condition.is_empty() {
                    eval(co, &body);
                }
            }
            "@" => {let v = co.variables.get(&co.stack.pop().unwrap()).unwrap(); co.stack.push(v.clone());}
            "drop" => {co.stack.pop().unwrap();}
            "[]" => {
                // This is O(n) because I'm too lazy to figure out how to do it properly.
                let b = co.stack.pop().unwrap().parse::<usize>().unwrap(); let a = co.stack.pop().unwrap(); co.stack.push((a.chars().nth(b).unwrap() as u32).to_string())
            }
            "char" => {let c = char::from_u32(co.stack.pop().unwrap().parse::<u32>().unwrap()).unwrap(); co.stack.push(c.to_string());}
            "len" => {let b = co.stack.pop().unwrap(); co.stack.push(b.chars().count().to_string());}
            "depth" => {let depth = co.stack.len(); co.stack.push(depth.to_string());
            }
            "loop" => {
                let body = co.stack.pop().unwrap();
                let condition = co.stack.pop().unwrap();
                loop {
                    eval(co, &condition);
                    let ok = co.stack.pop().unwrap();
                    if ok.is_empty() {
                        break;
                    }
                    eval(co, &body);
                }
            }
            "!" => {let fn_name = co.stack.pop().unwrap(); co.variables.insert(fn_name, co.stack.pop().unwrap());}
            _ => {co.stack.push(token.clone());}
        }
    }
}

fn main() {
    let mut context = Context{variables: HashMap::<String, String>::new(), stack: Vec::<String>::new()};
    let code = get_code();
    eval(&mut context, &code);
    //println!("{:?}", context.stack)
}
