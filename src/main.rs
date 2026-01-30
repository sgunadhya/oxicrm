use axum::{routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

mod application;
mod domain;
mod infrastructure;
mod shared;

use application::ports::{
    identity::IdentityProvider, input::RecordUseCase, output::OpportunityRepository, time::Clock,
};
use application::use_cases::RecordBoardCard;
use infrastructure::persistence::sea_orm_repo::SeaOrmRepo;
use infrastructure::web::handlers::{get_board_handler, move_card_handler, AppState};
// New Adapters
use infrastructure::billing::MockBillingProvider;
use infrastructure::external::MockWebhookSender;
use infrastructure::identity::MockIdentityProvider;
use infrastructure::messaging::InMemoryEventBus;
use infrastructure::scheduling::InMemoryJobQueue;
use infrastructure::search::MockSearchIndex;
use infrastructure::storage::FileSystemStorage;
use infrastructure::time::SystemClock;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // 1. Database Connection
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let db = Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // 2. Run Migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    // 3. Initialize Adapters
    let repo = Arc::new(SeaOrmRepo { db });
    let event_bus = Arc::new(InMemoryEventBus::new());

    // Logic for Job Queue Receiver would go here in real app, spawning a worker
    let (job_sender, _job_receiver) = mpsc::channel(100);
    let job_queue = Arc::new(InMemoryJobQueue::new(job_sender));

    let clock = Arc::new(SystemClock);
    let identity_provider = Arc::new(MockIdentityProvider);
    let search_index = Arc::new(MockSearchIndex);
    let webhook_sender = Arc::new(MockWebhookSender);
    let billing_provider = Arc::new(MockBillingProvider);
    let storage_provider = Arc::new(FileSystemStorage::new(std::path::PathBuf::from("storage")));

    // 4. Initialize Use Cases
    // Note: RecordBoardCard struct needs update to accept these new dependencies if we want to use them.
    // For now, adhering to the existing struct definition which only required repo.
    // If we want to use them, we'd update RecordBoardCard.
    let record_use_case = Arc::new(RecordBoardCard {
        opportunity_repo: repo.clone(),
    });

    // 5. Initialize App State
    let app_state = AppState {
        record_use_case: record_use_case.clone(),
    };

    // Seed Data if empty
    let opps = record_use_case.list_opportunities().await.unwrap_or(vec![]);
    if opps.is_empty() {
        println!("Seeding initial data...");
        let _ = repo
            .save(&domain::Opportunity::new(
                "Tech Corp Deal".to_string(),
                domain::OpportunityStage::New,
                1000000,
            ))
            .await;
        let _ = repo
            .save(&domain::Opportunity::new(
                "Startup License".to_string(),
                domain::OpportunityStage::Meeting,
                50000,
            ))
            .await;
        let _ = repo
            .save(&domain::Opportunity::new(
                "Enterprise Renewal".to_string(),
                domain::OpportunityStage::Proposal,
                2500000,
            ))
            .await;
    }

    // 6. Build Router
    let app = Router::new()
        .route("/", get(get_board_handler))
        .route("/cards/:id/move", axum::routing::post(move_card_handler))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    // Prevent unused warnings for new adapters by "using" them in a print (scaffolding hack)
    println!("Services initialized: Time={:?}, Auth={:?}, Search={:?}, Webhook={:?}, Billing={:?}, Storage={:?}, EventBus={:?}, JobQueue={:?}",
             clock.now(), identity_provider.get_current_user_id().await, search_index, webhook_sender, billing_provider, storage_provider, event_bus, job_queue);

    axum::serve(listener, app).await.unwrap();
}
