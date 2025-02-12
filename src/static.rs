use phf::phf_map;

pub static PAPER_SIZES: phf::Map<&'static str, &'static str> = phf_map! {
    "papersize_monarch" => "Envelope Monarch",
    "papersize_business" => "Envelope #10",
    "papersize_internal_dl" => "Envelope DL",
    "papersize_internal_c5" => "Envelope C5",
    "papersize_internal_b5" => "Intern B5",
    "papersize_executive" => "Executive",
    "papersize_us_letter" => "Letter",
    "papersize_us_letter_r" => "Letter-R",
    "papersize_us_legal" => "Legal",
    "papersize_a4" => "A4",
    "papersize_a4_r" => "A4-R",
    "papersize_jis_b5" => "B5",
    "papersize_jis_b5_r" => "B5-R",
    "papersize_a3" => "A3",
    "papersize_b4" => "B4",
    "papersize_us_ledger" => "Ledger",
    "papersize_a5" => "A5",
    "papersize_a6" => "A6",
    "papersize_jis_b6" => "B6",
    "papersize_commercial9" => "Envelope #9",
    "papersize_commercial6" => "Envelope #6",
    "papersize_iso_b5" => "ISO B5",
    "papersize_custom" => "Custom",
    "papersize_custom1" => "Custom 1",
    "papersize_envelope_c4" => "Envelope C4",
    "papersize_hagaki" => "Hagaki",
    "papersize_ofuku_hagaki" => "Oufukuhagaki",
    "papersize_oficio2" => "OficioII",
    "papersize_8k" => "8K",
    "papersize_16k" => "16K",
    "papersize_16k_r" => "16K-R",
    "papersize_statement" => "Statement",
    "papersize_folio" => "Folio",
    "papersize_youkei2" => "Youkei 2",
    "papersize_youkei4" => "Youkei 4",
    "papersize_216_340" => "216x340mm",
    "papersize_Auto_Cm" => "Auto (metric)",
    "papersize_Auto_Inch" => "Auto (inches)",
    "papersize_11_15" => "11x15",
    "papersize_us_legal_r" => "Legal-R",
    "papersize_a3_r" => "A3-R",
    "papersize_b4_r" => "B4-R",
    "papersize_us_ledger_r" => "Ledger-R",
    "papersize_a5_r" => "A5-R",
    "papersize_oficio2_r" => "Statement-R",
    "papersize_statement_r" => "Statement-R",
    "papersize_12_18" => "12x18\"",
    "papersize_kaku2" => "Kakugata 2",
    "papersize_nagakaku_3" => "Nagagata 3",
    "papersize_yolong_3" => "Younaga 3",
    "papersize_13_inches" => "13x19.2\"",
    "papersize_nagagata4" => "Nagagata 4",
    "papersize_sra3" => "SRA3",
};
pub static MEDIA_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    "mediatype_plain" => "Plain",
    "mediatype_transparency" => "Transparency",
    "mediatype_preprinted" => "Preprinted",
    "mediatype_labels" => "Labels",
    "mediatype_bond" => "Bond",
    "mediatype_recycled" => "Recycled",
    "mediatype_vellum" => "Vellum",
    "mediatype_rough" => "Rough",
    "mediatype_letterhead" => "Letterhead",
    "mediatype_color" => "Color",
    "mediatype_prepunched" => "Prepunched",
    "mediatype_envelope" => "Envelope",
    "mediatype_cardstock" => "Cardstock",
    "mediatype_thick" => "Thick",
    "mediatype_highquality" => "High Quality",
    "mediatype_coated" => "Coated",
    "mediatype_custom1" => "Custom 1",
    "mediatype_custom2" => "Custom 2",
    "mediatype_custom3" => "Custom 3",
    "mediatype_custom4" => "Custom 4",
    "mediatype_custom5" => "Custom 5",
    "mediatype_custom6" => "Custom 6",
    "mediatype_custom7" => "Custom 7",
    "mediatype_custom8" => "Custom 8",
    "mediatype_indexTab" => "Index Tab Dividers",
    "mediatype_inkjetPaper" => "Inkjet Paper",
    "mediatype_inkjetMattePaper" => "Inkjet Matte Paper",
    "mediatype_thinpaper" => "Thin"
};
// ripped this from the cursed printer js, hopefully wont change...
pub static STATUSES: [&str; 16] = [
    "Printing...",
    "Scanning...",
    "Ready.",
    "Toner Low...", // error
    "OK",
    "Connected phone in use.",
    "Dialing...",
    "Receiving...",
    "Sending...",
    "Error has occurred.", // error
    "Preparing...",
    "Sleeping...",
    "Cannot recognize.", // error
    "Adjusting...",
    "Phone is off the hook.",
    "Suspending...",
];
// below maps to the 3 errors above ^, also ripped from printer js
pub static ERRORS: [usize; 3] = [3, 9, 12];
pub static WASTE_TONER_STATUSES: [&str; 4] = [
    "Warning", //
    "Full",    //
    "OK",      // seems to be only "Ready" value?
    "Removed",
];
pub static TONER_KEYS: [&str; 4] = ["Black", "Cyan", "Magenta", "Yellow"];

// pub const TONER_THRESHOLD: f64 = 0f64;
// pub const PAPER_THRESHOLD: usize = 0usize;
