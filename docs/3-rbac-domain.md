
```rust
/// 数据传输对象 (DTO)
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemUserDTO {
    pub id: Option<i64>,
    pub username: String,
    pub email: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// 用户注册命令
#[derive(Debug, Deserialize)]
pub struct RegisterUserCommand {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

/// 领域基元 (Domain Primitive)
#[derive(Debug)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, String> {
        if value.len() < 3 {
            Err("Username must be at least 3 characters long".to_string())
        } else {
            Ok(Self(value))
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(value: String) -> Self {
        // 假设这里有哈希计算逻辑
        Self(value)
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Email(Option<String>);

impl Email {
    pub fn new(value: Option<String>) -> Self {
        // 可以添加 email 格式校验
        Self(value)
    }
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

/// 领域实体
#[derive(Debug)]
pub struct SystemUser {
    pub id: i64,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub email: Email,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl SystemUser {
    pub fn new(id: i64, username: Username, password_hash: PasswordHash, email: Email, created_at: NaiveDateTime, updated_at: NaiveDateTime) -> Self {
        Self { id, username, password_hash, email, created_at, updated_at }
    }
}

/// 数据库持久化对象 (PO)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[crud_table(table_name: "sys_users")]
pub struct SystemUserPO {
    pub id: Option<i64>,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}



```
