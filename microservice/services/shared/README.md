# NOTES

We are using the .env for assignments to keep sqlx happy.

We are using a postgres trigger to notify the microservice when a domain event is inserted. See https://medium.com/launchpad-lab/postgres-triggers-with-listen-notify-565b44ccd782 for the implementation using triggers/notifications.