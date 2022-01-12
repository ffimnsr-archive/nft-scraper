--
-- PostgreSQL database dump
--

-- Dumped from database version 14.1
-- Dumped by pg_dump version 14.1

-- Started on 2022-01-12 14:26:14 UTC

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 209 (class 1259 OID 17021)
-- Name: nft_contract; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.nft_contract (
    address character varying(255) NOT NULL,
    name character varying(255),
    symbol character varying(10),
    type character varying(10)
);


ALTER TABLE public.nft_contract OWNER TO postgres;

--
-- TOC entry 213 (class 1259 OID 17064)
-- Name: nft_data_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.nft_data_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.nft_data_seq OWNER TO postgres;

--
-- TOC entry 210 (class 1259 OID 17028)
-- Name: nft_data; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.nft_data (
    id integer DEFAULT nextval('public.nft_data_seq'::regclass) NOT NULL,
    nft_contract_address character varying(255) NOT NULL,
    chain character varying(255) NOT NULL,
    metadata_url text NOT NULL,
    file_url text NOT NULL,
    cached_file_url text NOT NULL,
    mint_date timestamp with time zone NOT NULL,
    updated_date timestamp with time zone NOT NULL,
    token_id integer
);


ALTER TABLE public.nft_data OWNER TO postgres;

--
-- TOC entry 212 (class 1259 OID 17051)
-- Name: nft_file_information; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.nft_file_information (
    height integer NOT NULL,
    width integer NOT NULL,
    file_size integer NOT NULL,
    nft_data_id integer NOT NULL
);


ALTER TABLE public.nft_file_information OWNER TO postgres;

--
-- TOC entry 211 (class 1259 OID 17035)
-- Name: nft_metadata; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.nft_metadata (
    description text NOT NULL,
    background_color character varying(10) NOT NULL,
    external_url text NOT NULL,
    image text NOT NULL,
    name character varying(255) NOT NULL,
    animation_url text NOT NULL,
    nft_data_id integer NOT NULL,
    attributes text NOT NULL
);


ALTER TABLE public.nft_metadata OWNER TO postgres;

--
-- TOC entry 214 (class 1259 OID 17102)
-- Name: v_nfts; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.v_nfts AS
 SELECT c.address AS contract_address,
    c.name AS contract_name,
    c.symbol AS contract_symbol,
    m.name,
    m.description,
    m.image,
    m.external_url,
    d.chain,
    d.metadata_url,
    d.file_url,
    d.cached_file_url,
    d.mint_date,
    d.updated_date,
    d.token_id
   FROM (((public.nft_contract c
     JOIN public.nft_data d ON (((c.address)::text = (d.nft_contract_address)::text)))
     JOIN public.nft_file_information fi ON ((d.id = fi.nft_data_id)))
     JOIN public.nft_metadata m ON ((d.id = m.nft_data_id)));


ALTER TABLE public.v_nfts OWNER TO postgres;

--
-- TOC entry 3184 (class 2606 OID 17027)
-- Name: nft_contract pk_nft_contract_address; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nft_contract
    ADD CONSTRAINT pk_nft_contract_address PRIMARY KEY (address);


--
-- TOC entry 3187 (class 2606 OID 17034)
-- Name: nft_data pk_nft_data_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nft_data
    ADD CONSTRAINT pk_nft_data_id PRIMARY KEY (id);


--
-- TOC entry 3185 (class 1259 OID 17050)
-- Name: fki_nft_data_nft_contract_address; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_nft_data_nft_contract_address ON public.nft_data USING btree (nft_contract_address);


--
-- TOC entry 3188 (class 2606 OID 17045)
-- Name: nft_data fk_nft_data_nft_contract_address; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nft_data
    ADD CONSTRAINT fk_nft_data_nft_contract_address FOREIGN KEY (nft_contract_address) REFERENCES public.nft_contract(address) NOT VALID;


--
-- TOC entry 3190 (class 2606 OID 17054)
-- Name: nft_file_information fk_nft_file_information_nft_data_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nft_file_information
    ADD CONSTRAINT fk_nft_file_information_nft_data_id FOREIGN KEY (nft_data_id) REFERENCES public.nft_data(id) NOT VALID;


--
-- TOC entry 3189 (class 2606 OID 17059)
-- Name: nft_metadata fk_nft_metadata_nft_data_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nft_metadata
    ADD CONSTRAINT fk_nft_metadata_nft_data_id FOREIGN KEY (nft_data_id) REFERENCES public.nft_data(id) NOT VALID;


-- Completed on 2022-01-12 14:26:15 UTC

--
-- PostgreSQL database dump complete
--
