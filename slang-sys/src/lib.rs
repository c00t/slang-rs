mod cbind;
mod cxxbind;

// reexport
pub use cxxbind::{
	EntryPointReflection, ProgramLayout, TypeLayoutReflection, TypeParameterReflection,
	TypeReflection, UserAttribute, VariableLayoutReflection, VariableReflection,
};

pub use cbind::{
	slang_BindingType, slang_CompilerOptionEntry, slang_CompilerOptionName,
	slang_CompilerOptionValue, slang_CompilerOptionValueKind, slang_LayoutRules, slang_SessionDesc,
	slang_TargetDesc, slang_TypeReflection_Kind, slang_TypeReflection_ScalarType,
	slang_createGlobalSession, slang_createGlobalSessionWithoutStdLib, IBlobVtable,
	IComponentTypeVtable, IEntryPointVtable, IGlobalSessionVtable, IModuleVtable, ISessionVtable,
	ISlangUnknown__bindgen_vtable, ITypeConformanceVtable, ParameterCategory, ResultCode,
	ResultFacility, ResultFacilityCode, SlangCapabilityID, SlangCompileTarget, SlangDebugInfoLevel,
	SlangFloatingPointMode, SlangLineDirectiveMode, SlangMatrixLayoutMode, SlangOptimizationLevel,
	SlangProfileID, SlangResourceAccess, SlangResourceShape, SlangSourceLanguage, SlangStage,
	SlangUUID, SLANG_API_VERSION, slang_DeclReflection_Kind
};
