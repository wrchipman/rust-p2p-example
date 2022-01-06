const STORAGE_FILE_PATH: &str = "./recipes.json";

type Result<t> = std::result::Result<T, Box<dyn std:error:Error + Send + Sync + 'static'>>;
static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

type Recipes = Vec<Recipe>;

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: String,
    instructions: String,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse { 
    mode: ListMode,
    data: Recipes,
    receiver: String,
}

enum EventType {
    Response(ListResponse),
    Input(String),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X25519Spec>::new().into_authentic(&KEYS).expect("can create auth keys");
    
}