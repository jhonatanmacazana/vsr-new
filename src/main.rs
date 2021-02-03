use async_recursion::async_recursion;
use crossterm::style::Colorize;
use exitfailure::ExitFailure;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    types: bool,

    #[structopt(required_unless("types"))]
    template_type: Option<String>,

    #[structopt(required_unless("types"))]
    new_app: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BpHandler {}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: Option<String>,
    r#type: String,
}

impl BpHandler {
    async fn get_types(&self) -> Result<Vec<String>, ExitFailure> {
        let content = self.get_content("/").await?;

        let mut types = vec![];
        for elem in content {
            if elem.r#type != "dir" {
                continue;
            }
            types.push(elem.name);
        }
        Ok(types)
    }
    async fn get_content(&self, endpoint: &str) -> Result<Vec<Details>, ExitFailure> {
        let url_str = format!(
            "https://api.github.com/repos/jhonatanmacazana/vscode-boilerplates/contents/{}",
            endpoint
        );
        let url = Url::parse(&*url_str)?;

        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header(USER_AGENT, "vsr-new rust app")
            .send()
            .await?;
        let txt = resp.text().await?;
        let parsed: Vec<Details> = serde_json::from_str(&txt)?;
        Ok(parsed)
    }

    #[async_recursion]
    async fn create_dir_from_template(
        &self,
        web_content_path: &str,
        parent_folder_path: &str,
    ) -> Result<(), ExitFailure> {
        let path_content = self.get_content(&web_content_path).await?;
        create_dir_all(parent_folder_path)?;
        for elem in path_content {
            let file_path = format!("{}/{}", parent_folder_path, elem.name);
            if elem.r#type == "file" {
                self.copy_web_content_to_file(&elem.download_url.unwrap(), &file_path)
                    .await?;
            } else if elem.r#type == "dir" {
                let child_dir_path = format!("{}/{}", parent_folder_path, elem.name);
                self.create_dir_from_template(&elem.path, &child_dir_path)
                    .await?;
            }
        }

        Ok(())
    }

    async fn copy_web_content_to_file(
        &self,
        web_url: &str,
        path_file: &str,
    ) -> Result<(), ExitFailure> {
        let mut out = File::create(path_file)?;

        let client = reqwest::Client::new();
        let resp = client
            .get(web_url)
            .header(USER_AGENT, "vsr-new rust app")
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let mut slice: &[u8] = bytes.as_ref();

        io::copy(&mut slice, &mut out)?;
        print!("  {} ", "created".cyan());
        println!(": {}", path_file);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    if args.types {
        let gh_response = BpHandler {};
        println!("Fetching available types...");
        let _resp = gh_response.get_types().await?;
        for elem in _resp {
            println!("{} ", elem.blue());
        }
        return Ok(());
    }

    let gh_response = BpHandler {};

    let template_type = args.template_type.unwrap();
    let new_app = args.new_app.unwrap();

    print!("{} ", "Creating");
    print!("{} ", new_app.clone().green());
    print!("{} ", "project of type");
    println!("{} ", template_type.clone().green());
    println!("");
    gh_response
        .create_dir_from_template(&template_type, &new_app)
        .await
        .expect("There was an error creating the custom template");

    Ok(())
}
