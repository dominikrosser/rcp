# rcp

## Run dev

### Run mongodb docker image (required for api)
`cd web-api && docker-compose up -d`

### Run api
`cd web-api && make dev`

### Run frontend with hot-reload
`cd web-frontend && yarn run start:dev`

## Api example requests

### Fetch recipes:

All recipes:
`curl http://localhost:8080/recipe`

With id:
`curl -X GET http://localhost:8080/recipe/5fad75980046a9e300522b24`

### Create a new recipe:
`curl -X POST http://localhost:8080/recipe -d '{"recipe_name": "good recipe"}' -H "content-type: application/json"`
Look at the code for additional properties of a recipe.

### Edit a recipe:
`curl -X PUT http://localhost:8080/recipe/5f15fd5400b98edc001944c0 -d '{"recipe_name": "good recipe"}' -H "content-type: application/json"`

### Delete a recipe
`curl -X DELETE http://localhost:8080/recipe/5f15fd3900789205001944bf`
