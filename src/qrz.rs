extern crate config;
extern crate dirs;
extern crate reqwest;

use serde_xml_rs::from_str;
use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Session {
    #[serde(rename="Key", default)]
    key: String,
    #[serde(rename="Count", default)]
    count: String,
    #[serde(rename="SubExp", default)]
    sub_exp: String,
    #[serde(rename="GMTime", default)]
    gmtime: String,
    #[serde(rename="Remark", default)]
    remark: String,
    #[serde(rename="Error", default)]
    error: String
}

impl Default for Session {
    fn default() -> Session {
        Session {
            key: "".to_string(),
            count: "".to_string(),
            sub_exp: "".to_string(),
            gmtime: "".to_string(),
            remark: "".to_string(),
            error: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Callsign {
    #[serde(default)]
    call: String,
    #[serde(default)]
    aliases: String,
    #[serde(default)]
    dxcc: String,
    #[serde(default)]
    fname: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    addr1: String,
    #[serde(default)]
    addr2: String,
    #[serde(default)]
    state: String,
    #[serde(default)]
    zip: String,
    #[serde(default)]
    country: String,
    #[serde(default)]
    ccode: String,
    #[serde(default)]
    lat: String,
    #[serde(default)]
    lon: String,
    #[serde(default)]
    grid: String,
    #[serde(default)]
    county: String,
    #[serde(default)]
    fips: String,
    #[serde(default)]
    land: String,
    #[serde(default)]
    efdate: String,
    #[serde(default)]
    expdate: String,
    #[serde(default)]
    p_call: String,
    #[serde(default)]
    class: String,
    #[serde(default)]
    codes: String,
    #[serde(default)]
    qslmgr: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    u_views: String,
    #[serde(default)]
    bio: String,
    #[serde(default)]
    image: String,
    #[serde(default)]
    serial: String,
    #[serde(default)]
    biodate: String,
    #[serde(default)]
    moddate: String,
    #[serde(rename="MSA", default)]
    msa: String,
    #[serde(rename="AreaCode", default)]
    area_code: String,
    #[serde(rename="TimeZone", default)]
    time_zone: String,
    #[serde(rename="GMTOffset", default)]
    gmt_offset: String,
    #[serde(rename="DST", default)]
    dst: String,
    #[serde(default)]
    user: String,
    #[serde(default)]
    esql: String,
    #[serde(default)]
    msql: String,
    #[serde(default)]
    cqzone: String,
    #[serde(default)]
    ituzone: String,
    #[serde(default)]
    geoloc: String,
    #[serde(default)]
    born: String
}

impl Default for Callsign {
    fn default() -> Callsign {
        Callsign {
            call: "".to_string(),
            aliases: "".to_string(),
            dxcc: "".to_string(),
            fname: "".to_string(),
            name: "".to_string(),
            addr1: "".to_string(),
            addr2: "".to_string(),
            state: "".to_string(),
            zip: "".to_string(),
            country: "".to_string(),
            ccode: "".to_string(),
            lat: "".to_string(),
            lon: "".to_string(),
            grid: "".to_string(),
            county: "".to_string(),
            fips: "".to_string(),
            land: "".to_string(),
            efdate: "".to_string(),
            expdate: "".to_string(),
            p_call: "".to_string(),
            class: "".to_string(),
            codes: "".to_string(),
            qslmgr: "".to_string(),
            email: "".to_string(),
            url: "".to_string(),
            u_views: "".to_string(),
            bio: "".to_string(),
            biodate: "".to_string(),
            image: "".to_string(),
            serial: "".to_string(),
            moddate: "".to_string(),
            msa: "".to_string(),
            area_code: "".to_string(),
            time_zone: "".to_string(),
            gmt_offset: "".to_string(),
            dst: "".to_string(),
            user: "".to_string(),
            esql: "".to_string(),
            msql: "".to_string(),
            cqzone: "".to_string(),
            ituzone: "".to_string(),
            geoloc: "".to_string(),
            born: "".to_string()
        }
    }
}

#[derive(Debug, Deserialize)]
struct Dxcc {
    #[serde(default)]
    dxcc: String,
    #[serde(default)]
    cc: String,
    #[serde(default)]
    ccc: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    continent: String,
    #[serde(default)]
    ituzone: String,
    #[serde(default)]
    cqzone: String,
    #[serde(default)]
    timezone: String,
    #[serde(default)]
    lat: String,
    #[serde(default)]
    lon: String,
    #[serde(default)]
    notes: String
}

impl Default for Dxcc {
    fn default() -> Dxcc {
        Dxcc {
            dxcc: "".to_string(),
            cc: "".to_string(),
            ccc: "".to_string(),
            name: "".to_string(),
            continent: "".to_string(),
            ituzone: "".to_string(),
            cqzone: "".to_string(),
            timezone: "".to_string(),
            lat: "".to_string(),
            lon: "".to_string(),
            notes: "".to_string()
        }
    }
}

#[derive(Debug, Deserialize)]
struct QrzDatabase {
    version: String,
    #[serde(rename="Session")]
    session: Session,
    #[serde(rename="Callsign", default)]
    callsign: Callsign,
    #[serde(rename="DXCC", default)]
    dxcc: Dxcc
}

pub fn session() -> Result<(), reqwest::Error> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };

    let config_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.toml");
    let mut settings = config::Config::default();
    settings.merge(config::File::from(Path::new(&config_path))).unwrap();
    let username = settings.get_str("qrz_callsign").unwrap();
    let password = settings.get_str("qrz_password").unwrap();

    let client = reqwest::Client::new();
    let params = [("username", username), ("password", password)];
    let query_resp = client.post("https://xmldata.qrz.com/xml/current/")
        .form(&params)
        .send()?
        .text()?;

    let qrzdb: QrzDatabase = from_str(&query_resp).unwrap();

    if qrzdb.session.error != ""
    {
        panic!("ERROR! {}", qrzdb.session.error);
    } else {
        set_session(qrzdb.session.key);
        Ok(())
    }
}

pub fn set_session(key: String) -> std::io::Result<()> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };
    let session_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.qrz");

    fs::write(&session_path, key)?;
    Ok(())
}

