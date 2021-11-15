mod block_chain;
mod web_server;

fn main() -> std::io::Result<()> {
    web_server::main()
}