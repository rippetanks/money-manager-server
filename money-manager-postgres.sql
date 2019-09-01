--
-- PostgreSQL database dump
--

-- Dumped from database version 10.6
-- Dumped by pg_dump version 10.6

-- Started on 2019-09-01 11:07:12

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 2926 (class 1262 OID 16616)
-- Name: money-manager; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE "money-manager" WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'Italian_Italy.1252' LC_CTYPE = 'Italian_Italy.1252';


ALTER DATABASE "money-manager" OWNER TO postgres;

\connect -reuse-previous=on "dbname='money-manager'"

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 1 (class 3079 OID 12924)
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- TOC entry 2928 (class 0 OID 0)
-- Dependencies: 1
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


--
-- TOC entry 220 (class 1255 OID 16881)
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO postgres;

--
-- TOC entry 221 (class 1255 OID 16882)
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO postgres;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 197 (class 1259 OID 16619)
-- Name: AccountType; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."AccountType" (
    id integer NOT NULL,
    type character varying(32) NOT NULL
);


ALTER TABLE public."AccountType" OWNER TO postgres;

--
-- TOC entry 196 (class 1259 OID 16617)
-- Name: AccountType_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."AccountType_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."AccountType_id_seq" OWNER TO postgres;

--
-- TOC entry 2929 (class 0 OID 0)
-- Dependencies: 196
-- Name: AccountType_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."AccountType_id_seq" OWNED BY public."AccountType".id;


--
-- TOC entry 202 (class 1259 OID 16644)
-- Name: AccountUser; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."AccountUser" (
    id_account bigint NOT NULL,
    id_user bigint NOT NULL
);


ALTER TABLE public."AccountUser" OWNER TO postgres;

--
-- TOC entry 201 (class 1259 OID 16638)
-- Name: account; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.account (
    id bigint NOT NULL,
    name character varying(64) NOT NULL,
    status boolean NOT NULL,
    note character varying(255),
    current_balance double precision NOT NULL,
    initial_balance double precision NOT NULL,
    creation_date timestamp with time zone NOT NULL,
    id_account_type integer NOT NULL,
    id_currency smallint NOT NULL
);


ALTER TABLE public.account OWNER TO postgres;

--
-- TOC entry 200 (class 1259 OID 16636)
-- Name: Account_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Account_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Account_id_seq" OWNER TO postgres;

--
-- TOC entry 2930 (class 0 OID 0)
-- Dependencies: 200
-- Name: Account_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Account_id_seq" OWNED BY public.account.id;


--
-- TOC entry 218 (class 1259 OID 16864)
-- Name: auth; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth (
    id bigint NOT NULL,
    email character varying(255) NOT NULL,
    iteration smallint NOT NULL,
    salt character(64) NOT NULL,
    stored_key character(64) NOT NULL,
    last_login timestamp with time zone
);


ALTER TABLE public.auth OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 16862)
-- Name: Auth_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Auth_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Auth_id_seq" OWNER TO postgres;

--
-- TOC entry 2931 (class 0 OID 0)
-- Dependencies: 217
-- Name: Auth_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Auth_id_seq" OWNED BY public.auth.id;


--
-- TOC entry 204 (class 1259 OID 16661)
-- Name: causal; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.causal (
    id bigint NOT NULL,
    description character varying(255) NOT NULL,
    id_user bigint
);


ALTER TABLE public.causal OWNER TO postgres;

--
-- TOC entry 203 (class 1259 OID 16659)
-- Name: Causal_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Causal_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Causal_id_seq" OWNER TO postgres;

--
-- TOC entry 2932 (class 0 OID 0)
-- Dependencies: 203
-- Name: Causal_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Causal_id_seq" OWNED BY public.causal.id;


--
-- TOC entry 215 (class 1259 OID 16811)
-- Name: Currency; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Currency" (
    id smallint NOT NULL,
    currency character varying(12) NOT NULL
);


ALTER TABLE public."Currency" OWNER TO postgres;

--
-- TOC entry 214 (class 1259 OID 16809)
-- Name: Currency_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Currency_id_seq"
    AS smallint
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Currency_id_seq" OWNER TO postgres;

--
-- TOC entry 2933 (class 0 OID 0)
-- Dependencies: 214
-- Name: Currency_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Currency_id_seq" OWNED BY public."Currency".id;


