use std::fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

fn load_passwords() -> Vec<String> {
    let mut passwords = Vec::new();
    let contents = fs::read_to_string("./data/passwords.lst").expect("Should have been able to read the file");
    let lines = contents.lines();
    for _line in lines {
        let line = _line.to_owned();
        if line != "" {
            passwords.push(line);
        }
    }
    return passwords;
}

fn validate_password(pass: String) -> bool {
    let passwords = load_passwords();
    let mut found = false;
    for p in passwords {
        if p.to_owned() == pass {
            found = true;
        }
    }
    return found;
}

#[get("/list")]
async fn list_bans() -> impl Responder {
    let contents = fs::read_to_string("./data/bans.lst").expect("Should have been able to read the file");
    let mut data = "".to_owned();
    for line in contents.lines() {
        let mut split = line.split("|");
        let target = split.next().to_owned().unwrap();
        let reason = split.next().to_owned().unwrap_or("");
        let time = split.next().to_owned().unwrap_or("-1");
        data.push_str(&format!("{{ target = \"{}\", reason = \"{}\", time = {} }}, ", target, reason, if time == "" { "-1" } else { time }));
    }
    return HttpResponse::Ok().body(format!("{{ status = \"ok\", result = {{ {} }} }}", data));
}

#[derive(Deserialize)]
struct Info {
    p: Option<String>,
}

#[derive(Deserialize)]
struct Ban {
    target: String,
    reason: Option<String>,
    time: Option<String>,
    p: Option<String>,
}

#[derive(Deserialize)]
struct Unban {
    target: String,
    p: Option<String>,
}

#[get("/add")]
async fn add_ban(payload: web::Query<Ban>) -> impl Responder {
    if payload.p == None {
        return HttpResponse::Ok().body("{ status = \"error\", error = \"No password provided\" }");
    }

    let authed = validate_password(payload.p.clone().unwrap());

    if !authed {
        return HttpResponse::Ok().body("{ status = \"error\", error = \"Invalid password\" }");
    }

    let contents = fs::read_to_string("./data/bans.lst").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut data = "".to_owned();
    for _line in lines {
        let line = _line.to_owned();
        if !line.contains(&format!("{}|", payload.target)) {
            data.push_str(&(line + "\n"));
        }
    }
    let reason = payload.reason.clone().unwrap_or("".to_string());
    let time = payload.time.clone().unwrap_or("".to_string());
    data.push_str(&format!("{}|{}|{}", payload.target, reason, time));
    fs::write("./data/bans.lst", data).expect("Unable to write file");
    return HttpResponse::Ok().body(format!("{{ status = \"ok\", result = \"{}\" }}", payload.target))
}

#[get("/remove")]
async fn remove_ban(payload: web::Query<Unban>) -> impl Responder {
    if payload.p == None {
        return HttpResponse::Ok().body("{ status = \"error\", error = \"No password provided\" }");
    }

    let authed = validate_password(payload.p.clone().unwrap());

    if !authed {
        return HttpResponse::Ok().body("{ status = \"error\", error = \"Invalid password\" }");
    }

    let contents = fs::read_to_string("./data/bans.lst").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut data = "".to_owned();
    let mut removed = false;
    for _line in lines {
        let line = _line.to_owned();
        if !line.contains(&format!("{}|", payload.target)) {
            data.push_str(&(line + "\n"));
        } else {
            removed = true;
        }
    }
    fs::write("./data/bans.lst", data).expect("Unable to write file");
    if removed {
        return HttpResponse::Ok().body(format!("{{ status = \"ok\", result = \"{}\" }}", payload.target))
    } else {
        return HttpResponse::Ok().body(format!("{{ status = \"ok\", result = nil, meta = \"No recorded target found for {}\" }}", payload.target))
    }
}

#[get("/info")]
async fn info(payload: web::Query<Info>) -> impl Responder {
    let mut authed = false;

    if payload.p != None {
        authed = validate_password(payload.p.clone().unwrap());
    }

    let contents = fs::read_to_string("./data/info.cfg").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut data = "".to_owned();
    for _line in lines {
        let line = _line.to_owned();
        if line.contains("info=") || line.contains("contact=") {
            data.push_str(&(line + ", "));
        }
    }

    if authed {
        return HttpResponse::Ok().body(format!("{{ status = \"ok\", result = {{ {} features = {{ \"list\", \"add\", \"remove\" }} }} }}", data))
    } else {
        return HttpResponse::Ok().body(format!("{{ status = \"ok\", result = {{ {} features = {{ \"list\" }} }} }}", data))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("80".to_string());
    println!("Running on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(list_bans)
            .service(add_ban)
            .service(remove_ban)
            .service(info)
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}