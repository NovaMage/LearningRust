use crate::commands::domains::DomainAssignCommand;

pub struct Info {
    pub assigned_as_personal_in: Vec<u32>,
    pub assigned_as_main_in: Vec<u32>,
}

pub fn validate(command: &DomainAssignCommand, info: &Info) -> Result<(), String> {
    println!("Validating command {:#?}", command);

    if command.domain_id == 0 {
        return Err("Domain ID is required".to_string());
    }
    if command.group_id == 0 {
        return Err("Group ID is required".to_string());
    }
    if info.assigned_as_personal_in.contains(&command.group_id) {
        return Err("Domain is already assigned as personal".to_string());
    }
    if info.assigned_as_main_in.contains(&command.group_id) {
        return Err("Domain is already assigned as main".to_string());
    }
    Ok(())
}