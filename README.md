# rcp

## Run dev

### Run MongoDB docker image locally (required for api)
`cd web-api && make mongostart`

### Run Api
`cd web-api && make dev`

### Run Frontend with Hot Reload
`cd web-frontend && yarn run start:dev`

## Api example requests

### fetch recipes:
`curl http://localhost:8080/recipe`

### create a new recipe:
`curl -X POST http://localhost:8080/recipe -d '{"recipe_name": "good book"}' -H "content-type: application/json"`

### edit a recipe:
`curl -X PUT http://localhost:8080/recipe/5f15fd5400b98edc001944c0 -d '{"name": "good book"}' -H "content-type: application/json"`

### delete a recipe
`curl -X DELETE http://localhost:8080/recipe/5f15fd3900789205001944bf`