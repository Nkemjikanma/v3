use crate::common::api_response::{APIResponse, AppResponse};
use crate::services::songs::SongService;
use crate::types::app::AppState;
use crate::types::songs::{Song, SongFormData, SongQueryInfo, UpdateSongFormData};

use std::sync::Arc;
use uuid::Uuid;

use actix_web::web;
pub async fn add_song(
    song_data: web::Json<SongFormData>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    SongService::add_song(&song_data.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(
        "New song successfully added".to_string(),
    ))
}

pub async fn get_all_songs(
    query: web::Query<SongQueryInfo>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<Vec<Song>> {
    let songs = SongService::get_all_songs(query.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(songs))
}

pub async fn get_song_by_id(
    path: web::Path<Uuid>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<Song> {
    let song = SongService::get_song_by_id(path.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(song))
}

pub async fn update_song(
    path: web::Path<Uuid>,
    song_data: web::Json<UpdateSongFormData>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    SongService::update_song(
        path.into_inner(),
        &song_data.into_inner(),
        &app_state.connection,
    )
    .await?;

    Ok(APIResponse::success(
        "Song was successfully updated".to_string(),
    ))
}

pub async fn delete_song(
    path: web::Path<Uuid>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    SongService::delete_song(path.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(
        "Song was deleted successfully".to_string(),
    ))
}
