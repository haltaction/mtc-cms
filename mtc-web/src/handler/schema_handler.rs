use mtc_model::list_model::RecordListModel;
use mtc_model::schema_model::{SchemaCreateModel, SchemaModel, SchemaUpdateModel};

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse, HandlerResponse};
use crate::model::response_model::ApiResponse;

pub trait SchemaHandler {
    async fn get_schema(&self, slug: &str) -> Result<SchemaModel, ApiError>;
    async fn get_schema_list(&self, page: usize)
        -> Result<ApiResponse<Vec<SchemaModel>>, ApiError>;
    async fn delete_schema(&self, slug: &str) -> Result<(), ApiError>;
    async fn create_schema(
        &self,
        slug: &str,
        schema: &SchemaCreateModel,
    ) -> Result<SchemaModel, ApiError>;
    async fn update_schema(
        &self,
        slug: &str,
        schema: &SchemaUpdateModel,
    ) -> Result<SchemaModel, ApiError>;
    async fn get_all_collections(&self) -> Result<RecordListModel, ApiError>;
}

impl SchemaHandler for ApiHandler {
    async fn get_schema(&self, slug: &str) -> Result<SchemaModel, ApiError> {
        self.api_client
            .get([&self.api_url, "schema", slug].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_schema_list(
        &self,
        page: usize,
    ) -> Result<ApiResponse<Vec<SchemaModel>>, ApiError> {
        self.api_client
            .get([&self.api_url, "schema", "list", &page.to_string()].join("/"))
            .send()
            .await
            .consume_page()
            .await
    }

    async fn delete_schema(&self, slug: &str) -> Result<(), ApiError> {
        self.api_client
            .delete([&self.api_url, "schema", slug].join("/"))
            .send()
            .await
            .consume()
            .await
    }

    async fn create_schema(
        &self,
        slug: &str,
        schema: &SchemaCreateModel,
    ) -> Result<SchemaModel, ApiError> {
        self.api_client
            .post([&self.api_url, "schema", slug].join("/"))
            .json(schema)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn update_schema(
        &self,
        slug: &str,
        schema: &SchemaUpdateModel,
    ) -> Result<SchemaModel, ApiError> {
        self.api_client
            .patch([&self.api_url, "schema", slug].join("/"))
            .json(schema)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_all_collections(&self) -> Result<RecordListModel, ApiError> {
        self.api_client
            .get([&self.api_url, "schema", "collections"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
}
