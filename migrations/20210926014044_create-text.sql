-- Add migration script here
create table texts (
  uuid varchar(48) primary key,
  value varchar(1024) not null,
  created_at varchar(1024) not null
);
