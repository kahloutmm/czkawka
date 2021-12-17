use crate::fl;
use crate::help_combo_box::IMAGES_HASH_SIZE_COMBO_BOX;
use crate::help_functions::get_all_children;
use czkawka_core::similar_images::{get_string_from_similarity, Similarity, SIMILAR_VALUES};
use gtk4::prelude::*;
use gtk4::{EventControllerKey, TreeView};
use gtk4::{GestureClick, Inhibit};

use crate::notebook_enums::{NotebookMainEnum, NUMBER_OF_NOTEBOOK_MAIN_TABS};

#[derive(Clone)]
pub struct GuiMainNotebook {
    pub notebook_main: gtk4::Notebook,

    pub scrolled_window_duplicate_finder: gtk4::ScrolledWindow,
    pub scrolled_window_empty_folder_finder: gtk4::ScrolledWindow,
    pub scrolled_window_empty_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_temporary_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_big_files_finder: gtk4::ScrolledWindow,
    pub scrolled_window_similar_images_finder: gtk4::ScrolledWindow,
    pub scrolled_window_similar_videos_finder: gtk4::ScrolledWindow,
    pub scrolled_window_same_music_finder: gtk4::ScrolledWindow,
    pub scrolled_window_invalid_symlinks: gtk4::ScrolledWindow,
    pub scrolled_window_broken_files: gtk4::ScrolledWindow,

    pub tree_view_duplicate_finder: gtk4::TreeView,
    pub tree_view_empty_folder_finder: gtk4::TreeView,
    pub tree_view_empty_files_finder: gtk4::TreeView,
    pub tree_view_temporary_files_finder: gtk4::TreeView,
    pub tree_view_big_files_finder: gtk4::TreeView,
    pub tree_view_similar_images_finder: gtk4::TreeView,
    pub tree_view_similar_videos_finder: gtk4::TreeView,
    pub tree_view_same_music_finder: gtk4::TreeView,
    pub tree_view_invalid_symlinks: gtk4::TreeView,
    pub tree_view_broken_files: gtk4::TreeView,

    // TODO, in GTK4 this can be changed to e.g. add_controller which is not 100% compatible with this  - https://discourse.gnome.org/t/how-to-convert-code-to-use-eventcontrollerkey/8198/2
    pub evk_tree_view_duplicate_finder: gtk4::EventControllerKey,
    pub evk_tree_view_empty_folder_finder: gtk4::EventControllerKey,
    pub evk_tree_view_empty_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_temporary_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_big_files_finder: gtk4::EventControllerKey,
    pub evk_tree_view_similar_images_finder: gtk4::EventControllerKey,
    pub evk_tree_view_similar_videos_finder: gtk4::EventControllerKey,
    pub evk_tree_view_same_music_finder: gtk4::EventControllerKey,
    pub evk_tree_view_invalid_symlinks: gtk4::EventControllerKey,
    pub evk_tree_view_broken_files: gtk4::EventControllerKey,

    pub gc_tree_view_duplicate_finder: gtk4::GestureClick,
    pub gc_tree_view_empty_folder_finder: gtk4::GestureClick,
    pub gc_tree_view_empty_files_finder: gtk4::GestureClick,
    pub gc_tree_view_temporary_files_finder: gtk4::GestureClick,
    pub gc_tree_view_big_files_finder: gtk4::GestureClick,
    pub gc_tree_view_similar_images_finder: gtk4::GestureClick,
    pub gc_tree_view_similar_videos_finder: gtk4::GestureClick,
    pub gc_tree_view_same_music_finder: gtk4::GestureClick,
    pub gc_tree_view_invalid_symlinks: gtk4::GestureClick,
    pub gc_tree_view_broken_files: gtk4::GestureClick,

    // General

    // Duplicate
    pub combo_box_duplicate_check_method: gtk4::ComboBoxText,
    pub combo_box_duplicate_hash_type: gtk4::ComboBoxText,
    pub label_duplicate_check_method: gtk4::Label,
    pub label_duplicate_hash_type: gtk4::Label,

    pub image_preview_duplicates: gtk4::Image,

    // Big file
    pub label_big_shown_files: gtk4::Label,
    pub entry_big_files_number: gtk4::Entry,

    // Similar Images
    pub scale_similarity_similar_images: gtk4::Scale,

