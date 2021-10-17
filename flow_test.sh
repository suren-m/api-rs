# signup_resp=`curl http://localhost:8000/signup -s \
# -d '{ "email": "joeadams@example.com", "password": "12345", "firstname" : "joe", "lastname" : "adams" }' \
# -H 'Content-Type: application/json'`

# echo $signup_resp

login_resp=`curl http://localhost:8000/login -s \
-d '{"email": "user@userland.com", "password": "1234"}' \
-H 'Content-Type: application/json'`

echo "Login Response \n"
echo $login_resp
echo "\n---------------\n"

jwt_token=`jq -n -r $login_resp.token`

user_resp=`curl http://localhost:8000/user -s -H "x-authentication-token: ${jwt_token}"`

echo "User Response \n"
echo $user_resp