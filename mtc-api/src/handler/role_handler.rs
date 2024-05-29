use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;
use tracing::warn;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::permission_model::PermissionsModel;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::state::AppState;

pub async fn role_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<RoleModel>> {
    session.permission("role::read").await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1
    };

    let pagination = PaginationModel::new(
        state.role_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(page);

    state
        .role_service
        .get_page(pagination.from, pagination.per_page)
        .await?
        .ok_page(pagination)
}

pub async fn role_get_handler(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<RoleModel> {
    session.permission("role::read").await?;

    state
        .role_service
        .find_by_slug(&slug)
        .await?
        .ok_model()
}

pub async fn role_create_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<RoleCreateModel>,
) -> Result<RoleModel> {
    session.permission("role::write").await?;

    state
        .role_service
        .create(&slug, payload)
        .await?
        .ok_model()
}

pub async fn role_update_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<RoleUpdateModel>,
) -> Result<RoleModel> {
    session.permission("role::write").await?;

    state
        .role_service
        .update(&slug, payload)
        .await?
        .ok_model()
}

pub async fn role_delete_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("role::delete").await?;

    state
        .role_service
        .delete(&slug)
        .await?
        .ok_ok()
}

pub async fn role_get_permissions(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<PermissionsModel> {
    session.permission("role::read").await?;

    state
        .permissions_service
        .find_by_role(&slug)
        .await?
        .ok_model()
}

pub async fn role_set_permissions(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
    ValidatedPayload(payload): ValidatedPayload<PermissionsModel>,
) -> Result<PermissionsModel> {
    session.permission("role::write").await?;

    let role_model = state
        .role_service
        .find_by_slug(&slug)
        .await?;

    state
        .role_service
        .permissions_drop(&role_model.id)
        .await?;

    for permission in payload.permissions {
        match state
            .permissions_service
            .find_by_slug(&permission)
            .await {
            Ok(value) => {
                state
                    .role_service
                    .permission_assign(&role_model.id, &value.id)
                    .await?
            }
            _ => warn!("can't find permission -> {permission}")
        }
    }

    state
        .permissions_service
        .find_by_role(&slug)
        .await?
        .ok_model()
}