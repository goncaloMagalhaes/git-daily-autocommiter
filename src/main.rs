use git2::{ObjectType, Commit, Repository, Signature, RemoteCallbacks, PushOptions};
use std::{env, fs};
use chrono::{DateTime, Local};
use std::path::Path;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let repo_name = env::var("REPO_PATH").expect("REPO_NAME env var not found");
    let file_base_name = env::var("FILE_BASE_NAME").expect("FILE_BASE_NAME env var not found");
    
    let ssh_private_key_path = env::var("SSH_PRIVATE_KEY_PATH").expect("SSH_PRIVATE_KEY env var not found");
    let ssh_private_key = fs::read_to_string(ssh_private_key_path).expect("Could not read ssh key!!");

    let commit_author = env::var("COMMIT_AUTHOR").expect("COMMIT_AUTHOR env var not found");
    let commit_email = env::var("COMMIT_EMAIL").expect("COMMIT_EMAIL env var not found");

    // get the current date and time
    let now: DateTime<Local> = Local::now();
    let day = now.format("%Y-%m-%d").to_string();

    // create the full file name
    let file_name = format!("{}-{}", day, file_base_name);

    println!("{repo_name}");

    // open the repository
    let repo = match Repository::open(&repo_name) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open repo: {}", e),
    };

    // set up the ssh key
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, username, _| {
        git2::Cred::ssh_key_from_memory(username.unwrap(), None, &ssh_private_key, None)
    });

    let mut push_options = PushOptions::default();
    push_options.remote_callbacks(callbacks);

    // read the file
    let contents = match fs::read_to_string(&file_name) {
        Ok(contents) => contents,
        Err(_) => {
            let mut full_path_file = repo_name.clone();
            full_path_file.push_str(&file_name);
            fs::File::create(full_path_file).expect("File not created!");
            String::new()
        }
    };

    let signature = Signature::now(&commit_author, &commit_email).expect("Panic!");
    let mut index = repo.index().expect("Panic!");
    index.add_path(Path::new(&file_name)).expect("Panic!");
    let tree_id = index.write_tree().expect("Panic!");
    let tree = repo.find_tree(tree_id).expect("Panic!");

    let parent_commit = find_last_commit(&repo).expect("No last commit??");

    // if the file is not empty, add, commit and push the changes
    if !contents.is_empty() {
        let message = format!("My daily commit - stuff for {}", day);
        repo.commit(
            Some("HEAD"), 
            &signature, 
            &signature, 
            &message, 
            &tree, 
            &[&parent_commit]
        ).expect("Panic!");
    } else {
        // let message = format!("My daily commit - no recorded stuff for {}", day);
        let message = "This is my first test commit";
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &[&parent_commit]
        ).expect("Panic!");
    }

    let mut remote = repo.find_remote("origin").expect("Panic!");
    remote.push(&["refs/heads/develop"], Some(&mut push_options)).expect("Panic!");
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
