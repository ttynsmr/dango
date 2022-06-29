use dependencies::plugins::github;
use dotenv::dotenv;

mod dependencies;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    dotenv().ok();
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![greet, fetch_note])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// #[tauri::command]
// fn fetch_note(query: &str) -> String {
//     format!("fetch note query: {}", query);

//     let mut notes: dependencies::notes::Notes = dependencies::notes::Notes::new();
//     match github::github_call(&mut notes, query.to_string()) {
//         Ok(()) => {
//             let mut phase = 1;
//             while {
//                 println!(
//                     "=========================================== phase {}",
//                     phase
//                 );
//                 phase += 1;
//                 notes.analyze()
//             } {}

//             notes.to_string()
//         }
//         Err(e) => {
//             println!("{}", e);
//             e.to_string()
//         }
//     }
// }

#[tauri::command]
fn fetch_note(url: &str) -> String {
    format!("fetch note url: {}", url);

    let mut notes: dependencies::notes::Notes = dependencies::notes::Notes::new();
    notes.append_url(url);

    let mut phase = 1;
    while {
        println!(
            "=========================================== phase {}",
            phase
        );
        phase += 1;
        notes.analyze()
    } {}

    notes.to_string()
}
