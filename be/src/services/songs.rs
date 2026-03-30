use crate::errors::{SongErrors, AppError};
use crate::types::songs::{Instrument, Song, SongFormData, SongQueryInfo, UpdateSongFormData};
use crate::common::valid_string_entry::ValidStringEntry; 
use chrono::Utc; 
use uuid::Uuid; 
pub struct SongService;

impl SongService {
    #[tracing::instrument(name = "add song", skip(pool))]
    pub async fn add_song(song: &SongFormData, pool: &sqlx::PgPool) -> Result<(), AppError> {
        tracing::info!("Adding song to scoring list");

        sqlx::query!(r#"INSERT INTO songs (title, artist, instrument, started_learning_at, notes) VALUES ($1, $2, $3, $4, $5)"#, 
            song.title.as_ref(),
            song.artist.as_ref(), 
            song.instrument.clone() as Instrument, 
            song.started_learning_at, 
            song.notes.as_deref(),
        )
            .execute(pool)
            .await
            .map_err(|e: sqlx::Error| {
                tracing::error!("Error adding song: {:?}", e); 
                SongErrors::ErrorAddingSong(song.title.as_ref().to_string())
            })?;

        Ok(())
    }
   
    #[tracing::instrument(name = "get song by id", skip(pool))]
    pub async fn get_song_by_id(song_id: Uuid, pool: &sqlx::PgPool) -> Result<Song, AppError> {
        tracing::info!("Fetching song by id"); 

        let song = sqlx::query_as!(Song, r#"SELECT id, title, artist, instrument AS "instrument: Instrument", started_learning_at, notes, created_at, updated_at FROM songs WHERE id = $1"#, song_id).fetch_one(pool).await.map_err(|e: sqlx::Error | {
            tracing::error!("Failed to fetch song with id, {}. {:?}", song_id, e); 

            match e {
                sqlx::Error::RowNotFound => SongErrors::SongNotFound(song_id.to_string()), 
                _ => SongErrors::SongQueryError("Error fetching song".to_string())
            }
        })?; 

        Ok(song)
    }

    #[tracing::instrument(name = "get all songs", skip(pool))]
    pub async fn get_all_songs(query: SongQueryInfo, pool: &sqlx::PgPool) -> Result<Vec<Song>, AppError> {
        tracing::info!("Fetching all songs"); 

        let SongQueryInfo {instrument} = query; 

        let songs = sqlx::query_as!(Song, r#"SELECT id, title, artist, instrument AS "instrument: Instrument", started_learning_at, notes, created_at, updated_at FROM songs ORDER BY started_learning_at"#).fetch_all(pool).await.map_err(|e: sqlx::Error| {
            tracing::error!("Failed to fetch all songs: {:?}", e); 

            SongErrors::ErrorFetchingSongs(e.to_string())
        })?; 

        let filtered: Vec<Song> = songs.into_iter().filter(|b| instrument.as_ref().is_none_or(|i| b.instrument == *i)).collect(); 

        Ok(filtered)

    }

    #[tracing::instrument(name = "update song", skip(pool))]
    pub async fn update_song(song_id: Uuid, update_body: &UpdateSongFormData, pool: &sqlx::PgPool) -> Result<(), AppError> {
        tracing::info!("Updating song"); 

        let old_song = sqlx::query_as!(Song, r#"SELECT id, title, artist, instrument AS "instrument: Instrument", started_learning_at, notes, created_at, updated_at FROM songs WHERE id = $1"#, song_id).fetch_one(pool).await.map_err(|e: sqlx::Error| {
            tracing::error!("Failed to fetch song for updating: {:?}", e); 

            match e {
                sqlx::Error::RowNotFound => SongErrors::SongNotFound(song_id.to_string()), 
                _ => SongErrors::SongQueryError("Error fetching song".to_string())
            }
        })?; 

        let title_str = update_body.title.clone().map(|t| t.as_ref().to_string()).unwrap_or(old_song.title); 
        let title = ValidStringEntry::parse(title_str).map_err(|e| AppError::ValidationError(e))?;

        let artist_str = update_body.artist.clone().map(|a| a.as_ref().to_string()).unwrap_or(old_song.artist); 
        let artist = ValidStringEntry::parse(artist_str).map_err(|e| AppError::ValidationError(e))?;

        let updated_song = SongFormData {
            title, 
            artist, 
            instrument: update_body.instrument.clone().unwrap_or(old_song.instrument), 
            notes: update_body.notes.clone().or(old_song.notes), 
            started_learning_at: update_body.started_learning_at.clone().unwrap_or(old_song.started_learning_at)
        };

        sqlx::query!(r#"UPDATE songs set title = $1, artist = $2, instrument = $3, notes = $4, started_learning_at = $5, updated_at = now() where id = $6"#, 
            updated_song.title.as_ref(), 
            updated_song.artist.as_ref(), 
            updated_song.instrument.clone() as Instrument,
            updated_song.notes.as_deref(),
            updated_song.started_learning_at,
            song_id)
            .execute(pool)
            .await
            .map_err(|e: sqlx::Error| {
            tracing::error!("Failed to update book: {:?}", e);
            SongErrors::ErrorUpdatingSong(updated_song.title.as_ref().to_string())
        })?;

        Ok(())

    }

    #[tracing::instrument(name = "delete song", skip(pool))]
    pub async fn delete_song(song_id: Uuid, pool: &sqlx::PgPool) -> Result<(), AppError> {
        let result = sqlx::query!(r#"DELETE FROM songs WHERE id = $1"#, song_id).execute(pool).await.map_err(|e: sqlx::Error| {
            tracing::error!(error =? e, "Error deleting songs"); 

            SongErrors::ErrorDeletingSong(e.to_string())
        })?;


        if result.rows_affected() == 0 {
            tracing::warn!(%song_id, "No songs was found to delete"); 

            return Err(SongErrors::SongNotFound("No song was deleted".to_string()))?
        }

        Ok(())
    }
}



