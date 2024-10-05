fn main() {
    sqlx::query_file!("src/leaderboard.sql");
}
