use crate::common::api_response::{APIResponse, AppResponse};
use crate::services::steps::StepService;
use crate::types::{
    app::AppState,
    steps::{Steps, StepsFormData, StepsQueryInfo},
};
use actix_web::web;

use std::sync::Arc;
pub async fn get_steps(
    query: web::Query<StepsQueryInfo>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<Vec<Steps>> {
    let steps = StepService::get_steps(query.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(steps))
}

pub async fn set_steps(
    steps_body: web::Json<StepsFormData>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    StepService::set_steps(&steps_body.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success("Step data set or updated".to_string()))
}
