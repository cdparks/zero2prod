use actix_web::HttpResponse;

#[tracing::instrument(name = "Health check")]
#[allow(clippy::async_yields_async)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
