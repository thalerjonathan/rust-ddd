SELECT f.fixture_id as id, f.date, f.status,
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club,
                r1.referee_id as first_referee_id, r1.name as first_referee_name, r1.club as first_referee_club,
                r2.referee_id as second_referee_id, r2.name as second_referee_name, r2.club as second_referee_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            LEFT JOIN rustddd.referees r1 ON r1.referee_id = f.first_referee_id
            LEFT JOIN rustddd.referees r2 ON r2.referee_id = f.second_referee_id
            WHERE f.fixture_id = '69e54d87-8552-469b-81b9-182a470238b4'; 