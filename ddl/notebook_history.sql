create table notebook_history (
    id bigint unsigned not null auto_increment
  , notebook_id bigint unsigned not null
  , created_by varchar(64)
  , created_at timestamp not null default current_timestamp
  , updated_at timestamp not null default current_timestamp on update current_timestamp
  , primary key (id)
);
