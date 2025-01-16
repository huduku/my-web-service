
# MySQL init

```sql
-- mysql password 123456

create database test;
use test;


CREATE TABLE `student` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `stu_no` varchar(50) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `name` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `age` tinyint DEFAULT NULL,
  `class_id` bigint DEFAULT NULL,
  `address` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
```

# Run

```shell
cargo run
```

# Request

## retrive_list
```shell
curl --location --request POST 'http://127.0.0.1:3000/api/students' \
--header 'Content-Type: application/json' \
--data-raw '{
    "page_no": 1,
    "page_size": 10
}'
```

## retrive_page 

```shell
curl --location --request POST 'http://127.0.0.1:3000/api/students' \
--header 'Content-Type: application/json' \
--data-raw '{
    "page_no": 1,
    "page_size": 10,
    "req" : {
        "name": ""
    }
}'
```

## retrive_detail

```shell
curl --location --request GET 'http://127.0.0.1:3000/api/students/5'
```

# create

```shell
curl --location --request POST 'http://127.0.0.1:3000/api/students/create' \
--header 'Content-Type: application/json' \
--data-raw '{
    "stu_no": "1005",
    "name": "test2",
    "age": 23,
    "class_id": 5,
    "address": "测试地址2"

}'
```

# update

```shell
curl --location --request PUT 'http://127.0.0.1:3000/api/students/5' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "李四"
}'
```

# delete

```shell
curl --location --request DELETE 'http://127.0.0.1:3000/api/students/5'
```