--
-- TOC entry 213 (class 1259 OID 16781)
-- Name: Detail; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Detail" (
    id bigint NOT NULL,
    description character varying(32) NOT NULL,
    id_user bigint
);


ALTER TABLE public."Detail" OWNER TO postgres;

--
-- TOC entry 212 (class 1259 OID 16779)
-- Name: Detail_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Detail_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Detail_id_seq" OWNER TO postgres;

--
-- TOC entry 2934 (class 0 OID 0)
-- Dependencies: 212
-- Name: Detail_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Detail_id_seq" OWNED BY public."Detail".id;


--
-- TOC entry 211 (class 1259 OID 16767)
-- Name: Giro; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Giro" (
    id bigint NOT NULL,
    id_source_account bigint NOT NULL,
    id_destination_account bigint NOT NULL,
    data timestamp with time zone NOT NULL,
    note character varying(255),
    amount double precision NOT NULL,
    expense double precision,
    id_currency smallint NOT NULL
);


ALTER TABLE public."Giro" OWNER TO postgres;

--
-- TOC entry 210 (class 1259 OID 16765)
-- Name: Giro_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Giro_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Giro_id_seq" OWNER TO postgres;

--
-- TOC entry 2935 (class 0 OID 0)
-- Dependencies: 210
-- Name: Giro_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Giro_id_seq" OWNED BY public."Giro".id;


--
-- TOC entry 205 (class 1259 OID 16683)
-- Name: Place; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Place" (
    id bigint NOT NULL,
    place character varying(64) NOT NULL,
    address character varying(128),
    country character varying(64),
    email character varying(255),
    website character varying(128),
    phone character varying(16),
    note character varying(255),
    id_user bigint
);


ALTER TABLE public."Place" OWNER TO postgres;

--
-- TOC entry 209 (class 1259 OID 16714)
-- Name: Transaction; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Transaction" (
    id bigint NOT NULL,
    id_account bigint NOT NULL,
    id_transaction_type integer NOT NULL,
    id_place bigint,
    id_beneficiary bigint,
    note character varying(255),
    amount double precision NOT NULL,
    data timestamp with time zone NOT NULL,
    id_currency smallint NOT NULL,
    expense double precision,
    id_causal bigint NOT NULL
);


ALTER TABLE public."Transaction" OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16847)
-- Name: TransactionDetail; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."TransactionDetail" (
    id_detail bigint NOT NULL,
    id_transaction bigint NOT NULL,
    amount smallint
);


ALTER TABLE public."TransactionDetail" OWNER TO postgres;

--
-- TOC entry 207 (class 1259 OID 16706)
-- Name: TransactionType; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."TransactionType" (
    id integer NOT NULL,
    type character varying(32) NOT NULL
);


ALTER TABLE public."TransactionType" OWNER TO postgres;

--
-- TOC entry 206 (class 1259 OID 16704)
-- Name: TransactionType_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."TransactionType_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."TransactionType_id_seq" OWNER TO postgres;

--
-- TOC entry 2936 (class 0 OID 0)
-- Dependencies: 206
-- Name: TransactionType_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."TransactionType_id_seq" OWNED BY public."TransactionType".id;


--
-- TOC entry 208 (class 1259 OID 16712)
-- Name: Transaction_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Transaction_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Transaction_id_seq" OWNER TO postgres;

--
-- TOC entry 2937 (class 0 OID 0)
-- Dependencies: 208
-- Name: Transaction_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Transaction_id_seq" OWNED BY public."Transaction".id;


--
-- TOC entry 199 (class 1259 OID 16627)
-- Name: user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."user" (
    id bigint NOT NULL,
    name character varying(32) NOT NULL,
    surname character varying(32) NOT NULL,
    phone character varying(16),
    country character varying(64),
    address character varying(128),
    birthdate date,
    note character varying(255)
);


ALTER TABLE public."user" OWNER TO postgres;

--
-- TOC entry 198 (class 1259 OID 16625)
-- Name: User_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."User_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."User_id_seq" OWNER TO postgres;

--
-- TOC entry 2938 (class 0 OID 0)
-- Dependencies: 198
-- Name: User_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."User_id_seq" OWNED BY public."user".id;


--
-- TOC entry 219 (class 1259 OID 16875)
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO postgres;

--
-- TOC entry 2744 (class 2604 OID 16622)
-- Name: AccountType id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AccountType" ALTER COLUMN id SET DEFAULT nextval('public."AccountType_id_seq"'::regclass);


