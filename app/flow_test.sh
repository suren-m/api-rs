echo "Signup Request \n"

signup_resp=`curl http://localhost:8000/signup -s \
-d '{ "email": "user@userland.com", "password": "12345", "firstname" : "joe", "lastname" : "adams" }' \
-H 'Content-Type: application/json'`

echo $signup_resp

echo "\n---------------\n"

echo "Login Request with email and password \n"

login_resp=`curl http://localhost:8000/login -s \
-d '{"email": "user@userland.com", "password": "12345"}' \
-H 'Content-Type: application/json'`

echo $login_resp
echo "\n---------------\n"

jwt_token=`jq -n -r $login_resp.token`

echo "Users Request using JWT token \n"

users_resp=`curl http://localhost:8000/users -s -H "x-authentication-token: ${jwt_token}"`

echo $users_resp

echo "\n---------------\n"

echo "Current User Update (firstname and lastname) with PUT \n"

update_resp=`curl http://localhost:8000/users -s \
-X PUT \
-d '{ "firstname" : "jamie", "lastname" : "adams" }' \
-H 'Content-Type: application/json' -H "x-authentication-token: ${jwt_token}"`

echo $update_resp

echo "\n---------------\n"

echo "Users Request after update\n"

users_resp=`curl http://localhost:8000/users -s -H "x-authentication-token: ${jwt_token}"`

echo $users_resp
