use std::env;
use actix_web::{web, HttpResponse, ResponseError};
use aws_sdk_s3::{Client, Error as SdkError};
use aws_sdk_s3::config::{Builder, Credentials, Region};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct ReqData{
    repl_id: String,
    language: String
}


#[derive(Debug)]
pub struct MyError(String); // <-- needs debug and display

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occurred on the input.")
    }
}
impl ResponseError for MyError {} // <-- key
pub async fn create_project(req_data: web::Json<ReqData>) -> Result<HttpResponse, MyError> {
    copy_s3_folder(&req_data.language,&req_data.repl_id).await;
    Ok(HttpResponse::from(HttpResponse::Ok()
        .content_type("application/json")
        .body("Project Created")
    ))
}

async fn get_aws_client(aws_access_key: &String, aws_secret: &String, region: &String, cloud_flare_endpoint: &String ) -> Result<Client, SdkError>{
    let cred = Credentials::new(aws_access_key, aws_secret, None, None, "loaded-from-custom-env");
    let reg = Region::new(region.to_owned());
    let conf_builder = Builder::new().region(reg).credentials_provider(cred).endpoint_url(cloud_flare_endpoint);
    let conf = conf_builder.build();

    let client = Client::from_conf(conf);
    Ok(client)
}

async fn list_keys(client: &Client, bucket_name: String) -> Result<Vec<String>, SdkError>{
    let req = client.list_objects_v2().prefix("").bucket(bucket_name);

    let res = req.send().await?;

    let keys = res.contents();

    let keys: Vec<String> = keys
        .iter()
        .filter_map(|o| o.key.as_ref())
        .map(|s| s.to_string())
        .collect();
    Ok(keys)
}

pub async fn copy_s3_folder(source_prefix: &String, destination_prefix: &String){
    let aws_access_key: String = env::var("CLOUDFLARE_ACCESS_KEY").expect("CLOUDFLARE_ACCESS_KEY must be set");
    let aws_secret = env::var("CLOUDFLARE_ACCESS_SECRET").expect("AWS_SECRET must be set");
    let cloud_flare_endpoint = env::var("CLOUDFLARE_ENDPOINT").expect("CLOUDFLARE_ENDPOINT must be set");
    let bucket_name: String = env::var("S3_BASE_BUCKET").expect("S3_BASE_BUCKET must be set");
    let code_bucket_name = env::var("S3_CODE_BUCKET").expect("S3_CODE_BUCKET must be set");
    let region: String = String::from("auto");
    let client = get_aws_client(&aws_access_key, &aws_secret, &region, &cloud_flare_endpoint).await.unwrap();

    let req = client.list_objects_v2().bucket(&bucket_name).prefix(source_prefix);
    let res = req.send().await.unwrap();
    let mut contents = res.contents();
    if contents.len() == 0{
        return;
    }
    for object in contents{
        let destination_key = object.key.clone().unwrap().replace(source_prefix, destination_prefix);
        let res = client
            .copy_object()
            .copy_source(format!("{}/{}",bucket_name,object.key().unwrap()))
            .key(&destination_key)
            .bucket(&code_bucket_name)
            .send()
            .await
            .unwrap();
    }

}


