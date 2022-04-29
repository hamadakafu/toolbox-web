use std::collections::HashMap;

use actix_web::{get, web, HttpResponse};
use pbkdf2::{
    password_hash::{PasswordHasher, Salt},
    Algorithm, Params,
};

pub async fn index(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> actix_web::Result<HttpResponse> {
    let prf = query.get("prf");
    let password_opt = query.get("password");
    let salt_opt = query.get("salt");
    let rounds_opt = query.get("rounds");
    let dklen = query.get("dklen");

    let s = match (password_opt, salt_opt, rounds_opt, dklen) {
        (Some(password), Some(salt), Some(rounds), Some(dklen)) => {
            let rounds = rounds.parse::<u32>().unwrap();
            let dklen = dklen.parse::<usize>().unwrap();

            let alg = match prf {
                Some(prf) => match prf.as_str() {
                    "sha1" => Algorithm::Pbkdf2Sha1,
                    "sha256" => Algorithm::Pbkdf2Sha256,
                    "sha512" => Algorithm::Pbkdf2Sha512,
                    _ => Algorithm::Pbkdf2Sha1,
                },
                None => Algorithm::Pbkdf2Sha1,
            };
            let salt_b64 = base64_url::encode(salt);
            let r = pbkdf2::Pbkdf2
                .hash_password_customized(
                    password.as_bytes(),
                    Some(alg.into()),
                    None,
                    Params {
                        rounds,
                        output_length: dklen,
                    },
                    Salt::new(&salt_b64).unwrap(),
                )
                .unwrap();
            let mut value_hex = String::new();
            for b in r.hash.unwrap().as_bytes() {
                value_hex.push_str(&format!("{:02x}", b));
            }
            // submitted form
            let mut ctx = tera::Context::new();
            ctx.insert("password", password);
            ctx.insert("salt", salt);
            ctx.insert("rounds", &rounds);
            ctx.insert("dklen", &dklen);
            ctx.insert("value", &value_hex);
            tmpl.render("pbkdf2/index.html", &ctx).unwrap()
        }
        _ => tmpl
            .render("pbkdf2/index.html", &tera::Context::new())
            .unwrap(),
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
// 734B6E882CC0E93F8688A5FBCB06AF24DC605CB23CF8C3AE0AC1A5B3870A3F00
