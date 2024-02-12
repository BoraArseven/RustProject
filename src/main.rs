use std::str;



// I have found that builder pattern is a good match for that logfile project, since I think it is valuable to be maintainable, so with builder pattern we can change the structure of the logs easier.
// So, I am using my old builder pattern trial project as a starting template, with changes.
fn main() {
let user:User = UserBuilder::new().setemailuser("bora.arseven@gmail.com".parse().unwrap()).setpw("122134".parse().unwrap()).setusername("Zerks".parse().unwrap()).build();
println!("{:?}", user);
}

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    password: String
}
struct UserBuilder {
    email: String,
    username: String,
    password: String
}
impl User {

    fn new(email: String, username: String, password: String) -> UserBuilder{

        UserBuilder{
            email,
            username,
            password
        }

    }

}

impl UserBuilder {
    pub fn new(/* ... */) -> UserBuilder {
        // Set the minimally required fields of Foo.
        UserBuilder {
            // I am not confident with the unwrap and parse functions, but I know unwap is for having a value from an option.
            email : "default@gmail.com".parse().unwrap(),
            username : "default".parse().unwrap(),
            password : "123456".parse().unwrap()
        }
    }
    fn setemailuser(&mut self, email:String) -> &mut Self{
 self.email = email;
        self
    }
    fn setusername (&mut self, username:String) -> &mut Self{
        self.username = username;
        self
    }
    fn setpw (&mut self, password:String) -> &mut Self{
        self.password = password;
        self
    }
    fn build(&mut self) -> User{
        User {
            username: self.username.clone(),
            email:self.email.clone(),
            password:self.password.clone()
        }
    }
}