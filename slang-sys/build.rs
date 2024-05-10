extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
	let slang_dir = env::var("SLANG_DIR").map(PathBuf::from).expect(
		"Environment variable `SLANG_DIR` should be set to the directory of a Slang installation.",
	);

	let out_dir = env::var("OUT_DIR")
		.map(PathBuf::from)
		.expect("Couldn't determine output directory.");

	// autocxx will generate class bindings that bindgen can't generate
	let mut b =
		autocxx_build::Builder::new("src/cxxbind.rs", &[&slang_dir, &slang_dir.join("include")])
			.build()
			.expect("Couldn't build autocxx bindings.");
	// This assumes all your C++ bindings are in main.rs
	b.flag_if_supported("-std=c++17").compile("slang-cxxbind"); // arbitrary library name, pick anything

	link_libraries(&slang_dir);

	// bindgen will generate the main source of the bindings
	bindgen::builder()
		.header(slang_dir.join("include/slang.h").to_str().unwrap())
		.clang_arg("-v")
		.clang_arg("-xc++")
		.clang_arg("-std=c++17")
		.allowlist_function("slang_.*")
		.allowlist_type("slang.*")
		.allowlist_var("SLANG_.*")
		.with_codegen_config(
			bindgen::CodegenConfig::FUNCTIONS
				| bindgen::CodegenConfig::TYPES
				| bindgen::CodegenConfig::VARS,
		)
		.parse_callbacks(Box::new(ParseCallback {}))
		.default_enum_style(bindgen::EnumVariation::Rust {
			non_exhaustive: false,
		})
		.vtable_generation(true)
		.layout_tests(false)
		.derive_copy(true)
		.generate()
		.expect("Couldn't generate bindings.")
		.write_to_file(out_dir.join("bindings.rs"))
		.expect("Couldn't write bindings.");

	modify_bindings(out_dir);

	// rerun settings
	println!("cargo:rerun-if-changed=src/cxxbind.rs");
}

/// Read bindings.rs file and modify it
fn modify_bindings(out_dir: PathBuf) {
	let bindings_path = out_dir.join("bindings.rs");
	let bindings = std::fs::read_to_string(&bindings_path).expect("Couldn't read bindings file.");

	let modified_bindings = bindings.replace("-> SlangResult", "-> crate::ResultCode");
	// .replace("SlangResult", "crate::ffi::SlangResult")
	// .replace("SlangMatrixLayoutMode", "crate::ffi::SlangMatrixLayoutMode")
	// .replace("SlangInt", "crate::ffi::SlangInt")
	// .replace("slang_SessionFlags", "crate::ffi::slang::SessionFlags")
	// .replace("SlangCompileTarget", "crate::ffi::SlangCompileTarget")
	// .replace("SlangProfileID", "crate::ffi::SlangProfileID")
	// .replace("SlangTargetFlags", "crate::ffi::SlangTargetFlags")
	// .replace("SlangFloatingPointMode", "crate::ffi::SlangFloatingPointMode")
	// .replace("SlangLineDirectiveMode", "crate::ffi::SlangLineDirectiveMode");
	std::fs::write(&bindings_path, modified_bindings).expect("Couldn't write modified bindings.");
}

fn link_libraries(slang_dir: &Path) {
	let lib_dir = slang_dir.join("lib");

	if !lib_dir.is_dir() {
		panic!("Couldn't find the `lib` subdirectory in the Slang installation directory.")
	}

	println!("cargo:rustc-link-search=native={}", lib_dir.display());
	println!("cargo:rustc-link-lib=dylib=slang");
}

#[derive(Debug)]
struct ParseCallback {}

impl bindgen::callbacks::ParseCallbacks for ParseCallback {
	fn enum_variant_name(
		&self,
		enum_name: Option<&str>,
		original_variant_name: &str,
		_variant_value: bindgen::callbacks::EnumVariantValue,
	) -> Option<String> {
		let enum_name = enum_name?;

		// Map enum names to the part of their variant names that needs to be trimmed.
		// When an enum name is not in this map the code below will try to trim the enum name itself.
		let mut map = std::collections::HashMap::new();
		map.insert("SlangMatrixLayoutMode", "SlangMatrixLayout");
		map.insert("SlangCompileTarget", "Slang");

		let trim = map.get(enum_name).unwrap_or(&enum_name);
		let new_variant_name = pascal_case_from_snake_case(original_variant_name);
		let new_variant_name = new_variant_name.trim_start_matches(trim);
		Some(new_variant_name.to_string())
	}
}

/// Converts `snake_case` or `SNAKE_CASE` to `PascalCase`.
/// If the input is already in `PascalCase` it will be returned as is.
fn pascal_case_from_snake_case(snake_case: &str) -> String {
	let mut result = String::new();

	let should_lower = snake_case
		.chars()
		.filter(|c| c.is_alphabetic())
		.all(|c| c.is_uppercase());

	for part in snake_case.split('_') {
		for (i, c) in part.chars().enumerate() {
			if i == 0 {
				result.push(c.to_ascii_uppercase());
			} else if should_lower {
				result.push(c.to_ascii_lowercase());
			} else {
				result.push(c);
			}
		}
	}

	result
}
