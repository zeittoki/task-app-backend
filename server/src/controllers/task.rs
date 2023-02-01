use crate::errors::CustomError;
use crate::models::task::{NewTask, Task, UpdateTask};

use axum::extract::{Extension, Path};
use axum::http::StatusCode;

use axum::Json;
use sqlx::PgPool;

pub async fn get_all_tasks(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<Task>>), CustomError> {
    let sql = r#"
    SELECT * FROM task
    "#;

    let tasks = sqlx::query_as::<_, Task>(&sql)
        .fetch_all(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn get_task(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Task>), CustomError> {
    let sql = r#"
    SELECT * FROM task 
    WHERE id = $1
    "#;

    let task = sqlx::query_as::<_, Task>(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    Ok((StatusCode::FOUND, Json(task)))
}

pub async fn create_task(
    Extension(pool): Extension<PgPool>,
    Json(task): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), CustomError> {
    let sql = r#"
    INSERT INTO task (name) VALUES ($1) 
    RETURNING *
    "#;

    let new_task = sqlx::query_as::<_, Task>(&sql)
        .bind(&task.name)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(new_task)))
}

pub async fn update_task(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(task): Json<UpdateTask>,
) -> Result<(StatusCode, Json<Task>), CustomError> {
    let sql = r#"
    SELECT * FROM task 
    WHERE id = $1
    "#;

    let old_task = sqlx::query_as::<_, Task>(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::BadRequest)?;

    let name = if let Some(new_name) = task.name {
        new_name
    } else {
        old_task.name
    };
    let completed = if let Some(new_completed) = task.completed {
        new_completed
    } else {
        old_task.completed
    };

    let sql = r#"
    UPDATE task SET name = $1, completed = $2 
    WHERE id = $3 
    RETURNING *
    "#;

    let new_task = sqlx::query_as::<_, Task>(&sql)
        .bind(name)
        .bind(completed)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::OK, Json(new_task)))
}

pub async fn delete_task(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Task>), CustomError> {
    let sql = r#"
    DELETE FROM task 
    WHERE id = $1 
    RETURNING *
    "#;

    let delete_task = sqlx::query_as::<_, Task>(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::OK, Json(delete_task)))
}
