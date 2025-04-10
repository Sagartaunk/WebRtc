# WebRtc

```markdown
# WebRtc Server

This is the server-side implementation of a WebRTC project. The client-side will be developed in another branch of this GitHub repository.

## How to Run the Server

1. Navigate to the `WebRtc` folder:
   ```bash
   cd WebRtc
   ```
2. Run the server using Cargo:
   ```bash
   cargo run
   ```

## Steps to Use the Server with `curl` Commands

### Step 1: Register a User
If this is the first time running the server or you want to create additional users, use the following command to register a user:

```bash
curl -X POST http://your_ip_address:6969/register \
-H "Content-Type: application/json" \
-d '{"key": "your_pairing_key", "username": "new_user", "password": "new_password"}'
```

- **`your_pairing_key`**: You can find the pairing key in the `key.txt` file. This file is generated when you run the server, and the key resets every time you create a new user.

### Step 2: Get an Authorization Token
To access secure routes, you need to log in and obtain a token. Use the following command:

```bash
curl -X POST http://your_ip_address:6969/login \
-H "Content-Type: application/json" \
-d '{"username": "new_user", "password": "new_password"}'
```

- Replace `new_user` and `new_password` with the credentials you registered in Step 1.

### Step 3: Access Secure Routes
Use the token obtained in Step 2 to access secure routes. For example, to access the `/api/test` route:

```bash
curl -X GET http://your_ip_address:6969/api/test \
-H "Authorization: Bearer $token"
```

- Replace `$token` with the token you received from the login step.

## Notes
- The token is valid for **1 hour** after generation.
- The server's IP address will be displayed in the terminal when the server starts.
- This project is currently under development, and additional features will be added soon.

Stay tuned for updates!
```