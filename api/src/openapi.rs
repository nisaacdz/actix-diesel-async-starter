use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(
        schemas()
    ),
    tags(
        (name = "starter", description = "Starter API Documentation")
    )
)]
pub struct ApiDoc;
