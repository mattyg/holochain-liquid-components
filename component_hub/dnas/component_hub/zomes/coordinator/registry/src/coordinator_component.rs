use hdk::prelude::*;
use registry_integrity::*;

#[hdk_extern]
pub fn search_coordinator_components(name: String) -> ExternResult<Vec<ActionHash>> {
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

    let action_hashes = links
        .into_iter()
        .map(|l| ActionHash::from(l.target))
        .collect();

    Ok(action_hashes)
}

fn search_prefix_path(name: String) -> ExternResult<TypedPath> {
    let prefix: String = name.to_lowercase().chars().take(3).collect();

    Path::from(format!("all_components.{}", prefix)).typed(LinkTypes::SearchIndex)
}

#[hdk_extern]
pub fn create_coordinator_component(
    coordinator_component: CoordinatorComponent,
) -> ExternResult<Record> {
    let coordinator_component_hash = create_entry(
        &EntryTypes::CoordinatorComponent(coordinator_component.clone()),
    )?;
    let record = get(coordinator_component_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created CoordinatorComponent"))
            ),
        )?;


    // Prefix search path by Coordinator Component name
    let search_path = search_prefix_path(coordinator_component.name.clone())?;
    search_path.ensure()?;
    create_link(
        search_path.path_entry_hash()?,
        coordinator_component_hash.clone(),
        LinkTypes::SearchIndexToComponent,
        LinkTag::new(coordinator_component.name.to_lowercase().as_bytes().to_vec()),
    )?;

    // All Coordinator Compnents Links
    let path = Path::from("all_coordinator_components");
    create_link(
        path.path_entry_hash()?,
        coordinator_component_hash.clone(),
        LinkTypes::AllCoordinatorComponents,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_coordinator_component(
    original_coordinator_component_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_coordinator_component_hash.clone(),
        LinkTypes::CoordinatorComponentUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_coordinator_component_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_coordinator_component_hash.clone(),
    };
    get(latest_coordinator_component_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCoordinatorComponentInput {
    pub original_coordinator_component_hash: ActionHash,
    pub previous_coordinator_component_hash: ActionHash,
    pub updated_coordinator_component: CoordinatorComponent,
}
#[hdk_extern]
pub fn update_coordinator_component(
    input: UpdateCoordinatorComponentInput,
) -> ExternResult<Record> {
    let updated_coordinator_component_hash = update_entry(
        input.previous_coordinator_component_hash.clone(),
        &input.updated_coordinator_component,
    )?;
    create_link(
        input.original_coordinator_component_hash.clone(),
        updated_coordinator_component_hash.clone(),
        LinkTypes::CoordinatorComponentUpdates,
        (),
    )?;
    let record = get(updated_coordinator_component_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated CoordinatorComponent"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_coordinator_component(
    original_coordinator_component_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_coordinator_component_hash)
}
