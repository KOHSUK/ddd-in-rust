use clap::{ Parser, ArgEnum };
use anyhow::{ Result, anyhow };

use crate::interface::controller::user_controller::{UserController, PostArgs, GetArgs, PutArgs, DeleteArgs};

pub struct CommandLine {
    args: Args,
    user_controller: UserController,
}

impl CommandLine {
    pub async fn new() -> Result<Self> {
        let args = Args::parse();
        let user_controller = UserController::new().await?;

        Ok(Self { args, user_controller })
    }    

    pub async fn start(&self) -> Result<()> {
        match self.args.operation {
            Operation::Create => {
                self.create().await
            },
            Operation::Read => {
                self.read().await
            },
            Operation::Update => {
                self.update().await
            },
            Operation::Delete => {
                self.delete().await
            },
        }
    }

    async fn create(&self) -> Result<()> {
        if let Some(name) = self.args.name.as_ref() {
            self.user_controller.post(PostArgs { name: name.to_string() }).await
        } else {
            Err(anyhow!("`name` option is required."))
        }
    }

    async fn read(&self) -> Result<()> {
        if let Some(id) = self.args.id.as_ref() {
            println!("id: {}", id);
            let args = GetArgs { id: id.to_string() };
            if let Some(user) = self.user_controller.get(args).await {
                println!("{:?}", user);
            } else {
                println!("Could not find user.");
            }
            Ok(())
        } else {
            Err(anyhow!("`id` option is required."))
        }
    }

    async fn update(&self) -> Result<()> {
        if let Some(id) = self.args.id.as_ref() {
            if let Some(name) = self.args.name.as_ref() {
                let args = PutArgs { id: id.to_string(), name: name.to_string() };
                self.user_controller.put(args).await
            } else {
                Err(anyhow!("`id` option id required."))
            }
        } else {
            Err(anyhow!("`name` option id required."))
        }
    }

    async fn delete(&self) -> Result<()> {
        if let Some(id) = self.args.id.as_ref() {
            let args = DeleteArgs { id: id.to_string() };
            self.user_controller.delete(args).await
        } else {
            Err(anyhow!("`id` option is required."))
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "DDD in Rust",
    author = "Kohsuk",
    version = "v0.0.1",
    about = "Example Command Line Application using DDD in Rust",
    long_about = None
)]
pub struct Args {
    #[clap(arg_enum, short, long)]
    operation: Operation,
    #[clap(short, long)]
    name: Option<String>,
    #[clap(short, long)]
    id: Option<String>,
}

#[derive(ArgEnum, Clone, Debug)]
enum Operation {
    Create,
    Read,
    Update,
    Delete,
}
