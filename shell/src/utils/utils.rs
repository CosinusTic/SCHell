pub fn str_to_vec(input: &str) -> Vec<&str> {
    input.trim().split(" ").collect::<Vec<&str>>()
}
