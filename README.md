# Git Daily Autocommiter
How about you log your daily activities on github? Look no further!

Git Daily Autocommiter is a script written in Rust that reads a file and commits and pushes its contents to a GitHub repository. You can use it to automatically log your daily activities, such as the papers or articles you've read, the repositories you've contributed to, the languages you've started learning, etc.

I'm learning Rust, so really the point here was also for me to learn while doing.

## Getting started
To get started with Git Daily Autocommiter, you'll need to set up a few environment variables. Here's what you'll need:

- `GITHUB_API_KEY`: Your GitHub API key for authorization.
- `REPO_NAME`: The name of the GitHub repository where you want to push the code.
- `FILE_BASE_NAME`: The base name of the file you want to read from. The script will concatenate this with the current date to create the actual file name to read from.
- `USERNAME`: Your GitHub username.

Once you have these environment variables set up, you can run the script by simply executing `cargo run` in your terminal.

## Automating with crontab
To make sure you never forget to commit your logged daily activities, you can add Git Daily Autocommiter to your crontab. Something like this:

```
0 0 * * * <path-to-your-script>/git-daily-autocommiter
```

This line tells crontab to run the script every day at midnight. You can change the schedule to fit your needs by modifying the first five fields of the line.
