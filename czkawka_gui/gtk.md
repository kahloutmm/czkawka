sd -s "window_progress.resize(1, 1);" "" czkawka_gui/src/*.rs # resize is missing in GTK


# TODO convert also ui files to GTK 4
gtk4-builder-tool simplify --3to4 czkawka_gui/ui/settings.glade > czkawka_gui/ui/settings2.glade
gtk4-builder-tool simplify --3to4 czkawka_gui/ui/about_dialog.glade > czkawka_gui/ui/about_dialog2.glade
gtk4-builder-tool simplify --3to4 czkawka_gui/ui/main_window.glade > czkawka_gui/ui/main_window2.glade
gtk4-builder-tool simplify --3to4 czkawka_gui/ui/popover_right_click.glade > czkawka_gui/ui/popover_right_click2.glade
gtk4-builder-tool simplify --3to4 czkawka_gui/ui/popover_select.glade > czkawka_gui/ui/popover_select2.glade
gtk4-builder-tool simplify --3to4 czkawka_gui/ui/progress.glade > czkawka_gui/ui/progress2.glade
mv czkawka_gui/ui/settings2.glade czkawka_gui/ui/settings.glade
mv czkawka_gui/ui/about_dialog2.glade czkawka_gui/ui/about_dialog.glade
mv czkawka_gui/ui/main_window2.glade czkawka_gui/ui/main_window.glade
mv czkawka_gui/ui/popover_right_click2.glade czkawka_gui/ui/popover_right_click.glade
mv czkawka_gui/ui/popover_select2.glade czkawka_gui/ui/popover_select.glade
mv czkawka_gui/ui/progress2.glade czkawka_gui/ui/progress.glade

sd -s "ButtonBox" "Box" czkawka_gui/ui/*.glade

sd "\n[ ]+<property name=\"shadow-type\">[a-z]+</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"caps-lock-warning\">False</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"resize-mode\">queue</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"window-position\">queue</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"window-position\">center</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"type-hint\">queue</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"type-hint\">dialog</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"layout-style\">queue</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"layout-style\">end</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"action_area\">queue</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"layout-style\">queue</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"layout-style\">end</property>" "" czkawka_gui/ui/*.glade
sd "\n[ ]+<property name=\"gravity\">center</property>" "" czkawka_gui/ui/*.glade

sd -s "<child internal-child=\"vbox\">" "<child>" czkawka_gui/ui/*.glade
sd -s "<child internal-child=\"action_area\">" "<child>" czkawka_gui/ui/*.glade


sd -s "show_all" "show" czkawka_gui/src/*.rs

sd -s "Some(Box::new(select_function_duplicates))" "select_function_duplicates" czkawka_gui/src/*.rs
sd -s "Some(Box::new(select_function_similar_images))" "select_function_similar_images" czkawka_gui/src/*.rs
sd -s "Some(Box::new(select_function_similar_videos))" "select_function_similar_videos" czkawka_gui/src/*.rs
sd -s "Some(Box::new(select_function_same_music))" "select_function_same_music" czkawka_gui/src/*.rs


sd -s "scrolled_window.add(&tree_view)" "scrolled_window.set_child(Some(&tree_view))" czkawka_gui/src/*.rs
sd -s ".add(" ".append(" czkawka_gui/src/*.rs

sd -s "gtk::" "gtk4::" czkawka_gui/src/*.rs
sd -s "gdk::" "gdk4::" czkawka_gui/src/*.rs
sd -s "use gtk4::prelude::*;" "use gtk4::prelude::*;use gtk4::Inhibit;" czkawka_gui/src/*.rs


sd -s ".value(" ".get(" czkawka_gui/src/*.rs
sd -s "scale_similarity_similar_images.get()" "scale_similarity_similar_images.value()" czkawka_gui/src/*.rs
sd -s "scale_similarity_similar_videos.get()" "scale_similarity_similar_videos.value()" czkawka_gui/src/*.rs

sd -s ".buffer().unwrap()" ".buffer()" czkawka_gui/src/*.rs

sd -s ".path(&iter).unwrap()" ".path(&iter)" czkawka_gui/src/*.rs
sd -s "model.path(&current_iter).unwrap()" "model.path(&current_iter)" czkawka_gui/src/*.rs
sd -s "model.path(&next_iter).unwrap()" "model.path(&next_iter)" czkawka_gui/src/*.rs

sd -s "window_main.set_title(\"Czkawka\")" "window_main.set_title(Some(\"Czkawka\"))" czkawka_gui/src/*.rs
sd -s "window_progress.set_title(\"Czkawka\")" "window_progress.set_title(Some(\"Czkawka\"))" czkawka_gui/src/*.rs

# NOT WORKING # sd "connect_delete_event\(([^,^\n]+), _" "connect_delete_event(${1" czkawka_gui/src/*.rs
sd -s "connect_delete_event(|dialog, _|" "connect_close_request(|dialog|" czkawka_gui/src/*.rs
sd -s "connect_delete_event(move |window, _|" "connect_close_request(move |window|" czkawka_gui/src/*.rs
sd -s "connect_delete_event(move |_, _|" "connect_close_request(move |_|" czkawka_gui/src/*.rs

# GTK BIN not exists, probably widget or button is better
sd -s "{Bin," "{" czkawka_gui/src/*.rs
sd -s "gtk4::Bin" "gtk4::Button" czkawka_gui/src/*.rs
sd -s ".upcast::<Bin>()" "" czkawka_gui/src/*.rs

# Checkbuttons set_label have now "Some"
sd -s 'label(&fl!("music_title_checkbox")' 'label(Some(&fl!("music_title_checkbox"))' czkawka_gui/src/*.rs  czkawka_gui/src/*.rs
sd -s 'label(&fl!("music_artist_checkbox")' 'label(Some(&fl!("music_artist_checkbox"))' czkawka_gui/src/*.rs  czkawka_gui/src/*.rs
sd -s 'label(&fl!("music_album_title_checkbox")' 'label(Some(&fl!("music_album_title_checkbox"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("music_album_artist_checkbox")' 'label(Some(&fl!("music_album_artist_checkbox"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("music_year_checkbox")' 'label(Some(&fl!("music_year_checkbox"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("music_comparison_checkbox")' 'label(Some(&fl!("music_comparison_checkbox"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("check_button_general_same_size")' 'label(Some(&fl!("check_button_general_same_size"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("check_button_general_same_size")' 'label(Some(&fl!("check_button_general_same_size"))' czkawka_gui/src/*.rs

sd -s 'label(&fl!("settings_save_at_exit_button")' 'label(Some(&fl!("settings_save_at_exit_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_load_at_start_button")' 'label(Some(&fl!("settings_load_at_start_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_confirm_deletion_button")' 'label(Some(&fl!("settings_confirm_deletion_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_confirm_link_button")' 'label(Some(&fl!("settings_confirm_link_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_confirm_group_deletion_button")' 'label(Some(&fl!("settings_confirm_group_deletion_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_show_text_view_button")' 'label(Some(&fl!("settings_show_text_view_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_use_cache_button")' 'label(Some(&fl!("settings_use_cache_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_use_trash_button")' 'label(Some(&fl!("settings_use_trash_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_duplicates_hide_hard_link_button")' 'label(Some(&fl!("settings_duplicates_hide_hard_link_button"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_multiple_image_preview_checkbutton")' 'label(Some(&fl!("settings_multiple_image_preview_checkbutton"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_multiple_delete_outdated_cache_checkbutton")' 'label(Some(&fl!("settings_multiple_delete_outdated_cache_checkbutton"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_duplicates_prehash_checkbutton")' 'label(Some(&fl!("settings_duplicates_prehash_checkbutton"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_multiple_image_preview_checkbutton")' 'label(Some(&fl!("settings_multiple_image_preview_checkbutton"))' czkawka_gui/src/*.rs
sd -s 'label(&fl!("settings_multiple_delete_outdated_cache_checkbutton")' 'label(Some(&fl!("settings_multiple_delete_outdated_cache_checkbutton"))' czkawka_gui/src/*.rs

sd -s 'label(&fl!("upper_recursive_button")' 'label(Some(&fl!("upper_recursive_button"))' czkawka_gui/src/*.rs

# TODO Menubutton don't have connect_clicked
sd -s "buttons_select.connect_clicked" "buttons_select.connect_activate" czkawka_gui/src/*.rs


# Errors Manual

`buffer.text()` -> `` returns valid text, so `match` can be removed around such statements


# TODO
get_custom_label_from_label_with_image - takes bin, but should also takes widget or menubutton
missing resize, what should I do?
- change upper window from paned to normal box, because paned behave strange

