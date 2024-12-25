pub mod commands;
pub mod queries;
pub mod errors;
pub mod response;

use commands::SpendingEditCommandHandler;

use self::commands::CreateSpendingCommandHandler;
use self::commands::SpendingRemoveCommandHandler;
use self::queries::SpendingsListQueryHandler;

pub use self::response::Response;

pub struct State {
    pub create_spending_command_handler: CreateSpendingCommandHandler,
    pub spending_remove_command_handler: SpendingRemoveCommandHandler,
    pub spending_edit_command_handler: SpendingEditCommandHandler,
    pub spendings_list_query_handler: SpendingsListQueryHandler,
}

impl State {
    pub fn new(
        create_spending_command_handler: CreateSpendingCommandHandler,
        spending_remove_command_handler: SpendingRemoveCommandHandler,
        spending_edit_command_handler: SpendingEditCommandHandler,
        spendings_list_query_handler: SpendingsListQueryHandler,
    ) -> State {
        State{
            create_spending_command_handler,
            spending_remove_command_handler,
            spending_edit_command_handler,
            spendings_list_query_handler,
        }
    }
}
