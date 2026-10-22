use crate::state::SessionState;
use utoipa_swagger_ui::SwaggerUi;
use axum::{Router, routing::{get, post, delete}};
use crate::handlers::user as handler_user;
use crate::models::user as model_user;
use utoipa::OpenApi;
use crate::models::error;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "English API",
        version = "0.1.0",
        description = "API для управления пользователями"
    ),
    paths(handler_user::root, handler_user::users_list, handler_user::create_user, handler_user::delete_user),
    components(schemas(model_user::User, model_user::UserPayLoad, error::ErrorResponse)),
    tags(
        (name = "Start", description = "Проверка доступности сервиса"),
        (name = "Users", description = "CRUD операции над пользователями")
    )
)]
struct ApiDoc;


pub fn app(state: SessionState) -> Router<()> {
    Router::new()
        .route("/", get(handler_user::root))
        .route("/users", get(handler_user::users_list).post(handler_user::create_user))
        .route("/users/{id}", delete(handler_user::delete_user))
        .merge(
            SwaggerUi::new("/api")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .with_state(state)
}