    pub label_image_resize_algorithm: gtk4::Label,
    pub label_image_hash_type: gtk4::Label,
    pub label_image_hash_size: gtk4::Label,

    pub combo_box_image_resize_algorithm: gtk4::ComboBoxText,
    pub combo_box_image_hash_algorithm: gtk4::ComboBoxText,
    pub combo_box_image_hash_size: gtk4::ComboBoxText,

    pub check_button_image_ignore_same_size: gtk4::CheckButton,
    pub check_button_video_ignore_same_size: gtk4::CheckButton,

    pub label_image_similarity: gtk4::Label,
    pub label_image_similarity_max: gtk4::Label,

    pub image_preview_similar_images: gtk4::Image,
    pub label_similar_images_minimal_similarity: gtk4::Label,

    // Video
    pub label_video_similarity: gtk4::Label,
    pub label_video_similarity_min: gtk4::Label,
    pub label_video_similarity_max: gtk4::Label,

    pub scale_similarity_similar_videos: gtk4::Scale,

    // Music
    pub check_button_music_title: gtk4::CheckButton,
    pub check_button_music_artist: gtk4::CheckButton,
    pub check_button_music_album_title: gtk4::CheckButton,
    pub check_button_music_album_artist: gtk4::CheckButton,
    pub check_button_music_year: gtk4::CheckButton,
    pub check_button_music_approximate_comparison: gtk4::CheckButton,
}

impl GuiMainNotebook {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_main: gtk4::Notebook = builder.object("notebook_main").unwrap();

        let scrolled_window_duplicate_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_duplicate_finder").unwrap();
        let scrolled_window_empty_folder_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_empty_folder_finder").unwrap();
        let scrolled_window_empty_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_empty_files_finder").unwrap();
        let scrolled_window_temporary_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_temporary_files_finder").unwrap();
        let scrolled_window_big_files_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_big_files_finder").unwrap();
        let scrolled_window_similar_images_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_similar_images_finder").unwrap();
        let scrolled_window_similar_videos_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_similar_videos_finder").unwrap();
        let scrolled_window_same_music_finder: gtk4::ScrolledWindow = builder.object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk4::ScrolledWindow = builder.object("scrolled_window_invalid_symlinks").unwrap();
        let scrolled_window_broken_files: gtk4::ScrolledWindow = builder.object("scrolled_window_broken_files").unwrap();

        let tree_view_duplicate_finder: gtk4::TreeView = TreeView::new();
        tree_view_duplicate_finder.set_widget_name("PIERD");
        let tree_view_empty_folder_finder: gtk4::TreeView = TreeView::new();
        let tree_view_empty_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_temporary_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_big_files_finder: gtk4::TreeView = TreeView::new();
        let tree_view_similar_images_finder: gtk4::TreeView = TreeView::new();
        let tree_view_similar_videos_finder: gtk4::TreeView = TreeView::new();
        let tree_view_same_music_finder: gtk4::TreeView = TreeView::new();
        let tree_view_invalid_symlinks: gtk4::TreeView = TreeView::new();
        let tree_view_broken_files: gtk4::TreeView = TreeView::new();

        // let evk_tree_view_duplicate_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_duplicate_finder);
        // let evk_tree_view_empty_folder_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_empty_folder_finder);
        // let evk_tree_view_empty_files_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_empty_files_finder);
        // let evk_tree_view_temporary_files_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_temporary_files_finder);
        // let evk_tree_view_big_files_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_big_files_finder);
        // let evk_tree_view_similar_images_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_similar_images_finder);
        // let evk_tree_view_similar_videos_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_similar_videos_finder);
        // let evk_tree_view_same_music_finder: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_same_music_finder);
        // let evk_tree_view_invalid_symlinks: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_invalid_symlinks);
        // let evk_tree_view_broken_files: gtk4::EventControllerKey = EventControllerKey::new(&tree_view_broken_files);

