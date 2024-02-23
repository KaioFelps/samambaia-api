pub fn exec(repository_name: &Vec<&str>) -> String {
    let mut repository_capitalized_name = String::new();

    for name in repository_name {
        let name_chars: Vec<char> = name.chars().collect();
        let (first_char, name_chars) = name_chars.split_first().unwrap();

        repository_capitalized_name.push(first_char.to_ascii_uppercase());

        for c in name_chars.to_owned() {
            repository_capitalized_name.push(c);
        }
    }

    repository_capitalized_name
}