create table paragraph (
  id bigint unsigned not null auto_increment
, notebook_id bigint unsigned not null
, notebook_history_id bigint unsigned not null
, code text not null
, result text not null
, created_at timestamp not null default current_timestamp
, updated_at timestamp not null default current_timestamp on update current_timestamp
, primary key (id)
);

alter table paragraph add index paragraph_0001 (
  notebook_id
, notebook_history_id
);
