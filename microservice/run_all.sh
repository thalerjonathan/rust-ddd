cd ./assignments
sh ./run_instance_1.sh &
sh ./run_instance_2.sh &

cd ../availabilities
sh ./run_instance_1.sh &
sh ./run_instance_2.sh &

cd ../fixtures
sh ./run_instance_1.sh &
sh ./run_instance_2.sh &

cd ../referees
sh ./run_instance_1.sh &
sh ./run_instance_2.sh &

cd ../teams
sh ./run_instance_1.sh &
sh ./run_instance_2.sh &

cd ../venues
sh ./run_instance_1.sh &
sh ./run_instance_2.sh &
