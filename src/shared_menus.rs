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
    let num_opcion: usize = eleccion.parse().ok()?;
    let &item_elegido = &items_menu.get(num_opcion - 1)?;
    return Some(&item_elegido.opcion);
}
