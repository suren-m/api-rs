# api-rs

Web API in Rust that demonstrates JWT based flows and data persistence using Postgres

### To run locally using Docker: 

```
docker-compose up --build
```

### To test all the endpoints and flows

```
chmod +x ./app/flow_test.sh
./flow_test.sh
```

![Screenshot from 2021-10-17 22-50-12](https://user-images.githubusercontent.com/3830633/137646213-346ab210-d42a-4e8d-8d18-52c0435c6983.png)

---

### Project Structure

* app directory
    * contains Rust source code, including `modules` and `api_server` binary. `lib.rs` is the root module.
    * src/bin/api_server.rs - endpoints
    * also includes the application's Dockerfile    
* postgres-docker
    * docker image for postgres along with `init.sql`
* _k8s 
    * kustomization manifests to run the solution on Kubernetes if required. (not fully tested yet)

```
api-rs on  main [!] 
❯ tree -L 3
.
├── app
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── diesel.toml
│   ├── Dockerfile
│   ├── flow_test.sh
│   ├── migrations
│   │   ├── 00000000000000_diesel_initial_setup
│   │   └── 2021-10-16-175923_create_users
│   ├── src
│   │   ├── auth.rs
│   │   ├── bin
│   │   ├── crypto.rs
│   │   ├── dto.rs
│   │   ├── error.rs
│   │   ├── lib.rs
│   │   ├── models.rs
│   │   ├── repository.rs
│   │   ├── schema.rs
│   │   └── seed.rs
├── docker-compose.yml
├── _k8s
│   ├── base
│   │   ├── api
│   │   ├── db
│   │   └── kustomization.yml
│   ├── dev
│   │   ├── kustomization.yml
│   │   ├── namespace.yml
│   │   └── pgadmin
│   └── kustomization.yml
├── postgres-docker
│   ├── Dockerfile
│   ├── init.sql
│   └── my-postgres.conf
└── README.md
```

#### Essential Rust Frameworks and Crates Used

* Diesel ORM for Data Access    
    * Diesel-cli when developing locally. (needs libpq-dev)
    * Above is not required for running the code as Migrations are also run at entry point.
* Warp for serving the API
* jsonwebtoken for JWT
* Serde for serializing and deserializing json
* Bcrypt for hashing

#### Todo

* Unit & Integration tests
* Validations on Request Body (Email validation, max length, etc.)
* Logging and improvements to error handling
* Auto docs generation and OpenAPI integration
* Connection Pooling for DB access
* CI pipeline

#### References:

* https://diesel.rs/guides/getting-started
* https://docs.diesel.rs/diesel/query_dsl/trait.QueryDsl.html
* https://blog.logrocket.com/jwt-authentication-in-rust/

