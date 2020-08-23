#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use orogene::Orogene;

#[async_std::main]
async fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      println!("{}", arg);
      Ok(())
      // async_std::task::block_on(async {
      //   match serde_json::from_str(arg) {
      //     Err(e) => {
      //       Err(e.to_string())
      //     }
      //     Ok(args) => {
      //       Orogene::load_from(args).await.map_err(|e| e.to_string())?;
      //       Ok(())
      //     }
      //   }
      // })
    })
    .build()
    .run();
}
