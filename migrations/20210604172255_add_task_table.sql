create table task (
  id integer primary key not null,
  title text not null,
  category_id integer not null,
  foreign key(category_id) references category(id) on delete cascade
);