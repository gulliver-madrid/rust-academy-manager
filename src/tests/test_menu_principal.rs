#[cfg(test)] 
use crate::{
    menus::MenuPrincipal,
    menus::OpcionMenuPrincipal,
    menus::ITEMS_MENU_DATA_MENU_PRINCIPAL,
    tests::fixtures::crear_control,
    tests::fixtures::{choice_to_string, crear_application_con_persistencia_vacia},
    tests::mock_console::MockConsole,
};

#[test]
fn salir_desde_menu_principal() {
    let application = crear_application_con_persistencia_vacia();
    let provided_input =
        choice_to_string(OpcionMenuPrincipal::Salir, ITEMS_MENU_DATA_MENU_PRINCIPAL)
            .unwrap();
    let mock_console = MockConsole::new();
    mock_console
        .provided_inputs
        .borrow_mut()
        .push(provided_input);
    let mut control = crear_control(mock_console, application);
    let mut menu = MenuPrincipal::new(&mut control);
    menu.abrir_menu();
    assert_eq!(menu.raised_loop_limit(), false);
}
