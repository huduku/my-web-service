# 1. 系统层面表结构

```sql
CREATE TABLE organizations
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '机构ID',
    parent_id  BIGINT    DEFAULT NULL COMMENT '父级机构ID',
    name       VARCHAR(255) NOT NULL COMMENT '机构名称',
    org_path   TEXT         NOT NULL COMMENT '机构路径 (存储根到当前机构的路径)',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='机构表';

CREATE TABLE sys_users
(
    id              BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '用户ID',
    organization_id BIGINT       NOT NULL COMMENT '所属机构ID',
    username        VARCHAR(255) NOT NULL UNIQUE COMMENT '用户名',
    password_hash   VARCHAR(255) NOT NULL COMMENT '密码哈希',
    is_super_admin  BOOLEAN      NOT NULL DEFAULT FALSE COMMENT '是否为超级管理员',
    created_by      BIGINT       NOT NULL COMMENT '创建人',
    updated_by      BIGINT       NOT NULL COMMENT '更新人',
    created_at      TIMESTAMP             DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at      TIMESTAMP             DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='系统用户表';

CREATE TABLE system_roles
(
    id          BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '系统角色ID',
    name        VARCHAR(255) NOT NULL UNIQUE COMMENT '角色名称',
    description TEXT      DEFAULT '' COMMENT '角色描述',
    created_by  BIGINT       NOT NULL COMMENT '创建人',
    updated_by  BIGINT       NOT NULL COMMENT '更新人',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='系统角色表';

CREATE TABLE sys_user_roles
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '主键ID',
    user_id    BIGINT NOT NULL COMMENT '用户ID',
    role_id    BIGINT NOT NULL COMMENT '系统角色ID',
    created_by BIGINT NOT NULL COMMENT '创建人',
    updated_by BIGINT NOT NULL COMMENT '更新人',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (user_id, role_id)
) COMMENT ='系统用户-系统角色关联表';

CREATE TABLE system_resources
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '系统资源ID',
    parent_id  BIGINT    DEFAULT NULL COMMENT '父级资源ID',
    name       VARCHAR(255) NOT NULL COMMENT '资源名称',
    path       VARCHAR(255) NOT NULL COMMENT '资源路径',
    type       SMALLINT     NOT NULL COMMENT '资源类型 (1: 菜单, 2: 功能)',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='系统资源表';

CREATE TABLE system_role_permissions
(
    id          BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '主键ID',
    role_id     BIGINT NOT NULL COMMENT '系统角色ID',
    resource_id BIGINT NOT NULL COMMENT '系统资源ID',
    created_by  BIGINT NOT NULL COMMENT '创建人',
    updated_by  BIGINT NOT NULL COMMENT '更新人',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (role_id, resource_id)
) COMMENT ='系统角色-资源权限表';

CREATE TABLE audit_logs
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '日志ID',
    user_id    BIGINT NOT NULL COMMENT '用户ID',
    action     TEXT   NOT NULL COMMENT '操作内容',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'
) COMMENT ='审计日志表';

CREATE TABLE projects
(
    id          BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '项目ID',
    name        VARCHAR(255) NOT NULL UNIQUE COMMENT '项目名称',
    description TEXT      DEFAULT '' COMMENT '项目描述',
    created_by  BIGINT       NOT NULL COMMENT '创建人',
    updated_by  BIGINT       NOT NULL COMMENT '更新人',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='项目表';

CREATE TABLE project_users
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '主键ID',
    project_id BIGINT NOT NULL COMMENT '项目ID',
    user_id    BIGINT NOT NULL COMMENT '用户ID',
    created_by BIGINT NOT NULL COMMENT '创建人',
    updated_by BIGINT NOT NULL COMMENT '更新人',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (project_id, user_id)
) COMMENT ='项目-用户关联表';

CREATE TABLE project_roles
(
    id          BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '项目角色ID',
    project_id  BIGINT       NOT NULL COMMENT '所属项目ID',
    name        VARCHAR(255) NOT NULL COMMENT '角色名称',
    description TEXT      DEFAULT '' COMMENT '角色描述',
    created_by  BIGINT       NOT NULL COMMENT '创建人',
    updated_by  BIGINT       NOT NULL COMMENT '更新人',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='项目角色表';

CREATE TABLE project_user_roles
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '主键ID',
    user_id    BIGINT NOT NULL COMMENT '用户ID',
    project_id BIGINT NOT NULL COMMENT '项目ID',
    role_id    BIGINT NOT NULL COMMENT '项目角色ID',
    created_by BIGINT NOT NULL COMMENT '创建人',
    updated_by BIGINT NOT NULL COMMENT '更新人',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (user_id, project_id, role_id)
) COMMENT ='项目用户-角色关联表';

CREATE TABLE project_resources
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '项目资源ID',
    project_id BIGINT       NOT NULL COMMENT '所属项目ID',
    parent_id  BIGINT    DEFAULT NULL COMMENT '父级资源ID',
    name       VARCHAR(255) NOT NULL COMMENT '资源名称',
    path       VARCHAR(255) NOT NULL COMMENT '资源路径',
    type       SMALLINT     NOT NULL COMMENT '资源类型 (1: 菜单, 2: 功能)',
    created_by BIGINT       NOT NULL COMMENT '创建人',
    updated_by BIGINT       NOT NULL COMMENT '更新人',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) COMMENT ='项目资源表';

CREATE TABLE project_role_permissions
(
    id          BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '主键ID',
    role_id     BIGINT NOT NULL COMMENT '项目角色ID',
    resource_id BIGINT NOT NULL COMMENT '项目资源ID',
    created_by  BIGINT NOT NULL COMMENT '创建人',
    updated_by  BIGINT NOT NULL COMMENT '更新人',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY (role_id, resource_id)
) COMMENT ='项目角色-资源权限表';




```