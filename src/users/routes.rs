use super::dao::IUser;
use super::user::{UserAddress, Email};
use crate::api::ApiResult;
use crate::state::AppState;
use ed25519_dalek::{ed25519, PublicKey, SignatureError, Verifier};

use mailgun_v3::email::{Message,EmailAddress , MessageBody};
use mailgun_v3::Credentials;

use {
    ruc::*,
    attohttpc,
    base64,
    serde_json,
};
use actix_web::{get, post, web, Responder};
use futures::future::err;
use crate::users::user::QueryAll;

async fn verify_sign(form: &UserAddress, state: &AppState) -> Result<()> {
    match  PublicKey::from_bytes(&form.anonymous_address.as_bytes())
    {
        Ok(public_key) => {
            let sign = ed25519::Signature::from_bytes(form.signature.as_bytes()).unwrap();
            return  public_key.verify(form.signature_data.as_bytes(), &sign).c(d!());
        },
        Err(e) => {
            return Err(eg!("verify fail{:?}",e));
        }
    }
}

#[get("/address/{address}")]
async fn get_address(address: web::Path<String>, state: AppState) -> impl Responder {
    let address = address.into_inner();
    match state.get_ref().adress_query(&address).await {
        Ok(addex) => {
            ApiResult::new().with_msg("ok").with_data(addex)
        },
        Err(_) => {
            println!("no found address");
            return  ApiResult::new().code(400).with_msg("no found address");
        }
    }
}

#[get("/addressall")]
async fn get_address_all(form: web::Json<QueryAll>, state: AppState) -> impl Responder {
    let form = form.into_inner();
    match state.get_ref().adress_all(form.limit, form.offset).await {
        Ok(addex) => {
            ApiResult::new().with_msg("ok").with_data(addex)
        },
        Err(_) => {
            println!("no found file");
            return  ApiResult::new().code(400).with_msg("no found file");
        }
    }
}

