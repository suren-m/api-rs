signup_resp=`curl http://localhost:8000/signup -s \
-d '{ "email": "user@userland.com", "password": "12345", "firstname" : "joe", "lastname" : "adams" }' \
-H 'Content-Type: application/json'`

echo $signup_resp

echo "\n---------------\n"

login_resp=`curl http://localhost:8000/login -s \
-d '{"email": "user@userland.com", "password": "12345"}' \
-H 'Content-Type: application/json'`

echo "Login Response \n"
echo $login_resp
echo "\n---------------\n"

jwt_token=`jq -n -r $login_resp.token`

echo "Users Response \n"

users_resp=`curl http://localhost:8000/users -s -H "x-authentication-token: ${jwt_token}"`

echo $users_resp

echo "\n---------------\n"

echo "Update Response \n"

update_resp=`curl http://localhost:8000/users -s \
-X PUT \
-d '{ "firstname" : "jamie", "lastname" : "adams" }' \
-H 'Content-Type: application/json' -H "x-authentication-token: ${jwt_token}"`

echo $update_resp

echo "\n---------------\n"

echo "Users Response \n"

users_resp=`curl http://localhost:8000/users -s -H "x-authentication-token: ${jwt_token}"`

echo $users_resp
