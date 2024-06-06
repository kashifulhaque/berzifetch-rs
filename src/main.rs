use users::{ get_user_by_uid, get_current_uid };

fn main() {
  let user = get_user_by_uid(get_current_uid()).unwrap();
  println!("Hello, {}", user.name().to_str().unwrap());
}