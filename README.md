# Deadlock Live Events API

## Self Host

You don't need to clone this repo, here is how to use it.

1. Create a new folder and copy the `.env.example` file into it, rename it to `.env`.
2. In the `.env` file, add your Deadlock API key (optional).
3. Copy the `docker-compose.yaml` file into the folder. (Adjust the port if needed)
4. Run `docker-compose up -d` to start the API.
5. Live events can be accessed at http://localhost:3000/v1/matches/{match_id}/live/demo/events
