use colored::*;
use regex::Regex;
use figlet_rs::FIGfont;
use std::{ env, fs, os::unix::ffi::OsStringExt };
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

      if idx == 1 {
        let std_font = FIGfont::standard().unwrap();
        let figure = std_font.convert(&cap[1]);
        println!("{}", figure.unwrap());
      }
    }
  }

  println!("{}\n  {}", " OS Name".yellow().bold(), os_name.italic());

  // Get the current username
  let user = get_user_by_uid(get_current_uid()).unwrap();
  println!("{}\n  {}", " Username".yellow().bold(), user.name().to_str().unwrap().italic());

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
  println!("{}\n  {}", " Hostname".yellow().bold(), OsString::from_vec(buffer).to_str().unwrap().italic());

  // Get the linux kernel version
  let kernel_version_file = fs::read_to_string("/proc/version").expect("not linux");

  if let Some(version) = extract_kernel_version(&kernel_version_file) {
    println!("{}\n  {}", " Kernel".yellow().bold(), version.italic());
  } else {
    println!("Failed to fetch Linux kernel version");
  }

  // Get CPU architecture
  let arch = std::env::consts::ARCH;
  println!("{}\n  {}", "󰻠 Architecture".yellow().bold(), arch.italic());

  // Get CPU name
  if let Ok(cpu_info) = fs::read_to_string("/proc/cpuinfo") {
    for line in cpu_info.lines() {
      if line.starts_with("model name") {
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() > 1 {
          let cpu_name = parts[1].trim();
          println!("{}\n  {}", " CPU Name".yellow().bold(), cpu_name.italic());
        }

        break;
      }
    }
  } else {
    println!("Failed to read /proc/cpuinfo");
  }

  // Get total memory
  if let Ok(memory_info) = fs::read_to_string("/proc/meminfo") {
    for line in memory_info.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if let Ok(mem_total) = parts[1].parse::<u64>() {
        let mem_gb = mem_total as f64 / 1024.0 / 1024.0;
        let mem_gb = (mem_gb * 100.0).round() / 100.0;
        println!("{}\n  {} {}", "󰍛 Total Memory".yellow().bold(), mem_gb.to_string().italic(), "GB".italic());
      }
      break;
    }
  }

  // Get current shell
  match env::var("SHELL") {
    Ok(shell) => {
      let (_, shell_name) = shell.rsplit_once('/').unwrap();
      println!("{}\n  {}", " Shell:".yellow().bold(), shell_name.italic())
    },
    Err(_err) => (),
  }

  // Get Desktop Environment
  match env::var("XDG_CURRENT_DESKTOP") {
    Ok(de) => println!("{}\n  {}", "󰪫 DE:".yellow().bold(), de.italic()),
    Err(_err) => (),
  }
}

fn extract_kernel_version(version_info: &str) -> Option<&str> {
  if let Some(end_index) = version_info.find(" (") {
    return Some(&version_info[..end_index]);
  }

  None
}