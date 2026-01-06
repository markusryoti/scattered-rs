use scattered_rs::Store;

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

use kvstore::kv_store_server::{KvStore, KvStoreServer};
use kvstore::{GetRequest, GetResponse, PutRequest, PutResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

struct Node {
    store: Store,
}

impl Node {
    pub fn new() -> Self {
        Node {
            store: Store::new(),
        }
    }
}

#[tonic::async_trait]
impl KvStore for Node {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        let req = request.into_inner();
        println!("Received PUT: {} = {}", req.key, req.value);

        let reply = PutResponse {
            success: true,
            leader_id: "node_1".to_string(),
            error: "".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        println!("Received GET: {}", req.key);

        let reply = GetResponse {
            value: Some("Hello from Rust!".to_string()),
            leader_id: "node_1".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let kv_node = Node::new();

    println!("KV Server listening on {}", addr);

    Server::builder()
        .add_service(KvStoreServer::new(kv_node))
        .serve(addr)
        .await?;

    Ok(())
}
