table! {
    account (id) {
        id -> Int8,
        name -> Varchar,
        status -> Bool,
        note -> Nullable<Varchar>,
        current_balance -> Float8,
        initial_balance -> Float8,
        creation_date -> Timestamptz,
        id_account_type -> Int4,
        id_currency -> Int2,
    }
}

table! {
    account_type (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    account_user (id_account, id_user) {
        id_account -> Int8,
        id_user -> Int8,
    }
}

table! {
    auth (id) {
        id -> Int8,
        email -> Varchar,
        iteration -> Int2,
        salt -> Bpchar,
        stored_key -> Bpchar,
        last_login -> Nullable<Timestamptz>,
    }
}

table! {
    causal (id) {
        id -> Int8,
        description -> Varchar,
        id_user -> Nullable<Int8>,
    }
}

table! {
    currency (id) {
        id -> Int2,
        description -> Varchar,
    }
}

table! {
    Detail (id) {
        id -> Int8,
        description -> Varchar,
        id_user -> Nullable<Int8>,
    }
}

table! {
    Giro (id) {
        id -> Int8,
        id_source_account -> Int8,
        id_destination_account -> Int8,
        data -> Timestamptz,
        note -> Nullable<Varchar>,
        amount -> Float8,
        expense -> Nullable<Float8>,
        id_currency -> Int2,
    }
}

table! {
    Place (id) {
        id -> Int8,
        place -> Varchar,
        address -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        note -> Nullable<Varchar>,
        id_user -> Nullable<Int8>,
    }
}

table! {
    Transaction (id) {
        id -> Int8,
        id_account -> Int8,
        id_transaction_type -> Int4,
        id_place -> Nullable<Int8>,
        id_beneficiary -> Nullable<Int8>,
        note -> Nullable<Varchar>,
        amount -> Float8,
        data -> Timestamptz,
        id_currency -> Int2,
        expense -> Nullable<Float8>,
        id_causal -> Int8,
    }
}

table! {
    TransactionDetail (id_detail, id_transaction) {
        id_detail -> Int8,
        id_transaction -> Int8,
        amount -> Nullable<Int2>,
    }
}

table! {
    TransactionType (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int8,
        name -> Varchar,
        surname -> Varchar,
        phone -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        birthdate -> Nullable<Date>,
        note -> Nullable<Varchar>,
    }
}

joinable!(Detail -> user (id_user));
joinable!(Giro -> currency (id_currency));
joinable!(Place -> user (id_user));
joinable!(Transaction -> Place (id_place));
joinable!(Transaction -> TransactionType (id_transaction_type));
joinable!(Transaction -> currency (id_currency));
joinable!(TransactionDetail -> Detail (id_detail));
joinable!(TransactionDetail -> Transaction (id_transaction));
joinable!(account -> account_type (id_account_type));
joinable!(account -> currency (id_currency));
joinable!(account_user -> account (id_account));
joinable!(account_user -> user (id_user));
joinable!(auth -> user (id));
joinable!(causal -> user (id_user));

allow_tables_to_appear_in_same_query!(
    account,
    account_type,
    account_user,
    auth,
    causal,
    currency,
    Detail,
    Giro,
    Place,
    Transaction,
    TransactionDetail,
    TransactionType,
    user,
);