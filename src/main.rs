use std::{fs, os::unix::ffi::OsStringExt};
use regex::Regex;
use users::{ get_user_by_uid, get_current_uid };

fn main() {
  // Get the current OS name
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

  // Get the current username
  let user = get_user_by_uid(get_current_uid()).unwrap();
  println!("Hello, {}", user.name().to_str().unwrap());

  // Get the current hostname
  use libc::{ c_char, sysconf, _SC_HOST_NAME_MAX };
  use std::ffi::OsString;

  let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) };
  let mut buffer = vec![0; (hostname_max as usize) + 1];
  let returncode = unsafe{ libc::gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) };

  if returncode != 0 {
    println!("Failed to fetch hostname");
  }

  let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
  buffer.truncate(end);
  println!("{}", OsString::from_vec(buffer).to_str().unwrap());
}
