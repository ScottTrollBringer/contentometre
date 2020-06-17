# contentometre

Build: cargo build

Run: cargo run

Tests: cargo test

Then to see the result, you can use a client like Postman to GET http://localhost:8000, you will obtain a JSON like:

```json
{
    "message": "Hello, World!"
}
```

Ici un noob va tenter de faire un simple CRUD avec front en expressjs, back en rust, une bdd sqlite, et 2-3 bricoles comme une CI/CD qui pousse sur GCP et un peu de sécurité basique.

Objectif : écrire l'humeur du jour et noter sa journée.