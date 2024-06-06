use std::fs;
use regex::Regex;
use users::{ get_user_by_uid, get_current_uid };

fn main() {
  const OS_DETAILS_FILE_PATH: &str = "/etc/os-release";
  let contents: String = fs::read_to_string(OS_DETAILS_FILE_PATH).expect("not linux");

  let re = Regex::new(r#""([^"]*)""#).unwrap();
  let mut os_name = String::new();

  for (idx, line) in contents.lines().enumerate() {
    if idx >= 2 {
      break;
    }

    if let Some(cap) = re.captures(&line) {
      if !os_name.is_empty() {
        os_name.push(' ');
      }

      os_name.push_str(&cap[1]);
    }
  }

  println!("{}", os_name);

  let user = get_user_by_uid(get_current_uid()).unwrap();
  println!("Hello, {}", user.name().to_str().unwrap());
}
