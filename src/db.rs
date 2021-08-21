use sqlx::MySqlPool;
use sqlx::Result;
use uuid::Uuid;

pub async fn get_account_id(pool: &MySqlPool, public_key: &[u8]) -> Result<Option<Uuid>> {
    let data = sqlx::query!(
        "SELECT account_id FROM public_key_mapping WHERE public_key = ?",
        public_key
    )
    .fetch_optional(pool)
    .await?;

    Ok(data.map(|f| Uuid::from_slice(&f.account_id).expect("error parsing uuid")))
}

pub async fn add_account_mapping(
    pool: &MySqlPool,
    public_key: &[u8],
    account_id: &Uuid,
    email: &str,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO public_key_mapping (email, public_key, account_id) VALUES (?, ?, ?)",
        email,
        public_key,
        &account_id.as_bytes()[..]
    )
    .execute(pool)
    .await?;
    Ok(())
}
