create table entry (
  id integer primary key not null,
  task_id integer not null,
  start_time datetime not null,
  end_time datetime,
  note text,
  foreign key(task_id) references task(id) on delete cascade
);