--
-- TOC entry 2752 (class 2604 OID 16814)
-- Name: Currency id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Currency" ALTER COLUMN id SET DEFAULT nextval('public."Currency_id_seq"'::regclass);


--
-- TOC entry 2751 (class 2604 OID 16784)
-- Name: Detail id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Detail" ALTER COLUMN id SET DEFAULT nextval('public."Detail_id_seq"'::regclass);


--
-- TOC entry 2750 (class 2604 OID 16770)
-- Name: Giro id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Giro" ALTER COLUMN id SET DEFAULT nextval('public."Giro_id_seq"'::regclass);


--
-- TOC entry 2749 (class 2604 OID 16717)
-- Name: Transaction id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction" ALTER COLUMN id SET DEFAULT nextval('public."Transaction_id_seq"'::regclass);


--
-- TOC entry 2748 (class 2604 OID 16709)
-- Name: TransactionType id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."TransactionType" ALTER COLUMN id SET DEFAULT nextval('public."TransactionType_id_seq"'::regclass);


--
-- TOC entry 2746 (class 2604 OID 16641)
-- Name: account id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account ALTER COLUMN id SET DEFAULT nextval('public."Account_id_seq"'::regclass);


--
-- TOC entry 2747 (class 2604 OID 16664)
-- Name: causal id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.causal ALTER COLUMN id SET DEFAULT nextval('public."Causal_id_seq"'::regclass);


--
-- TOC entry 2745 (class 2604 OID 16630)
-- Name: user id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public."User_id_seq"'::regclass);


--
-- TOC entry 2755 (class 2606 OID 16624)
-- Name: AccountType AccountType_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AccountType"
    ADD CONSTRAINT "AccountType_pkey" PRIMARY KEY (id);


--
-- TOC entry 2761 (class 2606 OID 16648)
-- Name: AccountUser AccountUser_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AccountUser"
    ADD CONSTRAINT "AccountUser_pkey" PRIMARY KEY (id_account, id_user);


--
-- TOC entry 2759 (class 2606 OID 16643)
-- Name: account Account_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT "Account_pkey" PRIMARY KEY (id);


--
-- TOC entry 2779 (class 2606 OID 16869)
-- Name: auth Auth_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth
    ADD CONSTRAINT "Auth_pkey" PRIMARY KEY (id);


--
-- TOC entry 2763 (class 2606 OID 16666)
-- Name: causal Causal_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.causal
    ADD CONSTRAINT "Causal_pkey" PRIMARY KEY (id);


--
-- TOC entry 2775 (class 2606 OID 16816)
-- Name: Currency Currency_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Currency"
    ADD CONSTRAINT "Currency_pkey" PRIMARY KEY (id);


--
-- TOC entry 2773 (class 2606 OID 16786)
-- Name: Detail Detail_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Detail"
    ADD CONSTRAINT "Detail_pkey" PRIMARY KEY (id);


--
-- TOC entry 2771 (class 2606 OID 16772)
-- Name: Giro Giro_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Giro"
    ADD CONSTRAINT "Giro_pkey" PRIMARY KEY (id);


--
-- TOC entry 2765 (class 2606 OID 16690)
-- Name: Place Place_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Place"
    ADD CONSTRAINT "Place_pkey" PRIMARY KEY (id);


--
-- TOC entry 2777 (class 2606 OID 16851)
-- Name: TransactionDetail TransactionDetail_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."TransactionDetail"
    ADD CONSTRAINT "TransactionDetail_pkey" PRIMARY KEY (id_detail, id_transaction);


--
-- TOC entry 2767 (class 2606 OID 16711)
-- Name: TransactionType TransactionType_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."TransactionType"
    ADD CONSTRAINT "TransactionType_pkey" PRIMARY KEY (id);


--
-- TOC entry 2769 (class 2606 OID 16719)
-- Name: Transaction Transaction_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction"
    ADD CONSTRAINT "Transaction_pkey" PRIMARY KEY (id);


--
-- TOC entry 2757 (class 2606 OID 16635)
-- Name: user User_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT "User_pkey" PRIMARY KEY (id);


--
-- TOC entry 2781 (class 2606 OID 16880)
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- TOC entry 2784 (class 2606 OID 16654)
-- Name: AccountUser account__user_account_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AccountUser"
    ADD CONSTRAINT account__user_account_fk FOREIGN KEY (id_account) REFERENCES public.account(id);


