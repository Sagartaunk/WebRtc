use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode , decode , Header , Validation , EncodingKey , DecodingKey , errors::Result , TokenData};
use std::time::{SystemTime , UNIX_EPOCH};
use std::fs;
use actix_web::{web  , App , HttpServer , HttpResponse , Responder , post};
use std::io;
use rand::Rng;


#[derive(Debug , Clone , Serialize , Deserialize)]
struct User {
    id : i32,
    username : String,
    password : String,
}


#[derive(Debug , Serialize , Deserialize)]
struct Claims {
    sub : String,
    exp : usize
}

#[derive(Deserialize)]
struct LoginRequest {
    username : String,
    password : String,
}

pub async fn run(choice : u8) {
    match choice {
        1  => {add_device();},
        2  => {
            start_server().await;

        }, // run equence here later 
        _ => {} //wont have to choice anyway 
    }
}

async fn start_server() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(login)
    }).bind("192.168.0.1.29:80")?
    .run()
    .await
}

fn create_jwt(username : &str , secret : &[u8]) -> Result<String> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600; 
    let claims = Claims {
        sub : username.to_string(),
        exp : exp as usize,
    };
    encode(&Header::default() , &claims , &EncodingKey::from_secret(secret) )
}

fn verify_jwt(token : &str , secret : &[u8] ) -> Result<TokenData<Claims>> {
    decode::<Claims>(token , &DecodingKey::from_secret(secret) , &Validation::default())
}

fn load_credentials() -> Vec<User> {
    let file = fs::read_to_string("credentials.json").expect("Unable to read file");
    let users: Vec<User> = serde_json::from_str(&file).expect("Unable to parse JSON");
    users
}

fn save_credentials(users: &Vec<User>) {
    match fs::File::open("credentials.json"){
        Ok(_) => {},
        Err(_) => {let _ = fs::File::create("credentials.json");}
    }
    let json = serde_json::to_string(users).expect("Unable to convert to JSON");
    fs::write("credentials.json", json).expect("Unable to write file");
}

fn add_device() {
    let users = load_credentials();
    let mut username = String::new();
    let mut password = String::new();
    println!("Enter username: ");
    io::stdin().read_line(&mut username).expect("Failed to read line");
    println!("Enter password: ");
    io::stdin().read_line(&mut password).expect("Failed to read line");
    let id = rand::rng().random_range(1..=130);
    username = username.trim().to_string();
    password = password.trim().to_string();
    if username.is_empty() || password.is_empty() {
        println!("Username and password cannot be empty");
        return;
    }
    let new_user = User {
        id: id,
        username: username,
        password: password,
    };
    let mut users = users;
    users.push(new_user);
    save_credentials(&users);
}

#[post("/login")]
async fn login(login_info : web::Json<LoginRequest>) -> impl Responder {
    let users = load_credentials();
    let mut result = HttpResponse::Unauthorized().body("Invalid credentials");
    for credentials in users.iter() {
        let username = credentials.username.clone();
        let password = credentials.password.clone();
        let id = credentials.id as u8;
        if login_info.username == username && login_info.password == password {
            let token = create_jwt(&login_info.username , &[id]).unwrap();
            result =  HttpResponse::Ok().json(token);
        }

    }
    result
}

