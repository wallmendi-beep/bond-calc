#[tauri::command]
fn get_chatbot_response(message: String) -> String {
    let normalized_message = message.to_lowercase();

    if normalized_message.contains("안녕") || normalized_message.contains("hi") {
        "안녕하세요! Bond-Bot 입니다. 무엇을 도와드릴까요?".to_string()
    } else if normalized_message.contains("모드") {
        "'맞춤형 상세 계산 (A)'와 '빠른 대시보드 (B)' 두 가지 모드가 있습니다. A 모드는 복잡한 계산에, B 모드는 빠른 계산에 사용됩니다.".to_string()
    } else if normalized_message.contains("지분") {
        "공유 지분 계산은 '맞춤형 상세 계산 (A)' 모드에서 '공유 (지분)' 옵션을 선택하거나, '빠른 대시보드 (B)' 모드의 지분 입력란에 분수 형태로 직접 입력하여 계산할 수 있습니다.".to_string()
    } else if normalized_message.contains("부담부증여") {
        "부담부증여는 '맞춤형 상세 계산 (A)' 모드에서 등기 원인을 '부담부증여'로 선택하거나, '빠른 대시보드 (B)' 모드에서 '채무액'을 입력하면 자동으로 계산됩니다.".to_string()
    } else {
        "죄송합니다, 이해하지 못했어요. '모드', '지분', '부담부증여' 등과 같은 키워드로 질문해 보세요.".to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_chatbot_response])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
