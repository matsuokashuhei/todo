mod authority;
mod graphql;
use repository::async_graphql;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use graphql::schema::{build_schema, AppSchema};
use repository::db::Database;
use tower::ServiceBuilder;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:3002")
            .finish(),
    )
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let database = Database::new().await;
    let schema = build_schema(database).await;
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route_layer(middleware::from_fn(authority::authenticate_user))
        .layer(ServiceBuilder::new().layer(Extension(schema)));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
