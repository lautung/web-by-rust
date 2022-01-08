mod server;
mod handler;
mod router;

use server::Server;

fn main() {
    Server::new("localhost:3000").run();
}