pub fn get_session() -> String {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };
    let session_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.qrz");

    if !Path::new(&session_path).exists() {
        let _s = match session() {
            Ok(k) => k,
            Err(_e) => panic!("error")
        };
    }

    let mut file = File::open(&session_path).expect("Unable to open the file");
    let mut session_id = String::new();
    file.read_to_string(&mut session_id).expect("Unable to read the file");

    return session_id;
}

pub fn query(callsign: &str) -> Result<(), reqwest::Error> {
    let session_id = get_session();

    let client = reqwest::Client::new();

    let query_resp = client.get("http://xmldata.qrz.com/xml/current/")
        .query(&[("s", session_id), ("callsign", callsign.to_string())])
        .send()?
        .text()?;

    let qrzdb: QrzDatabase = from_str(&query_resp).unwrap();

    if qrzdb.session.error == "Session Timeout" || qrzdb.session.error == "Username / password required" {
        let _s = match session() {
            Ok(k) => k,
            Err(_e) => panic!("error")
        };
        query(callsign)?;
        Ok(())
    } else if qrzdb.session.error != "" {
        panic!("ERROR! {}", qrzdb.session.error);
    } else {
        println!("\n{} (QRZ)", qrzdb.callsign.call);
        println!("  Name: {} {}", qrzdb.callsign.fname, qrzdb.callsign.name);
        println!("  Location: {}, {}, {}", qrzdb.callsign.addr2, qrzdb.callsign.state, qrzdb.callsign.land);
        println!("  Class: {}", qrzdb.callsign.class);
        Ok(())
    }
}

pub fn dxcc(entity: &str) -> Result<(), reqwest::Error> {
    let session_id = get_session();

    let client = reqwest::Client::new();

    let query_resp = client.get("http://xmldata.qrz.com/xml/current/")
        .query(&[("s", session_id), ("dxcc", entity.to_string())])
        .send()?
        .text()?;

    let qrzdb: QrzDatabase = from_str(&query_resp).unwrap();

    if qrzdb.session.error == "Session Timeout" {
        let _key = match session() {
            Ok(k) => k,
            Err(_e) => panic!("error")
        };

        query(entity)?;
        Ok(())
    } else if qrzdb.session.error != "" {
        panic!("ERROR! {}", qrzdb.session.error);
    } else {
        println!("\n{} (QRZ)", qrzdb.dxcc.dxcc);
        println!("  Name: {}", qrzdb.dxcc.name);
        println!("  ITU: {}", qrzdb.dxcc.ituzone);
        println!("  CQ: {}", qrzdb.dxcc.cqzone);
        println!("  UTC: {}", qrzdb.dxcc.timezone);
        Ok(())
    }
}