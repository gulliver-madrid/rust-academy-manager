import re

optional_spaces = r"\s*"
menu_option_variant = r"MenuOption::[A-Z]\w*"
optional_part_of_key_pattern = r"(?:[.]\w+)"
key_pattern = fr"[\w]+{optional_part_of_key_pattern}?" 

text_pattern = fr'\({optional_spaces}{menu_option_variant},{optional_spaces}"({key_pattern})"[,)]'

# example: MenuOption::Teachers, "menu_options.teacher")
pattern_2: re.Pattern[str] = re.compile(text_pattern, re.ASCII)

def test_match_pattern_2_1():
    assert pattern_2.search('(MenuOption::ShowList, "subjects_menu_options.show_list"),')
def test_match_pattern_2_2():
    assert pattern_2.search('''(
        MenuOption::AssignTeacher,
        "subjects_menu_options.assign_teacher",
    ),''')
def test_no_match_pattern_2():
    assert not pattern_2.search('("subjects_menu_options.assign_teacher"')