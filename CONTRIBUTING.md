Run the frontend:
```
cd src/frontend
npm run dev
```

Run the backend:
```
cd src/rust/bonk-http
cargo run -- --cfg ../../../example/Bonk.toml
```

(Optional) Run ollama for Chat feature:
FIXME: I think you also first need to download the `codellama:7b` model, but I don't remember how right now
```
ollama serve
```