        // TODO GTK 4
        let evk_tree_view_duplicate_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_duplicate_finder.add_controller(&evk_tree_view_duplicate_finder);
        let evk_tree_view_empty_folder_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_empty_folder_finder.add_controller(&evk_tree_view_empty_folder_finder);
        let evk_tree_view_empty_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_empty_files_finder.add_controller(&evk_tree_view_empty_files_finder);
        let evk_tree_view_temporary_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_temporary_files_finder.add_controller(&evk_tree_view_temporary_files_finder);
        let evk_tree_view_big_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_big_files_finder.add_controller(&evk_tree_view_big_files_finder);
        let evk_tree_view_similar_images_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_similar_images_finder.add_controller(&evk_tree_view_similar_images_finder);
        let evk_tree_view_similar_videos_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_similar_videos_finder.add_controller(&evk_tree_view_similar_videos_finder);
        let evk_tree_view_same_music_finder: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_same_music_finder.add_controller(&evk_tree_view_same_music_finder);
        let evk_tree_view_invalid_symlinks: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_invalid_symlinks.add_controller(&evk_tree_view_invalid_symlinks);
        let evk_tree_view_broken_files: gtk4::EventControllerKey = EventControllerKey::new();
        tree_view_broken_files.add_controller(&evk_tree_view_broken_files);

        let gc_tree_view_duplicate_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_duplicate_finder.add_controller(&gc_tree_view_duplicate_finder);
        let gc_tree_view_empty_folder_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_empty_folder_finder.add_controller(&gc_tree_view_empty_folder_finder);
        let gc_tree_view_empty_files_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_empty_files_finder.add_controller(&gc_tree_view_empty_files_finder);
        let gc_tree_view_temporary_files_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_temporary_files_finder.add_controller(&gc_tree_view_temporary_files_finder);
        let gc_tree_view_big_files_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_big_files_finder.add_controller(&gc_tree_view_big_files_finder);
        let gc_tree_view_similar_images_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_similar_images_finder.add_controller(&gc_tree_view_similar_images_finder);
        let gc_tree_view_similar_videos_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_similar_videos_finder.add_controller(&gc_tree_view_similar_videos_finder);
        let gc_tree_view_same_music_finder: gtk4::GestureClick = GestureClick::new();
        tree_view_same_music_finder.add_controller(&gc_tree_view_same_music_finder);
        let gc_tree_view_invalid_symlinks: gtk4::GestureClick = GestureClick::new();
        tree_view_invalid_symlinks.add_controller(&gc_tree_view_invalid_symlinks);
        let gc_tree_view_broken_files: gtk4::GestureClick = GestureClick::new();
        tree_view_broken_files.add_controller(&gc_tree_view_broken_files);

        let combo_box_duplicate_check_method: gtk4::ComboBoxText = builder.object("combo_box_duplicate_check_method").unwrap();
        let combo_box_duplicate_hash_type: gtk4::ComboBoxText = builder.object("combo_box_duplicate_hash_type").unwrap();

        let entry_big_files_number: gtk4::Entry = builder.object("entry_big_files_number").unwrap();

        //// Check Buttons
        let check_button_music_title: gtk4::CheckButton = builder.object("check_button_music_title").unwrap();
        let check_button_music_artist: gtk4::CheckButton = builder.object("check_button_music_artist").unwrap();
        let check_button_music_album_title: gtk4::CheckButton = builder.object("check_button_music_album_title").unwrap();
        let check_button_music_album_artist: gtk4::CheckButton = builder.object("check_button_music_album_artist").unwrap();
        let check_button_music_year: gtk4::CheckButton = builder.object("check_button_music_year").unwrap();
        let check_button_music_approximate_comparison: gtk4::CheckButton = builder.object("check_button_music_approximate_comparison").unwrap();

        //// Radio Buttons

        let scale_similarity_similar_images: gtk4::Scale = builder.object("scale_similarity_similar_images").unwrap();
        let scale_similarity_similar_videos: gtk4::Scale = builder.object("scale_similarity_similar_videos").unwrap();

        let combo_box_image_resize_algorithm: gtk4::ComboBoxText = builder.object("combo_box_image_resize_algorithm").unwrap();
        let combo_box_image_hash_algorithm: gtk4::ComboBoxText = builder.object("combo_box_image_hash_algorithm").unwrap();
        let combo_box_image_hash_size: gtk4::ComboBoxText = builder.object("combo_box_image_hash_size").unwrap();

        let check_button_image_ignore_same_size: gtk4::CheckButton = builder.object("check_button_image_ignore_same_size").unwrap();
        let check_button_video_ignore_same_size: gtk4::CheckButton = builder.object("check_button_video_ignore_same_size").unwrap();

