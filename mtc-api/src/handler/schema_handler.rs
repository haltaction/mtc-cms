use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::model::schema_model::{SchemaCreateModel, SchemaFieldsModel, SchemaModel, SchemaUpdateModel};
use crate::repository::RepositoryPaginate;
use crate::repository::schema_repository::SchemaRepositoryTrait;
use crate::state::AppState;

pub async fn schema_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<SchemaModel>> {
    session.permission("schema::read").await?;
    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1
    };

    let pagination = PaginationModel::new(
        state.schema_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(page);

    state
        .schema_service
        .get_page(pagination.from, pagination.per_page)
        .await?
        .ok_page(pagination)
}

pub async fn schema_get_handler(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<SchemaModel> {
    session.permission("schema::read").await?;

    state
        .schema_service
        .find_by_slug(&slug)
        .await?
        .ok_model()
}

pub async fn schema_create_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SchemaCreateModel>,
) -> Result<SchemaModel> {
    session.permission("schema::write").await?;

    state
        .schema_service
        .create(&slug, payload)
        .await?
        .ok_model()
}

pub async fn schema_delete_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("schema::delete").await?;

    state
        .schema_service
        .delete(&slug)
        .await?
        .ok_ok()
}

pub async fn schema_update_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SchemaUpdateModel>,
) -> Result<SchemaModel> {
    session.permission("schema::write").await?;

    state
        .schema_service
        .update(&slug, payload)
        .await?
        .ok_model()
}

pub async fn schema_update_fields_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SchemaFieldsModel>,
) -> Result<SchemaFieldsModel> {
    session.permission("schema::write").await?;

    let schema_model = state
        .schema_service
        .update_fields(&slug, payload)
        .await?;

    Ok(ApiResponse::Data(SchemaFieldsModel { fields: schema_model.fields }))
}

pub async fn schema_get_fields_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<SchemaFieldsModel> {
    session.permission("schema::read").await?;

    state
        .schema_service
        .get_fields(&slug)
        .await?
        .ok_model()
}