use crate::{repositories::read::domain as domain_read_repository, repositories::write::domain as domain_write_repository};
use crate::commands::domains::DomainAssignCommand;
use crate::validators;
use crate::validators::domain_assign::Info;

pub fn update() -> Result<(), String> {
    println!("Applying some update action...");
    //if category matches, call private
    handle_assign()
}

fn handle_assign() -> Result<(), String> {
    let command = DomainAssignCommand {
        domain_id: 1,
        group_id: 1,
    };
    let info = Info {
        assigned_as_personal_in: domain_read_repository::get_groups_where_domain_is_personal(),
        assigned_as_main_in: domain_read_repository::get_groups_where_domain_is_main(),
    };
    validators::domain_assign::validate(&command, &info).map(|_|
        domain_write_repository::assign_domain_to_group(command.domain_id, command.group_id)
    )
}