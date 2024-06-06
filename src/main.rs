use std::{ fs, os::unix::ffi::OsStringExt };
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

  println!("  OS Name:\t{}", os_name);

  // Get the current username
  let user = get_user_by_uid(get_current_uid()).unwrap();
  println!("  Username:\t{}", user.name().to_str().unwrap());

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
  println!("  Hostname:\t{}", OsString::from_vec(buffer).to_str().unwrap());

  // Get the linux kernel version
  let kernel_version_file = fs::read_to_string("/proc/version").expect("not linux");

  if let Some(version) = extract_kernel_version(&kernel_version_file) {
    println!("  Kernel:\t{}", version);
  } else {
    println!("Failed to fetch Linux kernel version");
  }

  let arch = std::env::consts::ARCH;

  println!("Architecture:\t{}", arch);

  if let Ok(cpu_info) = fs::read_to_string("/proc/cpuinfo") {
    for line in cpu_info.lines() {
      if line.starts_with("model name") {
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() > 1 {
          let cpu_name = parts[1].trim();
          println!("CPU Name:\t{}", cpu_name);
        }

        break;
      }
    }
  } else {
    println!("Failed to read /proc/cpuinfo");
  }
}

fn extract_kernel_version(version_info: &str) -> Option<&str> {
  if let Some(end_index) = version_info.find(" (") {
    return Some(&version_info[..end_index]);
  }

  None
}