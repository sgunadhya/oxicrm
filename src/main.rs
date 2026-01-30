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

use application::ports::{identity::IdentityProvider, time::Clock};
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
    use application::use_cases::create_company::CreateCompany;
    use application::use_cases::create_note::CreateNote;
    use application::use_cases::create_opportunity::CreateOpportunity;
    use application::use_cases::create_person::CreatePerson;
    use application::use_cases::create_task::CreateTask;
    use application::use_cases::create_workflow::CreateWorkflow;
    use application::use_cases::create_workspace::CreateWorkspace;
    use application::use_cases::manage_company::ManageCompany;
    use application::use_cases::manage_note::ManageNote;
    use application::use_cases::manage_opportunity::ManageOpportunity;
    use application::use_cases::manage_person::ManagePerson;
    use application::use_cases::manage_task::ManageTask;
    use application::use_cases::manage_workflow::ManageWorkflow;
    use application::use_cases::register_user::RegisterUser;
    // ... imports ...

    // 4. Initialize Use Cases
    let record_use_case = Arc::new(RecordBoardCard {
        opportunity_repo: repo.clone(),
    });
    let register_user_use_case = Arc::new(RegisterUser {
        user_repo: repo.clone(),
    });
    let create_workspace_use_case = Arc::new(CreateWorkspace::new(repo.clone()));
    let create_person_use_case = Arc::new(CreatePerson::new(repo.clone()));
    let manage_person_use_case = Arc::new(ManagePerson::new(repo.clone()));
    let create_company_use_case = Arc::new(CreateCompany::new(repo.clone()));
    let manage_company_use_case = Arc::new(ManageCompany::new(repo.clone()));
    let create_opportunity_use_case = Arc::new(CreateOpportunity::new(repo.clone()));
    let manage_opportunity_use_case = Arc::new(ManageOpportunity::new(repo.clone()));
    let create_task_use_case = Arc::new(CreateTask::new(repo.clone()));
    let manage_task_use_case = Arc::new(ManageTask::new(repo.clone()));
    let create_note_use_case = Arc::new(CreateNote::new(repo.clone()));
    let manage_note_use_case = Arc::new(ManageNote::new(repo.clone()));
    let create_workflow_use_case = Arc::new(CreateWorkflow::new(repo.clone()));
    let manage_workflow_use_case = Arc::new(ManageWorkflow::new(repo.clone()));

    // 5. Initialize App State
    let app_state = AppState {
        record_use_case: record_use_case.clone(),
        register_user: register_user_use_case.clone(),
        create_workspace: create_workspace_use_case.clone(),
        create_person: create_person_use_case.clone(),
        manage_person: manage_person_use_case.clone(),
        person_repo: repo.clone(),
        create_company: create_company_use_case.clone(),
        manage_company: manage_company_use_case.clone(),
        company_repo: repo.clone(),
        create_opportunity: create_opportunity_use_case.clone(),
        manage_opportunity: manage_opportunity_use_case.clone(),
        opportunity_repo: repo.clone(),
        create_task: create_task_use_case.clone(),
        manage_task: manage_task_use_case.clone(),
        task_repo: repo.clone(),
        create_note: create_note_use_case.clone(),
        manage_note: manage_note_use_case.clone(),
        note_repo: repo.clone(),
        create_workflow: create_workflow_use_case.clone(),
        manage_workflow: manage_workflow_use_case.clone(),
        workflow_repo: repo.clone(),
    };

    // ... seeding ...

    // 6. Build Router
    let app = Router::new()
        .route("/", get(get_board_handler))
        .route(
            "/register",
            get(infrastructure::web::handlers::get_register_handler)
                .post(infrastructure::web::handlers::post_register_handler),
        )
        .route(
            "/workspaces",
            get(infrastructure::web::handlers::get_create_workspace_handler)
                .post(infrastructure::web::handlers::post_create_workspace_handler),
        )
        .route(
            "/people",
            get(infrastructure::web::handlers::get_people_handler)
                .post(infrastructure::web::handlers::post_create_person_handler),
        )
        .route(
            "/people/new",
            get(|| async {
                crate::infrastructure::web::fragments::layout(
                    crate::infrastructure::web::fragments::person_form(),
                )
            }),
        )
        .route(
            "/people/:id",
            axum::routing::delete(infrastructure::web::handlers::delete_person_handler),
        )
        .route(
            "/companies",
            get(infrastructure::web::handlers::get_companies_handler)
                .post(infrastructure::web::handlers::post_create_company_handler),
        )
        .route(
            "/companies/new",
            get(infrastructure::web::handlers::get_create_company_handler),
        )
        .route(
            "/companies/:id",
            axum::routing::delete(infrastructure::web::handlers::delete_company_handler),
        )
        .route(
            "/opportunities",
            get(infrastructure::web::handlers::get_opportunities_handler)
                .post(infrastructure::web::handlers::post_create_opportunity_handler),
        )
        .route(
            "/opportunities/new",
            get(infrastructure::web::handlers::get_create_opportunity_handler),
        )
        .route(
            "/opportunities/:id",
            axum::routing::delete(infrastructure::web::handlers::delete_opportunity_handler),
        )
        .route(
            "/tasks",
            get(infrastructure::web::handlers::get_tasks_handler)
                .post(infrastructure::web::handlers::post_create_task_handler),
        )
        .route(
            "/tasks/new",
            get(infrastructure::web::handlers::get_create_task_handler),
        )
        .route(
            "/tasks/:id",
            axum::routing::delete(infrastructure::web::handlers::delete_task_handler),
        )
        .route(
            "/notes",
            get(infrastructure::web::handlers::get_notes_handler)
                .post(infrastructure::web::handlers::post_create_note_handler),
        )
        .route(
            "/notes/new",
            get(infrastructure::web::handlers::get_create_note_handler),
        )
        .route(
            "/notes/:id",
            axum::routing::delete(infrastructure::web::handlers::delete_note_handler),
        )
        .route(
            "/workflows",
            get(infrastructure::web::handlers::get_workflows_handler)
                .post(infrastructure::web::handlers::post_create_workflow_handler),
        )
        .route(
            "/workflows/new",
            get(infrastructure::web::handlers::get_create_workflow_handler),
        )
        .route(
            "/workflows/:id",
            axum::routing::delete(infrastructure::web::handlers::delete_workflow_handler),
        )
        .route("/cards/:id/move", axum::routing::post(move_card_handler))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    // Prevent unused warnings for new adapters by "using" them in a print (scaffolding hack)
    println!("Services initialized: Time={:?}, Auth={:?}, Search={:?}, Webhook={:?}, Billing={:?}, Storage={:?}, EventBus={:?}, JobQueue={:?}",
             clock.now(), identity_provider.get_current_user_id().await, search_index, webhook_sender, billing_provider, storage_provider, event_bus, job_queue);

    axum::serve(listener, app).await.unwrap();
}
