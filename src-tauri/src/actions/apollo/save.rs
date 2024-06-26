use async_std::fs;
use json_objects_to_csv::Json2Csv;
use serde_json::Value;
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;
use tauri::AppHandle;
use crate::actions::controllers::Response;

#[tauri::command]
pub async fn save_file(_: AppHandle, ext: String, path: String, content: Value) -> Response<Option<()>> {
  let path = format!("{path}.{ext}");


  if ext == "csv" {
    let flattener = Flattener::new()
      .set_key_separator(".")
      .set_array_formatting(ArrayFormatting::Plain)
      .set_preserve_empty_arrays(true)
      .set_preserve_empty_objects(true);

    let mut output = Vec::<u8>::new();

    let csv_writer = csv::WriterBuilder::new()
      .delimiter(b',')
      .from_writer(&mut output);

    match Json2Csv::new(flattener).convert_from_array(content.as_array().unwrap(), csv_writer) {
      Ok(_) => {},
      Err(e) => return Response::fail_none(None)
    };

    match fs::write(path, &output).await {
      Ok(_) => return Response::ok_none(),
      Err(e) => return Response::fail_none(None)
    };

  } else {
    let content = content.to_string();
    let content = content.as_bytes();

    match fs::write(path, content).await {
      Ok(_) => {return Response::ok_none()},
      Err(e) => return Response::fail_none(None)
    };
  }
}