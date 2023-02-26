use std::env;
use std::fs::File;
use std::io::Write;
use std::process;

// main fn with optional args --help and --version
fn main() {
    let args: Vec<String> = env::args().collect();

    create_docker_file(args);
}

fn create_docker_file(
    // args
    args: Vec<String>,
) {
    // if args contains --help or help or --version or version
    if args.contains(&"--help".to_string())
        || args.contains(&"help".to_string())
        || args.contains(&"--version".to_string())
        || args.contains(&"version".to_string())
    {
        // print help
        println!(
            "
        Usage: edoc [folder] [options]

        Options:
            --help      Show this help message
            --version   Show version

            --port      Set port to expose

            node       Create a docker file for nodejs
            python     Create a docker file for python
            dotnet     Create a docker file for dotnet
            ruby       Create a docker file for ruby
            vue        Create a docker file for vue
            react      Create a docker file for react
        "
        );
        // exit
        process::exit(0);
    }

    // get folder location
    let folder_location = args[1].clone();

    // check directory exists
    if !std::path::Path::new(&folder_location).exists() {
        println!("Directory does not exist or is not a directory. --help for more info");
        process::exit(1);
    }

    // check if docker file exists replace it DockerFile-old
    if std::path::Path::new("Dockerfile").exists() {
        std::fs::rename("Dockerfile", "Dockerfile-old").unwrap();
    }

    // create docker file
    let mut file = File::create("Dockerfile").unwrap();

    // foreach args
    for arg in args.iter().skip(1) {
        if arg == "node" {
            // main content
            file.write_all(b"FROM node:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y nodejs\n").unwrap();
            file.write_all(b"RUN apt-get install -y npm\n").unwrap();
            file.write_all(b"RUN apt-get install -y yarn\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN npm install\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"npm\", \"start\"]\n").unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "python" {
            // main content
            file.write_all(b"FROM python:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y python3\n").unwrap();
            file.write_all(b"RUN apt-get install -y python3-pip\n")
                .unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN pip3 install -r requirements.txt\n")
                .unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"python3\", \"app.py\"]\n").unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "dotnet" {
            // main content
            file.write_all(b"FROM mcr.microsoft.com/dotnet/core/sdk:latest\n")
                .unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y dotnet\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN dotnet restore\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"dotnet\", \"run\"]\n").unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "ruby" {
            // main content
            file.write_all(b"FROM ruby:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y ruby\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN bundle install\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"ruby\", \"app.rb\"]\n").unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "vue" {
            // main content
            file.write_all(b"FROM node:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y npm\n").unwrap();
            file.write_all(b"RUN apt-get install -y yarn\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN npm install\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"npm\", \"run\", \"serve\"]\n")
                .unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "react" {
            // main content
            file.write_all(b"FROM node:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y npm\n").unwrap();
            file.write_all(b"RUN apt-get install -y yarn\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN npm install\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"npm\", \"run\", \"start\"]\n")
                .unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "react" {
            // main content
            file.write_all(b"FROM node:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y npm\n").unwrap();
            file.write_all(b"RUN apt-get install -y yarn\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN npm install\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"npm\", \"run\", \"start\"]\n")
                .unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        } else if arg == "angular" {
            // main content
            file.write_all(b"FROM node:latest\n").unwrap();
            file.write_all(b"RUN apt-get update && apt-get install -y\n")
                .unwrap();
            file.write_all(b"RUN apt-get install -y npm\n").unwrap();
            file.write_all(b"RUN apt-get install -y yarn\n").unwrap();

            // copy folder to docker
            file.write_all(format!("COPY {} /usr/src/app/\n", folder_location).as_bytes())
                .unwrap();

            // set working directory
            file.write_all(b"WORKDIR /usr/src/app/\n").unwrap();

            // install dependencies
            file.write_all(b"RUN npm install\n").unwrap();

            // expose port 3000 || args containt --port or port
            if args.contains(&"--port".to_string()) || args.contains(&"port".to_string()) {
                // get port
                let port = args[args
                    .iter()
                    .position(|r| r == "--port" || r == "port")
                    .unwrap()
                    + 1]
                .clone();

                // expose port
                file.write_all(format!("EXPOSE {}\n", port).as_bytes())
                    .unwrap();
            } else {
                // expose port 3000
                file.write_all(b"EXPOSE 3000\n").unwrap();
            }

            // start app
            file.write_all(b"CMD [\"npm\", \"run\", \"start\"]\n")
                .unwrap();

            // Log okay
            println!("Dockerfile created successfully");

            // exit
            process::exit(0);
        }
    }
}
