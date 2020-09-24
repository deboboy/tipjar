drop table if exists worker;
drop table if exists worker_list;

create table worker_list (
    id serial primary key,
    category varchar(150),
    create_date DATE NOT NULL DEFAULT CURRENT_DATE
);

create table worker (
    id serial primary key,
    bio varchar(140) not null,
    tip_method varchar(20),
    create_date DATE NOT NULL DEFAULT CURRENT_DATE,
    update_date DATE NOT NULL DEFAULT CURRENT_DATE,
    checked boolean not null default false,
    list_id integer not null,
    foreign key (list_id) references worker_list(id) 
);

insert into worker_list (category) values ('Mass Transit');
insert into worker (bio, tip_method, list_id) values ('driver for downtown bus routes', '@safemetro', 1);
