use crate::util::StringUtils;

#[derive(Debug)]
enum Filter {
    Uppercase,
    Lowercase,
}

impl Filter {
    pub fn apply_on(&self, target: String) -> String {
        match self {
            Self::Uppercase => target.to_uppercase(),
            Self::Lowercase => target.to_lowercase(),
        }
    }
}

#[derive(Debug)]
enum Token {
    If,
    Endif,
    For,
    Endfor,
    Pipe,
    Variable(String),
    Filter(Filter),
}

#[derive(Debug)]
struct Value {
    tokens: Vec<Token>,
}

impl Value {
    pub fn render(&self, context: &serde_json::Value) -> String {
        let mut rendered_string = String::from("");
        let mut next_is_pipe = false;

        for token in self.tokens.iter() {
            match token {
                Token::Variable(identifier) => {
                    if next_is_pipe {
                        eprintln!("Trying to put variable in a filter position");
                        std::process::abort();
                    }
                    rendered_string = {
                        let result = context.get(identifier).expect(&format!(
                            "Variable {identifier} was not present in the context"
                        ));
                        match result {
                            serde_json::Value::String(res) => res.to_string(),
                            _ => {
                                eprintln!("Only strings are supported in context json for now");
                                std::process::abort();
                            }
                        }
                    };
                    next_is_pipe = true;
                }
                Token::Pipe => {
                    next_is_pipe = false;
                }
                Token::Filter(filter) => {
                    if next_is_pipe {
                        eprintln!("Trying to put variable in a filter position");
                        std::process::abort();
                    }
                    rendered_string = filter.apply_on(rendered_string);
                    next_is_pipe = true;
                },
                _ => {
                    eprintln!("Wrong syntax when trying to template variables!");
                    std::process::abort();
                }
            }
        }

        if !next_is_pipe {
            eprintln!("Missing filter operator");
            std::process::abort();
        }

        return rendered_string;
    }
}

#[derive(Debug)]
struct Logic {
    tokens: Vec<Token>,
}

#[derive(Debug)]
struct IfCondition(String);

#[derive(Debug)]
enum Code {
    Value(Value),
    Logic(Logic),
}

impl Code {
    pub fn render(&self, context: &serde_json::Value) -> String {
        match self {
            Self::Value(v) => v.render(context),
            Self::Logic(_) => "Logic block".to_string(),
        }
    }
}

#[derive(Debug)]
enum TemplateTreeNode {
    Literal(String),
    Code(Code),
}

impl TemplateTreeNode {
    pub fn render(&self, context: &serde_json::Value) -> String {
        match self {
            Self::Literal(text) => text.to_string(),
            Self::Code(code) => code.render(context),
        }
    }
}

#[derive(Debug)]
struct TemplateTree {
    nodes: Vec<TemplateTreeNode>,
}

impl TemplateTree {
    pub fn rendered(&self, context: &serde_json::Value) -> String {
        let mut output = String::from("");
        for node in self.nodes.iter() {
            output.push_str(&node.render(context))
        }
        return output;
    }
}

fn chunk_to_tokens(chunk: &String) -> Vec<Token> {
    let mut tokens = vec![];
    for word in chunk.trim().split(' ') {
        tokens.push(match word {
            "if" => Token::If,
            "endif" => Token::Endif,
            "|>" => Token::Pipe,
            "for" => Token::For,
            "endfor" => Token::Endfor,
            "upper" => Token::Filter(Filter::Uppercase),
            "lower" => Token::Filter(Filter::Lowercase),
            _ => Token::Variable(word.to_string()),
        });
    }
    tokens
}

fn build_code_node(chunk: &mut String) -> TemplateTreeNode {
    let is_variable = chunk.starts_then_ends_with("{{", "}}");

    if is_variable {
        return TemplateTreeNode::Code(Code::Value(Value {
            tokens: chunk_to_tokens(&chunk.without("{{").without("}}")),
        }));
    }

    let is_logic_line = chunk.starts_then_ends_with("{%", "%}");

    if is_logic_line {
        return TemplateTreeNode::Code(Code::Logic(Logic {
            tokens: chunk_to_tokens(&chunk.without("{%").without("%}")),
        }));
    }

    return TemplateTreeNode::Literal(chunk.to_string());
}

fn parse_tree(content: String) -> TemplateTree {
    let chunks = content.split_in_chunks_inclusive("{", "}");
    let mut nodes = Vec::new();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let is_code = i % 2 == 1;
        if is_code {
            nodes.push(build_code_node(&mut chunk.to_string()))
        } else {
            nodes.push(TemplateTreeNode::Literal(chunk.to_string()))
        }
    }

    return TemplateTree { nodes };
}

pub fn render(content: String, context: serde_json::Value) -> String {
    // This function could be better, doing everything in one iteration, but atm
    // im leaving it like this
    let node_tree = parse_tree(content);
    node_tree.rendered(&context)
}

pub fn render_file(path: &str, context: serde_json::Value) -> String {
    let file_content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!(
                "Could not read the contents of the file at the path {}, aborting",
                path
            );
            std::process::abort();
        }
    };

    render(file_content, context)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::render;

    #[test]
    fn it_works() {
        let result = render(
            "Your name is {{ nome }}.".to_string(),
            json!({ "nome": "Viktor" }),
        );
        assert_eq!(result, "Your name is Viktor.".to_string());
    }

    #[test]
    fn works_with_multiple_vars() {
        let result = render(
            "Hello, {{friendName}}. I am {{ name }}".to_string(),
            json!({ "name": "Viktor", "friendName": "John Doe" }),
        );
        assert_eq!(result, "Hello, John Doe. I am Viktor".to_string());
    }
}
