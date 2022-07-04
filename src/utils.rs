use rand::{Rng, thread_rng};

pub struct Helpers<'a> {
    covered_items: Vec<&'a str>
}

pub fn create_random_id(token_length: usize) -> String {
    let mut chars_list: String = String::from("abcdefghijklmnopqrstuvwxyz0123456789.");
    let mut token: Vec<u8> = vec![];
    loop {
        let end_loop_condition: bool = token.len().eq(&token_length);
        if end_loop_condition {
            break
        } else {
            let mut random_thread = thread_rng();
            let mut random_index: usize = random_thread.gen_range(0..chars_list.len());
            let char_at_index = chars_list.bytes().nth(random_index).unwrap();
            token.push(char_at_index);
        }
    }
    let str_token = String::from_utf8(token).unwrap();
    str_token
}

