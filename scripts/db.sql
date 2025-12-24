create table repository
(
    id              bigint unsigned primary key auto_increment,
    created_at      bigint          not null,
    modified_at     bigint          not null,
    deleted         bigint unsigned not null default 0,

    repository_name varchar(50)     not null,
    version         bigint unsigned not null

);

create table item
(
    id            bigint unsigned primary key auto_increment,
    created_at    bigint          not null,
    modified_at   bigint          not null,
    deleted       bigint unsigned not null default 0,

    repository_id bigint unsigned not null
);

create table item_version
(
    id                bigint unsigned primary key auto_increment,
    created_at        bigint          not null,
    modified_at       bigint          not null,
    deleted           bigint unsigned not null default 0,

    repository_id     bigint unsigned not null,
    item_id           bigint unsigned not null,
    parent_id         bigint unsigned null,

    latest_version    tinyint         not null,
    meta_version      bigint unsigned not null,
    content_version   bigint unsigned not null,

    content_type      tinyint         not null,
    file_name         varchar(255)    not null,
    file_id           bigint unsigned null,

    capabilities      bigint unsigned not null,
    modification_date bigint          null,
    creation_date     bigint          null,
    last_use_date     bigint          null
);

create table file_content
(
    id          integer primary key AUTOINCREMENT,
    created_at  bigint unsigned not null,
    modified_at bigint unsigned not null,
    deleted     integer         not null default 0,
    file_id     varchar(256)    not null,
    file_size   integer         not null
);