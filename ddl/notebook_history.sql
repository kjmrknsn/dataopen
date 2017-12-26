create table notebook_history (
  id bigint unsigned not null auto_increment
, notebook_id bigint unsigned not null
, created_by varchar(64) not null
, title varchar(128) not null default ''
, is_completed bool not null default false
, completed_at timestamp null
, created_at timestamp not null default current_timestamp
, updated_at timestamp not null default current_timestamp on update current_timestamp
, primary key (id)
);

alter table notebook_history add index notebook_history_0001 (
  notebook_id
, is_completed
, created_by
);
