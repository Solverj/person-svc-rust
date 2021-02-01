drop table if exists Person;

create table Person (
    id serial primary key,
    name varchar(150) unique not null,
    phone int not null,
    address varchar(150) not null
);

insert into Person(name, phone, address) values ('Sølve', 45801153, 'Peter sinneruds veg 100, 2312 Ottestad');
insert into Person(name, phone, address) values ('Kristine', 45801153, 'Peter sinneruds veg 100, 2312 Ottestad');
insert into Person(name, phone, address) values ('Samuel', 45801153, 'Peter sinneruds veg 100, 2312 Ottestad');