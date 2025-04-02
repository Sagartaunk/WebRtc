use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode , decode , Header , Validation , EncodingKey , DecodingKey , errors::Result , TokenData};
use std::time::{SystemTime , UNIX_EPOCH};
use std::fs;
use actix_web::{web , HttpServer , HttpResponse , Responder , post};
use rand::Rng;
use bcrypt::{hash, DEFAULT_COST};



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
pub struct LoginRequest {
    pub username : String,
    pub password : String,
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
    let file = fs::read_to_string("credentials.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&file).unwrap_or_else(|_| vec![])
}

fn save_credentials(users: &Vec<User>) {
    match fs::File::open("credentials.json"){
        Ok(_) => {},
        Err(_) => {let _ = fs::File::create("credentials.json");}
    }
    let json = serde_json::to_string(users).expect("Unable to convert to JSON");
    fs::write("credentials.json", json).expect("Unable to write file");
}

#[post("/register")]
pub async fn register(register_info : web::Json<LoginRequest>) -> impl Responder {
    let mut users = load_credentials(); 
    let id = rand::rng().random_range(1..=130);
    let new_user = User {
        id,
        username: register_info.username.clone(),
        password: register_info.password.clone(),
    };
    users.push(new_user);
    save_credentials(&users);

    HttpResponse::Ok().body("User registered successfully")
}

#[post("/login")]
pub async fn login(login_info : web::Json<LoginRequest>) -> impl Responder {
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

