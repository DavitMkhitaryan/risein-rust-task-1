fn main() {
    let s1 = String::from("Str1");
    let s2 = String::from("Str2");

    let concatenated_string = concatenate_strings(&s1, &s2);
    println!("Concatenated string is {}", concatenated_string);
}

fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result: String = String::from("");
    result.push_str(s1);
    result.push_str(s2);
    result
}