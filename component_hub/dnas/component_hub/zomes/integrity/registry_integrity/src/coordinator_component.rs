use hdi::prelude::*;
use devhub_types::{
    DevHubResponse, Entity, EntityResponse, FilterInput,
    dnarepo_entry_types::{
	    ProfileEntry,
	    DnaEntry,
	    DnaVersionEntry, DnaVersionPackage,
	    ZomeEntry,
	    ZomeVersionEntry,
    }
};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct CoordinatorComponent {
    // Human readable name
    pub name: String,

    // Raw bytes of webcomponent
    pub webcomponent: Vec<u8>,

    // The DNAs on devhub needed for this webcomponent to operate
    pub devhub_dna_hash: Vec<EntryHash>,
}


pub fn validate_create_coordinator_component(
    _action: EntryCreationAction,
    _coordinator_component: CoordinatorComponent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_coordinator_component(
    _action: Update,
    _coordinator_component: CoordinatorComponent,
    _original_action: EntryCreationAction,
    _original_coordinator_component: CoordinatorComponent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_coordinator_component(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_coordinator_component: CoordinatorComponent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_coordinator_component_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordinator_component: crate::CoordinatorComponent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordinator_component: crate::CoordinatorComponent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_coordinator_component_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("CoordinatorComponentUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_coordinator_components(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordinator_component: crate::CoordinatorComponent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_coordinator_components(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllCoordinatorComponents links cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_search_index(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Valid
    )
}

pub fn validate_delete_link_search_index(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("SearchIndex links cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_search_index_to_component(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let entry = must_get_entry(EntryHash::from(target_address))?;
    CoordinatorComponent::try_from(entry).map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    
    Ok(
        ValidateCallbackResult::Valid
    )
}

pub fn validate_delete_link_search_index_to_component(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    match original_action.author.eq(&action.author) {
        true => Ok(
            ValidateCallbackResult::Valid
        ),
        false => Ok(
            ValidateCallbackResult::Invalid(
                String::from("AllCoordinatorComponents links can only be deleted by the author"),
            ),
        )
    }
}
