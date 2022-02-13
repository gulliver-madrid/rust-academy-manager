mod counter;
mod menu_asignar_profesor;
mod menu_asignaturas;
mod menu_principal;
mod menu_profesores;
pub mod shared;

pub use menu_asignaturas::{
    OpcionMenuAsignaturas, ITEMS_MENU_DATA_MENU_ASIGNATURAS,
};
pub use menu_principal::Opcion as OpcionMenuPrincipal;
pub use menu_principal::{
    MenuPrincipal, ITEMS_MENU_DATA as ITEMS_MENU_DATA_MENU_PRINCIPAL,
};