        let label_similar_images_minimal_similarity: gtk4::Label = builder.object("label_similar_images_minimal_similarity").unwrap();

        let label_duplicate_check_method: gtk4::Label = builder.object("label_duplicate_check_method").unwrap();
        let label_duplicate_hash_type: gtk4::Label = builder.object("label_duplicate_hash_type").unwrap();
        let label_big_shown_files: gtk4::Label = builder.object("label_big_shown_files").unwrap();
        let label_image_resize_algorithm: gtk4::Label = builder.object("label_image_resize_algorithm").unwrap();
        let label_image_hash_type: gtk4::Label = builder.object("label_image_hash_type").unwrap();
        let label_image_hash_size: gtk4::Label = builder.object("label_image_hash_size").unwrap();
        let label_image_similarity: gtk4::Label = builder.object("label_image_similarity").unwrap();
        let label_image_similarity_max: gtk4::Label = builder.object("label_image_similarity_max").unwrap();
        let label_video_similarity: gtk4::Label = builder.object("label_video_similarity").unwrap();
        let label_video_similarity_min: gtk4::Label = builder.object("label_video_similarity_min").unwrap();
        let label_video_similarity_max: gtk4::Label = builder.object("label_video_similarity_max").unwrap();

        let image_preview_similar_images: gtk4::Image = builder.object("image_preview_similar_images").unwrap();
        let image_preview_duplicates: gtk4::Image = builder.object("image_preview_duplicates").unwrap();

