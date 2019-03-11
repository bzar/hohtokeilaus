# Hohtokeilaus
A bowling game for Hohto, a competence management system

## Running
- Build the frontend: `cd frontend && npm install && node_modules/bin/ng build && cd ..`
- Create file `.env` to the project root with line `HOHTO_SESSION=session_id=<your hohto session key here>`
- Build and run the backend: `cargo run`
- Open `http://localhost:8080` to view the game
