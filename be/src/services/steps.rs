use crate::errors::{AppError, StepErrors};
use crate::types::steps::{Steps, StepsFormData, StepsQueryInfo};
use sqlx::PgPool;
pub struct StepService;

impl StepService {
    #[tracing::instrument(name = "get steps", skip(pool))]
    pub async fn get_steps(
        time_range: StepsQueryInfo,
        pool: &PgPool,
    ) -> Result<Vec<Steps>, AppError> {
        let StepsQueryInfo { from, to } = time_range;
        let steps = sqlx::query_as!(
            Steps,
            r#"SELECT id, date, step_count, created_at, updated_at FROM daily_steps ORDER BY date"#
        )
        .fetch_all(pool)
        .await
        .map_err(|e: sqlx::Error| {
            tracing::error!("Error fetching steps for given date");

            StepErrors::StepQueryError(e.to_string())
        })?;

        let filtered: Vec<Steps> = steps
            .into_iter()
            .filter(|b| from.as_ref().is_none_or(|f| b.date >= *f))
            .filter(|b| to.as_ref().is_none_or(|t| b.date >= *t))
            .collect();

        Ok(filtered)
    }

    #[tracing::instrument(name = "set steps", skip(pool))]
    pub async fn set_steps(steps_data: &StepsFormData, pool: &PgPool) -> Result<(), AppError> {
        let StepsFormData { step_count, date } = steps_data;

        sqlx::query!(r#"INSERT INTO daily_steps (date, step_count) VALUES ($1, $2) ON CONFLICT (date) DO UPDATE SET step_count = excluded.step_count, updated_at = now()"#, date, step_count)
            .execute(pool)
            .await
            .map_err(|e: sqlx::Error| {
                tracing::error!("Couldn't set the steps count for given date");

                StepErrors::StepsSetError
            })?;

        Ok(())
    }
}