#[post("/address")]
async fn user_address(form: web::Json<UserAddress>, state: AppState) -> impl Responder {
    let form = form.into_inner();
    if let Err(e) = verify_sign(&form, &state).await{
        print!("Signature verification failed, Err = {:?}",e);
        return ApiResult::new().code(400).with_msg("Signature verification failed");
    }

    let url = state.get_ref().config.request_rpc.clone() + &*form.transaction_hash.clone();
    let resp = attohttpc::get(&url).send().c(d!()).unwrap();
    if resp.is_success() {
        let object: serde_json::Value = resp.json_utf8().c(d!()).unwrap();
        match object.get("result").c(d!())
        {
            Ok(rpc_result) => {
                let tx = rpc_result.get("tx").c(d!()).unwrap();
                let genesis_str = tx.as_str().unwrap();
              //  println!("Request success : tx = {:?}", std::str::from_utf8(genesis_str.as_ref()).unwrap());
                let tx_decode = base64::decode_config(&genesis_str, base64::URL_SAFE)
                    .map_err(|e| error!("erro : {:?}", e)).unwrap();

                let str_json = String::from_utf8(tx_decode).unwrap();
                let object_tx:serde_json::Value = serde_json::from_str(str_json.as_str()).unwrap();

                if object_tx.is_object()
                {
                    let body_signatures = object_tx.get("body").c(d!()).unwrap_or(&serde_json::Value::Null  )
                        .get("operations").c(d!()).unwrap_or(&serde_json::Value::Null  )
                        .get(0).c(d!()).unwrap_or(&serde_json::Value::Null  )
                        .get("TransferAsset").c(d!()).unwrap_or(&serde_json::Value::Null  )
                        .get("body_signatures").c(d!()).unwrap_or(&serde_json::Value::Null  );
                    if body_signatures.is_array() {
                        let vec_json =  body_signatures.as_array().c(d!()).unwrap();
                        let mut b_find = false;
                        for json_address in vec_json.iter(){
                            let data_address = json_address.get(0).c(d!()).unwrap_or(&serde_json::Value::Null  )
                                .get("address").c(d!()).unwrap_or(&serde_json::Value::Null  )
                                .get("key").c(d!()).unwrap_or(&serde_json::Value::Null  );

                            let address_str = data_address.as_str().unwrap();
                            if form.anonymous_address == address_str
                            {
                                b_find = true;
                                break;
                            }
                        }
                        if !b_find
                        {
                            return ApiResult::new().code(400).with_msg("tx Address mismatch");
                        }
                    }
                }
            }
            Err(_) => {
                return ApiResult::new().code(400).with_msg("tx not found");
            }
        }
    } else {
        return ApiResult::new().code(400).with_msg("tx Request failed");
    }

    if !form.share_address.is_empty() && form.share_address != form.evm_anonymous_address{
        let mut b_find = false;
        match state.get_ref().adress_query(&form.share_address.clone()).await {
            Ok(addex) => {
                b_find = true;
                let mut addres :i16 = addex.experience.parse::<i16>().unwrap();
                addres = addres + 1;
                match state.get_ref().adress_update(&form.share_address.clone(), &addres.to_string()).await {
                    Ok(_) => {}
                    Err(e) => {
                      return  ApiResult::new().code(400).with_msg(e.to_string());
                    }
                }
            },
            Err(_) => {
                print!("Not Find Address");
            }
        }
        if !b_find{
            match state.get_ref().adress_add(&form.share_address, "1").await {
                Ok(_) => {}
                Err(e) => {
                    return ApiResult::new().code(400).with_msg(e.to_string());
                }
            }
        }
    }

    match state.get_ref().adress_query(&form.evm_anonymous_address.clone()).await {
        Ok(addex) => {
            let mut addres :i16 = addex.experience.parse::<i16>().unwrap();
            addres = addres + 1;
            match state.get_ref().adress_update(&form.evm_anonymous_address.clone(), &addres.to_string()).await {
                Ok(res) => {
                   return ApiResult::new().with_msg("ok").with_data(res);
                }
                Err(e) => {
                    return  ApiResult::new().code(400).with_msg(e.to_string());
                }
            }
        },
        Err(_) => {
            print!("no find address");
        }
    }
    match state.get_ref().adress_add(&form.evm_anonymous_address, "1").await {
        Ok(res) => {
            ApiResult::new().with_msg("ok").with_data(res)
        }
        Err(e) => {
            ApiResult::new().code(400).with_msg(e.to_string())
        }
    }
}

#[post("/email")]
async fn post_email(form: web::Json<Email>, state: AppState) -> impl Responder {

    let form = form.into_inner();
    let domain = state.get_ref().config.mail_domain.as_str();
    let key = state.get_ref().config.mail_key.as_str();

    let msg = Message {
        to: vec![EmailAddress::address(form.email)],
        body: MessageBody::Text(form.url.parse().unwrap()),
        subject: String::from("Your Url"),
        ..Default::default()
    };
    let sender = EmailAddress::address(state.get_ref().config.mail_post.clone());
    let creds = Credentials::new(
        key,
        domain,
    );

    match mailgun_v3::email::send_email(&creds, &sender, msg) {
        Ok(res) => {
            return ApiResult::new().code(200).with_msg("Email sent successfully!");
        }
        Err(e) => {
            error!("Could not send email: {:?}", e);
            return ApiResult::new().code(200).with_data(e.to_string());
        }
    }

/*
    let email = Message::builder()
        .from(state.get_ref().config.mail_post.parse().unwrap())  // 发件人
        .to(form.email.parse().unwrap())        // 收件人
        .subject("Your Url")  // 主题
        .body(form.url)          // 邮件内容
        .unwrap();
    let creds = Credentials::new(state.get_ref().config.mail_post.clone(), state.get_ref().config.mail_password.clone());
    let mailer = SmtpTransport::relay(state.get_ref().config.mail_smtp.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            return ApiResult::new().code(200).with_msg("Email sent successfully!");
        }
        Err(e) => {
                error!("Could not send email: {:?}", e);
                return ApiResult::new().code(200).with_data(e.to_string());
            }
    }*/
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(user_address);
    cfg.service(get_address);
    cfg.service(post_email);
    cfg.service(get_address_all);
}
