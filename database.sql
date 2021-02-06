drop table if exists person;

create table person (
    id serial primary key,
    name varchar(150) unique not null,
    phone int not null,
    address varchar(150) not null
);

insert into person(name, phone, address)
values ('Sølve', 45801153, 'Peter sinneruds veg 100, 2312 Ottestad'),
       ('Kristine', 45801153, 'Peter sinneruds veg 100, 2312 Ottestad'),
       ('Samuel', 45801153, 'Peter sinneruds veg 100, 2312 Ottestad');