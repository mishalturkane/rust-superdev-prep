#[derive(Debug, Clone)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
}

pub fn options() {
    let users = vec![
        User { id: 1, name: String::from("Mishal") },
        User { id: 2, name: String::from("Rahul") },
        User { id: 3, name: String::from("Aman") },
    ];
    
    let result = find_user(&users, 4);
  
      match result {
          Some(user) => println!("Found user: {:?}", user),
          None => println!("User not found"),
      }
}

fn find_user(users: &Vec<User>, user_id: u32) -> Option<User> {
    for user in users {
        if user.id == user_id {
            return Some(user.clone());
        }
    }
    None
}