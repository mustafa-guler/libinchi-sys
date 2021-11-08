use std::env;
use std::path::Path;

// these were extracted from RDKit's CMake files for InChI support
const INCHI_C_FILES: [&str; 57] = [
    "INCHI-1-SRC/INCHI_BASE/src/ichi_bns.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichi_io.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichican2.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichicano.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichicans.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichierr.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiisot.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichimak2.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichimake.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichimap1.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichimap2.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichimap4.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichinorm.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiparm.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiprt1.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiprt2.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiprt3.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiqueu.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiread.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichiring.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr1.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr2.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr3.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr4.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr5.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr6.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichirvr7.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichisort.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichister.c",
    "INCHI-1-SRC/INCHI_BASE/src/ichitaut.c",
    "INCHI-1-SRC/INCHI_BASE/src/ikey_base26.c",
    "INCHI-1-SRC/INCHI_BASE/src/ikey_dll.c",
    "INCHI-1-SRC/INCHI_BASE/src/mol2atom.c",
    "INCHI-1-SRC/INCHI_BASE/src/mol_fmt1.c",
    "INCHI-1-SRC/INCHI_BASE/src/mol_fmt2.c",
    "INCHI-1-SRC/INCHI_BASE/src/mol_fmt3.c",
    "INCHI-1-SRC/INCHI_BASE/src/mol_fmt4.c",
    "INCHI-1-SRC/INCHI_BASE/src/readinch.c",
    "INCHI-1-SRC/INCHI_BASE/src/runichi.c",
    "INCHI-1-SRC/INCHI_BASE/src/runichi2.c",
    "INCHI-1-SRC/INCHI_BASE/src/runichi3.c",
    "INCHI-1-SRC/INCHI_BASE/src/runichi4.c",
    "INCHI-1-SRC/INCHI_BASE/src/sha2.c",
    "INCHI-1-SRC/INCHI_BASE/src/strutil.c",
    "INCHI-1-SRC/INCHI_BASE/src/util.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ichilnct.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/inchi_dll.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/inchi_dll_a.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/inchi_dll_a2.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/inchi_dll_b.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/inchi_dll_main.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ixa/ixa_builder.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ixa/ixa_inchikey_builder.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ixa/ixa_mol.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ixa/ixa_read_inchi.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ixa/ixa_read_mol.c",
    "INCHI-1-SRC/INCHI_API/libinchi/src/ixa/ixa_status.c",
];

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    cc::Build::new()
        .files(INCHI_C_FILES.iter())
        .pic(true)
        .define("TARGET_API_LIB", None)
        .include(Path::new(&out_dir).join("INCHI-1-SRC/INCHI_BASE/src"))
        .include(Path::new(&out_dir).join("INCHI-1-SRC/INCHI_API/libinchi/src/"))
        .opt_level(3)
        .flag_if_supported("-Wno-everything")
        .flag_if_supported("-w")
        .warnings(false)
        .extra_warnings(false)
        .compile("inchi");

    bindgen::builder()
        .header("INCHI-1-SRC/INCHI_BASE/src/inchi_api.h")
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("inchi_api.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=INCHI-1-SRC");
}