        Self {
            notebook_main,
            scrolled_window_duplicate_finder,
            scrolled_window_empty_folder_finder,
            scrolled_window_empty_files_finder,
            scrolled_window_temporary_files_finder,
            scrolled_window_big_files_finder,
            scrolled_window_similar_images_finder,
            scrolled_window_similar_videos_finder,
            scrolled_window_same_music_finder,
            scrolled_window_invalid_symlinks,
            scrolled_window_broken_files,
            tree_view_duplicate_finder,
            tree_view_empty_folder_finder,
            tree_view_empty_files_finder,
            tree_view_temporary_files_finder,
            tree_view_big_files_finder,
            tree_view_similar_images_finder,
            tree_view_similar_videos_finder,
            tree_view_same_music_finder,
            tree_view_invalid_symlinks,
            tree_view_broken_files,
            evk_tree_view_duplicate_finder,
            evk_tree_view_empty_folder_finder,
            evk_tree_view_empty_files_finder,
            evk_tree_view_temporary_files_finder,
            evk_tree_view_big_files_finder,
            evk_tree_view_similar_images_finder,
            evk_tree_view_similar_videos_finder,
            evk_tree_view_same_music_finder,
            evk_tree_view_invalid_symlinks,
            evk_tree_view_broken_files,
            gc_tree_view_duplicate_finder,
            gc_tree_view_empty_folder_finder,
            gc_tree_view_empty_files_finder,
            gc_tree_view_temporary_files_finder,
            gc_tree_view_big_files_finder,
            gc_tree_view_similar_images_finder,
            gc_tree_view_similar_videos_finder,
            gc_tree_view_same_music_finder,
            gc_tree_view_invalid_symlinks,
            entry_big_files_number,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_album_title,
            check_button_music_album_artist,
            check_button_music_year,
            check_button_music_approximate_comparison,
            scale_similarity_similar_images,
            scale_similarity_similar_videos,
            check_button_image_ignore_same_size,
            label_similar_images_minimal_similarity,
            label_duplicate_check_method,
            label_duplicate_hash_type,
            combo_box_duplicate_check_method,
            label_big_shown_files,
            label_image_resize_algorithm,
            label_image_hash_type,
            label_image_hash_size,
            combo_box_image_resize_algorithm,
            combo_box_image_hash_algorithm,
            label_image_similarity,
            label_image_similarity_max,
            label_video_similarity,
            label_video_similarity_min,
            label_video_similarity_max,
            image_preview_similar_images,
            image_preview_duplicates,
            combo_box_duplicate_hash_type,
            combo_box_image_hash_size,
            check_button_video_ignore_same_size,
            gc_tree_view_broken_files,
        }
    }

    pub fn get_main_tree_views(&self) -> [TreeView; NUMBER_OF_NOTEBOOK_MAIN_TABS] {
        [
            self.tree_view_duplicate_finder.clone(),
            self.tree_view_empty_folder_finder.clone(),
            self.tree_view_big_files_finder.clone(),
            self.tree_view_empty_files_finder.clone(),
            self.tree_view_temporary_files_finder.clone(),
            self.tree_view_similar_images_finder.clone(),
            self.tree_view_similar_videos_finder.clone(),
            self.tree_view_same_music_finder.clone(),
            self.tree_view_invalid_symlinks.clone(),
            self.tree_view_broken_files.clone(),
        ]
    }

    pub fn update_language(&self) {
        self.check_button_music_title.set_label(Some(&fl!("music_title_checkbox")));
        self.check_button_music_artist.set_label(Some(&fl!("music_artist_checkbox")));
        self.check_button_music_album_title.set_label(Some(&fl!("music_album_title_checkbox")));
        self.check_button_music_album_artist.set_label(Some(&fl!("music_album_artist_checkbox")));
        self.check_button_music_year.set_label(Some(&fl!("music_year_checkbox")));
        self.check_button_music_approximate_comparison.set_label(Some(&fl!("music_comparison_checkbox")));

        self.check_button_music_approximate_comparison.set_tooltip_text(Some(&fl!("music_comparison_checkbox_tooltip")));

        self.label_duplicate_check_method.set_label(&fl!("main_label_check_method"));
        self.label_duplicate_hash_type.set_label(&fl!("main_label_hash_type"));
        self.label_big_shown_files.set_label(&fl!("main_label_shown_files"));
        self.label_image_resize_algorithm.set_label(&fl!("main_label_resize_algorithm"));
        self.label_image_hash_type.set_label(&fl!("main_label_hash_type"));
        self.label_image_hash_size.set_label(&fl!("main_label_hash_size"));
        self.label_image_similarity.set_label(&fl!("main_label_similarity"));
        self.label_image_similarity_max.set_label(&fl!("core_similarity_very_high"));
        self.label_video_similarity.set_label(&fl!("main_label_similarity"));
        self.label_video_similarity_min.set_label(&fl!("core_similarity_minimal"));
        self.label_video_similarity_max.set_label(&fl!("core_similarity_very_high"));

        self.label_duplicate_check_method.set_tooltip_text(Some(&fl!("duplicate_check_method_tooltip")));
        self.combo_box_duplicate_check_method.set_tooltip_text(Some(&fl!("duplicate_check_method_tooltip")));
        self.label_duplicate_hash_type.set_tooltip_text(Some(&fl!("duplicate_hash_type_tooltip")));
        self.combo_box_duplicate_hash_type.set_tooltip_text(Some(&fl!("duplicate_hash_type_tooltip")));

        self.combo_box_image_hash_size.set_tooltip_text(Some(&fl!("image_hash_size_tooltip")));
        self.label_image_hash_size.set_tooltip_text(Some(&fl!("image_hash_size_tooltip")));

        self.combo_box_image_resize_algorithm.set_tooltip_text(Some(&fl!("image_resize_filter_tooltip")));
        self.label_image_resize_algorithm.set_tooltip_text(Some(&fl!("image_resize_filter_tooltip")));

        self.combo_box_image_hash_algorithm.set_tooltip_text(Some(&fl!("image_hash_alg_tooltip")));
        self.label_image_hash_type.set_tooltip_text(Some(&fl!("image_hash_alg_tooltip")));

        self.check_button_image_ignore_same_size.set_tooltip_text(Some(&fl!("check_button_general_same_size_tooltip")));
        self.check_button_video_ignore_same_size.set_tooltip_text(Some(&fl!("check_button_general_same_size_tooltip")));
        self.check_button_image_ignore_same_size.set_label(Some(&fl!("check_button_general_same_size")));
        self.check_button_video_ignore_same_size.set_label(Some(&fl!("check_button_general_same_size")));

        {
            let hash_size_index = self.combo_box_image_hash_size.active().unwrap() as usize;
            let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index];
            match hash_size {
                8 => {
                    self.label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[0][5]), 8));
                }
                16 => {
                    self.label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[1][5]), 16));
                }
                32 => {
                    self.label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[2][5]), 32));
                }
                64 => {
                    self.label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[3][5]), 64));
                }
                _ => panic!(),
            }
        }

        // let vec_children: Vec<gtk4::Widget> = self.notebook_main.children();

        let vec_children: Vec<gtk4::Widget> = get_all_children(&self.notebook_main);
        let vec_children: Vec<gtk4::Widget> = get_all_children(&vec_children[1]);

        // Change name of main notebook tabs
        for (main_enum, fl_thing) in [
            (NotebookMainEnum::Duplicate as usize, fl!("main_notebook_duplicates")),
            (NotebookMainEnum::EmptyDirectories as usize, fl!("main_notebook_empty_directories")),
            (NotebookMainEnum::BigFiles as usize, fl!("main_notebook_big_files")),
            (NotebookMainEnum::EmptyFiles as usize, fl!("main_notebook_empty_files")),
            (NotebookMainEnum::Temporary as usize, fl!("main_notebook_temporary")),
            (NotebookMainEnum::SimilarImages as usize, fl!("main_notebook_similar_images")),
            (NotebookMainEnum::SimilarVideos as usize, fl!("main_notebook_similar_videos")),
            (NotebookMainEnum::SameMusic as usize, fl!("main_notebook_same_music")),
            (NotebookMainEnum::Symlinks as usize, fl!("main_notebook_symlinks")),
            (NotebookMainEnum::BrokenFiles as usize, fl!("main_notebook_broken_files")),
        ] {
            self.notebook_main.tab_label(&vec_children[main_enum]).unwrap().downcast::<gtk4::Label>().unwrap().set_text(&fl_thing);
        }

        // Change names of columns
        let names_of_columns = [
            vec![fl!("main_tree_view_column_file_name"), fl!("main_tree_view_column_path"), fl!("main_tree_view_column_modification")], // Duplicates
            vec![fl!("main_tree_view_column_folder_name"), fl!("main_tree_view_column_path"), fl!("main_tree_view_column_modification")], // Empty Folders
            vec![
                fl!("main_tree_view_column_size"),
                fl!("main_tree_view_column_file_name"),
                fl!("main_tree_view_column_path"),
                fl!("main_tree_view_column_modification"),
            ], // Big files
            vec![fl!("main_tree_view_column_file_name"), fl!("main_tree_view_column_path"), fl!("main_tree_view_column_modification")], // Empty files
            vec![fl!("main_tree_view_column_file_name"), fl!("main_tree_view_column_path"), fl!("main_tree_view_column_modification")], // Temporary Files
            vec![
                fl!("main_tree_view_column_similarity"),
                fl!("main_tree_view_column_size"),
                fl!("main_tree_view_column_dimensions"),
                fl!("main_tree_view_column_file_name"),
                fl!("main_tree_view_column_path"),
                fl!("main_tree_view_column_modification"),
            ], // Similar Images
            vec![
                fl!("main_tree_view_column_size"),
                fl!("main_tree_view_column_file_name"),
                fl!("main_tree_view_column_path"),
                fl!("main_tree_view_column_modification"),
            ], // Similar Videos
            vec![
                fl!("main_tree_view_column_size"),
                fl!("main_tree_view_column_file_name"),
                fl!("main_tree_view_column_path"),
                fl!("main_tree_view_column_title"),
                fl!("main_tree_view_column_artist"),
                fl!("main_tree_view_column_year"),
                fl!("main_tree_view_column_album_title"),
                fl!("main_tree_view_column_album_artist"),
                fl!("main_tree_view_column_modification"),
            ], // Music Dupliactes
            vec![
                fl!("main_tree_view_column_symlink_file_name"),
                fl!("main_tree_view_column_symlink_folder"),
                fl!("main_tree_view_column_destination_path"),
                fl!("main_tree_view_column_type_of_error"),
                fl!("main_tree_view_column_modification"),
            ], // Invalid Symlinks
            vec![
                fl!("main_tree_view_column_file_name"),
                fl!("main_tree_view_column_path"),
                fl!("main_tree_view_column_type_of_error"),
                fl!("main_tree_view_column_modification"),
            ], // Broken Files
        ];

        for (notebook_index, tree_view) in self.get_main_tree_views().iter().enumerate() {
            for (column_index, column) in tree_view.columns().iter().enumerate() {
                if column_index == 0 {
                    continue; // Selection button
                }
                column.set_title(&names_of_columns[notebook_index][column_index - 1]);
            }
        }
    }
}
