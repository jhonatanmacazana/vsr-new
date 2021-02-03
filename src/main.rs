use exitfailure::ExitFailure;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;
use termion::color;

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
struct GHResponse {}

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

impl GHResponse {
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
            "https://api.github.com/repos/jhonatanmacazana/vscode-boilerplates/contents{}",
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
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    if args.types {
        let gh_response = GHResponse {};
        println!("{}Fetching available types...", color::Fg(color::LightRed));
        let _resp = gh_response.get_types().await?;
        for elem in _resp {
            println!("{}{}", color::Fg(color::LightBlue), elem);
        }
        return Ok(());
    }
    println!(
        "City: {:?}. CC: {:?}. F: {}",
        args.template_type, args.new_app, args.types
    );

    fs::create_dir_all(args.new_app.unwrap())?;

    Ok(())
}
