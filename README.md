diesel setup
dieles migration generate create_tweets_and_likes (if not exist the migration folder)
diesel migration run

docker pull postgres:9.6-bullseye && 
docker run --rm \
--name rust_blogpost_auth_async \
--publish 127.0.0.1:5432:5432 \
-e POSTGRES_USER=user \
-e POSTGRES_PASSWORD=1234 \
-e POSTGRES_DB=tweets_api_rust \
-d postgres:9.6-bullseye