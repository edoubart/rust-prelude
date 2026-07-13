/*
 * Project: Web Scrapping
 */

use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};
use ureq::{Agent};

fn main() -> Result<(), ureq::Error> {
    let web_pages: Vec<&str> = vec![
        "https://github.com/Dicklesworthstone/destructive_command_guard",
        "https://github.com/wonderwhy-er/DesktopCommanderMCP",
        "https://github.com/HKUDS/Vibe-Trading",
        "https://github.com/PrefectHQ/prefect",
        "https://github.com/Shubhamsaboo/awesome-llm-apps",
        "https://github.com/anthropics/claude-cookbooks",
        "https://github.com/home-assistant/core",
        "https://github.com/Crosstalk-Solutions/project-nomad",
        "https://github.com/ColeMurray/background-agents",
        "https://github.com/k1tbyte/Wand-Enhancer",
        "https://github.com/virattt/ai-hedge-fund",
    ];

    let agent = Agent::new_with_defaults();
    let now = Instant::now();

    for web_page in &web_pages {
        let web_body: String = agent.get(*web_page)
            .call()?
            .body_mut()
            .read_to_string()?;
    }

    println!("Time taken without threads: {:.2?}", now.elapsed());

    let now = Instant::now();
    let agent = Arc::new(agent);
    //let mut handles = Vec::new();
    let mut handles: Vec<JoinHandle<Result<_, ureq::Error>>> = Vec::new();

    for web_page in &web_pages {
        let web_page_owned = web_page.to_string(); // Convert &str to String for move.
        let agent_thread = agent.clone();
        let t = thread::spawn(move || {
            let web_body: String = agent_thread.get(&web_page_owned)
                .call()?
                .body_mut()
                .read_to_string()?;

            Ok(())
        });
        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Time taken without threads: {:.2?}", now.elapsed());

    Ok(())
}
