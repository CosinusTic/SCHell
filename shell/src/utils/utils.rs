pub fn str_to_vec(input: &str) -> Vec<&str> {
    input.trim().split(" ").collect::<Vec<&str>>()
}

pub fn trim_eol(input: &str) -> Vec<&str> {
    input.split("\n").collect::<Vec<&str>>()
}
