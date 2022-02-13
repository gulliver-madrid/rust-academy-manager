#[cfg(test)]
use crate::{
    menus::MenuPrincipal,
    menus::OpcionMenuAsignaturas,
    menus::OpcionMenuPrincipal,
    menus::ITEMS_MENU_DATA_MENU_ASIGNATURAS,
    menus::ITEMS_MENU_DATA_MENU_PRINCIPAL,
    tests::{
        fixtures::{
            choice_to_string, crear_application_con_persistencia_vacia,
            crear_control,
        },
        mock_console::MockConsole,
    },
};

#[test]
fn entrar_a_asignaturas_y_salir() {
    let application = crear_application_con_persistencia_vacia();
    let inputs = [
        choice_to_string(
            OpcionMenuPrincipal::Asignaturas,
            ITEMS_MENU_DATA_MENU_PRINCIPAL,
        ),
        choice_to_string(
            OpcionMenuAsignaturas::Volver,
            ITEMS_MENU_DATA_MENU_ASIGNATURAS,
        ),
        choice_to_string(OpcionMenuPrincipal::Salir, ITEMS_MENU_DATA_MENU_PRINCIPAL),
    ];

    let mock_console = MockConsole::new();
    {
        let mut provided_inputs = mock_console.provided_inputs.borrow_mut();
        for input in inputs {
            provided_inputs.push(input.unwrap());
        }
    }

    let mut control = crear_control(mock_console, application);
    let mut menu = MenuPrincipal::new(&mut control);
    menu.abrir_menu();
    assert_eq!(menu.raised_loop_limit(), false);
}
