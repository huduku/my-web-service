# 1. 系统层面表结构

```sql
-- 机构表
CREATE TABLE sys_org
(
    id          BIGINT PRIMARY KEY AUTO_INCREMENT,
    org_code    VARCHAR(64)  NOT NULL UNIQUE COMMENT '机构编码',
    parent_code VARCHAR(64)           DEFAULT NULL COMMENT '父级机构编码',
    org_path    VARCHAR(255) NOT NULL COMMENT '根机构到当前机构的路径，使用 - 连接',
    name        VARCHAR(255) NOT NULL COMMENT '机构名称',
    created_by  BIGINT       NOT NULL COMMENT '创建人',
    updated_by  BIGINT       NOT NULL COMMENT '更新人',
    created_at  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 系统用户表
CREATE TABLE sys_users
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    username   VARCHAR(255) NOT NULL UNIQUE COMMENT '用户名',
    password   VARCHAR(255) NOT NULL COMMENT '密码',
    org_id     BIGINT       NOT NULL COMMENT '所属机构ID',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 系统角色表
CREATE TABLE sys_roles
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    name       VARCHAR(255) NOT NULL COMMENT '角色名称',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 系统用户角色表
CREATE TABLE sys_user_roles
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id    BIGINT   NOT NULL COMMENT '系统用户ID',
    role_id    BIGINT   NOT NULL COMMENT '系统角色ID',
    created_by BIGINT   NOT NULL COMMENT '创建人',
    updated_by BIGINT   NOT NULL COMMENT '更新人',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 系统资源表
CREATE TABLE sys_resources
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    name       VARCHAR(255) NOT NULL COMMENT '资源名称',
    type       SMALLINT     NOT NULL COMMENT '资源类型: 1-菜单, 2-功能',
    parent_id  BIGINT                DEFAULT NULL COMMENT '父级资源ID',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 项目表
CREATE TABLE projects
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    name       VARCHAR(255) NOT NULL COMMENT '项目名称',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 项目用户表
CREATE TABLE project_users
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    project_id BIGINT   NOT NULL COMMENT '所属项目ID',
    user_id    BIGINT   NOT NULL COMMENT '系统用户ID',
    created_by BIGINT   NOT NULL COMMENT '创建人',
    updated_by BIGINT   NOT NULL COMMENT '更新人',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 项目角色表
CREATE TABLE project_roles
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    project_id BIGINT       NOT NULL COMMENT '所属项目ID',
    name       VARCHAR(255) NOT NULL COMMENT '角色名称',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 项目用户角色表
CREATE TABLE project_user_roles
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id    BIGINT   NOT NULL COMMENT '项目用户ID',
    role_id    BIGINT   NOT NULL COMMENT '项目角色ID',
    created_by BIGINT   NOT NULL COMMENT '创建人',
    updated_by BIGINT   NOT NULL COMMENT '更新人',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 项目资源表
CREATE TABLE project_resources
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    project_id BIGINT       NOT NULL COMMENT '所属项目ID',
    name       VARCHAR(255) NOT NULL COMMENT '资源名称',
    type       SMALLINT     NOT NULL COMMENT '资源类型: 1-菜单, 2-功能',
    parent_id  BIGINT                DEFAULT NULL COMMENT '父级资源ID',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 项目角色权限表
CREATE TABLE project_role_permissions
(
    id              BIGINT PRIMARY KEY AUTO_INCREMENT,
    project_role_id BIGINT   NOT NULL COMMENT '项目角色ID',
    resource_id     BIGINT   NOT NULL COMMENT '授权资源ID',
    created_by      BIGINT   NOT NULL COMMENT '创建人',
    updated_by      BIGINT   NOT NULL COMMENT '更新人',
    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);



```