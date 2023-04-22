
struct TemplateVariable {
    pub identifier: String,
}

pub fn render(content: String, context: serde_json::Value) -> String {
    // This function could be better, doing everything in one iteration, but atm
    // im leaving it like this
    
    let mut template_vars = Vec::new();
    for (idx, slice) in content.split("{{").enumerate() {
        if idx == 0 {
            continue;
        }
        let identifier = slice
            .split("}}")
            .nth(0)
            .expect("internal rendering error")
            .to_string();
        template_vars.push(TemplateVariable { identifier });
    }

    template_vars.iter().fold(content, |acc, var| {
        acc.replace(&format!("{{{{{}}}}}", var.identifier), {
            &context
                .get(var.identifier.trim().to_owned())
                .expect("Could not get var from context")
                .as_str()
                .expect("Serde::Value was not a string")
        })
    })
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
}
