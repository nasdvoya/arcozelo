create table doadores (
    id uuid primary key default gen_random_uuid(), 
    nome varchar (100),
    email varchar (100) unique,
    horario time,
    doador boolean default false,
    morada varchar (200),
    freguesia varchar (200),
    concelho varchar (100), 
    codigo_postal char (10),
    tel_residencial char (15),
    tel_trabalho char (15),
    telemovel char (15),
    criado_em timestamp default current_timestamp,
    observacoes text
);
