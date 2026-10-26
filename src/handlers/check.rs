/// Проверка, что сервис жив
#[utoipa::path(
    get,
    path = "/",
    tag = "Start",
    responses(
        (status = 200, description = "Сервис работает", body = String, content_type = "text/plain", example = "Hello")
    )
)]
pub async fn root() -> &'static str {
    "Hello"
}