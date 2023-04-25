pub trait StringUtils {
    fn starts_then_ends_with(&self, start: &str, end: &str) -> bool;
    fn split_in_chunks_inclusive(&self, start: &str, end: &str) -> Vec<String>;
    fn without(&self, string: &str) -> Self;
}

impl StringUtils for String {
    fn starts_then_ends_with(&self, start: &str, end: &str) -> bool {
        self.starts_with(start) && self.ends_with(end)
    }

    fn without(&self, string: &str) -> Self {
        self.split(string)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn split_in_chunks_inclusive(&self, start: &str, end: &str) -> Vec<String> {
        let mut slices: Vec<String> = Vec::new();

        for (idx, chunk) in self.split(start).filter(|s| s.len() > 0).enumerate() {
            if idx == 0 && !self.starts_with(start) {
                slices.push(chunk.to_string());
                continue;
            }

            let rest = chunk
                .split(end)
                .filter(|s| s.len() > 0)
                .collect::<Vec<&str>>();

            let is_closed = rest.len() > 1;

            if !is_closed && !chunk.ends_with(end) {
                eprintln!("Missing a closing character {}", end);
                std::process::abort();
            }

            slices.push(format!(
                "{{{{{}}}}}",
                rest.get(0).expect("internal string manipulation error"),
            ));

            if rest.get(1).is_some() {
                slices.push(rest.get(1).expect("internal error").to_string());
            }
        }

        return slices;
    }
}
