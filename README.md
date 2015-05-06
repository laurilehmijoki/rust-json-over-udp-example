# A JSON-over-UDP example in Rust

## Setup

Install Rust and netcat

## Usage

Open the terminal and type:

    cargo run

Then in another terminal type:

    nc -u 127.0.0.1 34254 # nc is netcat
    {"recipient":"lauri","content":"foo"}

After entering the above JSON string, press `enter`. That will send the JSON
over the wire to our example program. Our example program will reply to you in
JSON.

## License

MIT
