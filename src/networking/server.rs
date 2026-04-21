use tokio::net::TcpStream;
use crate::networking::user::User;

pub struct Server{
	stream: TcpStream,
	users: Vec<User>,
}