# Rocket !
This is the web server of the application. The framework used is [Rocket](https://rocket.rs).

## Setup
### Rust !
Launch in debug mode with `cargo run` in this directory

Don't forget to set rust to `nighly` thus :
```bash
rustup override set nightly
```

You need `sqlite3` and `libsqlite3-dev` installed

You can start the server with
```bash
cargo run --release
```

### Data base
Make sure to have a working `db.sqlite` file next to this file. You can generate such a file via the
[preprocessor](../wikipedia-db/README.md) tool.

## How to use
**NB**: in the following the order of the query arguments doesn't matter. If you call for a wiki that is not 
supported or a category or page that is unknown to the server it will return an error 404.

**NB 2**: `wp` will always stand for the "Wp-code" of the wiki
### Example
```bash
curl \
  -i -X POST \
  -H 'Content-Type: application/json' \
  -d '[{"categories": [122273,259429,329178,335892,387833,599340,608325,609952,664904,665006],"weight": 1},{"categories": [82807,161473,665006],"weight": 2}]' \
   http://localhost:8000/api/eo/category?limit=2
```
returns
```json
[
  {
    "category": {
      "id": 665006,
      "page_rank": 0.000011267469128040279
    },
    "weight": 3
  },
  {
    "category": {
      "id": 82807,
      "page_rank": 0.000011267470707522963
    },
    "weight": 2
  }
]
```


or `http://localhost:8000/api/eo/page?id=40547&limit=2`

```json
[
  {
    "category": {
      "id": 713833,
      "page_rank": 0.000011263873083120192
    },
    "weight": 1
  },
  {
    "category": {
      "id": 531423,
      "page_rank": 0.000011256661896767361
    },
    "weight": 1
  }
]
```
### Via `POST`

You need to send a `POST` request to what will be `scaling-potatoes.ml/api/<wp>/category?limit=a` where:
- `wp` is the wiki code (ie. en, fr, ...)
- `a` is the number of categories wanted (one can also get them all with just `scaling-potatoes.ml/api/<wp>/category`)
- You need to send JSON, thus you have to put in the header `Content-type` à `application/json`. Then the JSON must 
  have the following form
  ```json
  [
    {
        "categories": [id1,id2,...],
        "weight": w
    },
    ...
  ]
  ```
  where `w` and `idn` are positives integers. The idea is that you can send many *groups* of categories at once (as 
  much as you want actually) and you can also give teh weight that you what to this groups

It returns a JSON of this form
```json
[
  {
    "category":{
      "id":id,
      "page_rank": pk
    },
    "weight": w
  },
  ...
]
```
where `id` and `w` are positive integers and `pk` is a floating number on 64 bits. This array is lexicographically 
sorted by decreasing `(w, pk)`.

This optional argument `limit` make it so the request return at most `limit` categories. It takes the `limit` 
biggest categories starting from the la category which `w` is maximal. Intuitively, if we sent all the weight at 1 
it starts by the closest common ancestor to the groups. If there are not enough categories under it, it will look above.

### Via `GET`
//TODO