--
-- TOC entry 2785 (class 2606 OID 16649)
-- Name: AccountUser account__user_user_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AccountUser"
    ADD CONSTRAINT account__user_user_fk FOREIGN KEY (id_user) REFERENCES public."user"(id);


--
-- TOC entry 2782 (class 2606 OID 16750)
-- Name: account account_account_type_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_account_type_fk FOREIGN KEY (id_account_type) REFERENCES public."AccountType"(id);


--
-- TOC entry 2783 (class 2606 OID 16817)
-- Name: account account_currency_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_currency_fk FOREIGN KEY (id_currency) REFERENCES public."Currency"(id);


--
-- TOC entry 2799 (class 2606 OID 16883)
-- Name: auth auth_user_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth
    ADD CONSTRAINT auth_user_fk FOREIGN KEY (id) REFERENCES public."user"(id);


--
-- TOC entry 2786 (class 2606 OID 16755)
-- Name: causal causal_user_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.causal
    ADD CONSTRAINT causal_user_fk FOREIGN KEY (id_user) REFERENCES public."user"(id);


--
-- TOC entry 2796 (class 2606 OID 16787)
-- Name: Detail detail_user_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Detail"
    ADD CONSTRAINT detail_user_fk FOREIGN KEY (id_user) REFERENCES public."user"(id);


--
-- TOC entry 2794 (class 2606 OID 16827)
-- Name: Giro giro_account_1_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Giro"
    ADD CONSTRAINT giro_account_1_fk FOREIGN KEY (id_source_account) REFERENCES public.account(id);


--
-- TOC entry 2795 (class 2606 OID 16832)
-- Name: Giro giro_account_2_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Giro"
    ADD CONSTRAINT giro_account_2_fk FOREIGN KEY (id_destination_account) REFERENCES public.account(id);


--
-- TOC entry 2793 (class 2606 OID 16822)
-- Name: Giro giro_currency_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Giro"
    ADD CONSTRAINT giro_currency_fk FOREIGN KEY (id_currency) REFERENCES public."Currency"(id);


--
-- TOC entry 2787 (class 2606 OID 16760)
-- Name: Place place_user_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Place"
    ADD CONSTRAINT place_user_fk FOREIGN KEY (id_user) REFERENCES public."user"(id);


--
-- TOC entry 2790 (class 2606 OID 16730)
-- Name: Transaction transaction_account_1_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction"
    ADD CONSTRAINT transaction_account_1_fk FOREIGN KEY (id_account) REFERENCES public.account(id);


--
-- TOC entry 2792 (class 2606 OID 16842)
-- Name: Transaction transaction_account_2_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction"
    ADD CONSTRAINT transaction_account_2_fk FOREIGN KEY (id_beneficiary) REFERENCES public.account(id);


--
-- TOC entry 2791 (class 2606 OID 16837)
-- Name: Transaction transaction_currency_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction"
    ADD CONSTRAINT transaction_currency_fk FOREIGN KEY (id_currency) REFERENCES public."Currency"(id);


--
-- TOC entry 2797 (class 2606 OID 16852)
-- Name: TransactionDetail transaction_detail_detail_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."TransactionDetail"
    ADD CONSTRAINT transaction_detail_detail_fk FOREIGN KEY (id_detail) REFERENCES public."Detail"(id);


--
-- TOC entry 2798 (class 2606 OID 16857)
-- Name: TransactionDetail transaction_detail_transaction_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."TransactionDetail"
    ADD CONSTRAINT transaction_detail_transaction_fk FOREIGN KEY (id_transaction) REFERENCES public."Transaction"(id);


--
-- TOC entry 2789 (class 2606 OID 16740)
-- Name: Transaction transaction_place_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction"
    ADD CONSTRAINT transaction_place_fk FOREIGN KEY (id_place) REFERENCES public."Place"(id);


--
-- TOC entry 2788 (class 2606 OID 16735)
-- Name: Transaction transaction_type_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Transaction"
    ADD CONSTRAINT transaction_type_fk FOREIGN KEY (id_transaction_type) REFERENCES public."TransactionType"(id);


-- Completed on 2019-09-01 11:07:12

--
-- PostgreSQL database dump complete
--

