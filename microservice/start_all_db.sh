cd ./referees
sh ./start_db.sh &
cd ../venues
sh ./start_db.sh &
cd ../teams
sh ./start_db.sh &
cd ../fixtures
sh ./start_db.sh &
cd ../availabilities
sh ./start_db.sh &
cd ../assignments
sh ./start_db.sh &
