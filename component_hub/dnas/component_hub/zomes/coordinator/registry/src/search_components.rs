use hdk::prelude::*;
use registry_integrity::*;

#[hdk_extern]
pub fn search_coordinator_components(name: String) -> ExternResult<Vec<Record>> {
    if name.len() < 3 {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Cannot search with a prefix less than 3 characters".into(),
        )));
    }

    let prefix_path = search_prefix_path(name.clone())?;
    let links = get_links(
        prefix_path.path_entry_hash()?,
        LinkTypes::SearchIndexToComponent,
        Some(LinkTag::new(
            name.to_lowercase().as_bytes().to_vec(),
        )),
    )?;

    let records: Vec<Record> = links
        .into_iter()
        .filter_map(|l| get(ActionHash::from(l.target), GetOptions::default()).ok())
        .filter_map(|x| x)
        .collect();

    Ok(records)
}

fn search_prefix_path(name: String) -> ExternResult<TypedPath> {
    let prefix: String = name.to_lowercase().chars().take(3).collect();

    Path::from(format!("all_components.{}", prefix)).typed(LinkTypes::SearchIndex)
}
