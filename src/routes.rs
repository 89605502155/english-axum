use crate::handlers::user as handler_user;
use crate::models::error;
use crate::models::user as model_user;
use crate::state::SessionState;
use axum::{
    Router,
    routing::{delete, get, post},
};
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().unwrap().add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "English API",
        version = "0.1.0",
        description = "API для управления пользователями"
    ),
    modifiers(&SecurityAddon),
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
        .route(
            "/users",
            get(handler_user::users_list).post(handler_user::create_user),
        )
        .route("/users/{id}", delete(handler_user::delete_user))
        .merge(SwaggerUi::new("/api").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
