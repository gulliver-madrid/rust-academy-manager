pub struct ItemMenu<'a, OpcionMenu> {
    pub texto: &'a str,
    pub opcion: OpcionMenu,
}

pub type TextoOpcion = &'static str;

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

pub fn crear_texto_opciones<T>(items_menu: &Vec<ItemMenu<T>>) -> String {
    // Crea el texto correspondiente a la lista de ItemMenu recibidos
    String::from("Elige una opción:\n")
        + &items_menu
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{} - {}", i + 1, item.texto))
            .collect::<Vec<String>>()
            .join("\n")
}
pub fn crear_items_menu<OpcionMenu, const N: usize>(
    items_menu_data: [(&'static OpcionMenu, TextoOpcion); N],
) -> Vec<ItemMenu<&'static OpcionMenu>> {
    // Genera un vector de ItemMenu a partir de un array de tuplas
    let mut items_menu = Vec::new();
    for (opcion, texto) in items_menu_data {
        items_menu.push(ItemMenu { texto, opcion });
    }
    items_menu
}
