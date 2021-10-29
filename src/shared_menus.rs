pub struct ItemMenu<'a, OpcionMenu> {
    pub texto: &'a str,
    pub opcion: OpcionMenu,
}

pub fn extraer_opcion<'a, OpcionMenu>(
    eleccion: String,
    items_menu: &'a Vec<ItemMenu<OpcionMenu>>,
) -> Option<&'a OpcionMenu> {
    // Recibe una string que idealmente contendra un numero
    // y a partir de ese numero devolveremos la OpcionMenu
    // correspondiente, segun el orden del vector de objetos ItemMenu
    let posible: Option<usize> = eleccion.parse().ok();
    if let Some(num_opcion) = posible {
        let posible = items_menu.get(num_opcion - 1);
        if let Some(item_elegido) = posible {
            return Some(&item_elegido.opcion);
        }
    }
    return None;
}
