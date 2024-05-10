#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_char, c_int, c_void};

use crate::ProgramLayout;

/// Merge of `slang::ParameterCategory` and `SlangParameterCategory`
pub type ParameterCategory = slang_ParameterCategory;

impl From<SlangParameterCategory> for ParameterCategory {
	fn from(category: SlangParameterCategory) -> Self {
		match category {
			SlangParameterCategory::None => ParameterCategory::None,
			SlangParameterCategory::Mixed => ParameterCategory::Mixed,
			SlangParameterCategory::ConstantBuffer => ParameterCategory::ConstantBuffer,
			SlangParameterCategory::ShaderResource => ParameterCategory::ShaderResource,
			SlangParameterCategory::UnorderedAccess => ParameterCategory::UnorderedAccess,
			SlangParameterCategory::VaryingInput => ParameterCategory::VaryingInput,
			SlangParameterCategory::VaryingOutput => ParameterCategory::VaryingOutput,
			SlangParameterCategory::SamplerState => ParameterCategory::SamplerState,
			SlangParameterCategory::Uniform => ParameterCategory::Uniform,
			SlangParameterCategory::DescriptorTableSlot => ParameterCategory::DescriptorTableSlot,
			SlangParameterCategory::SpecializationConstant => {
				ParameterCategory::SpecializationConstant
			}
			SlangParameterCategory::PushConstantBuffer => ParameterCategory::PushConstantBuffer,
			SlangParameterCategory::RegisterSpace => ParameterCategory::RegisterSpace,
			SlangParameterCategory::Generic => ParameterCategory::GenericResource,
			SlangParameterCategory::RayPayload => ParameterCategory::RayPayload,
			SlangParameterCategory::HitAttributes => ParameterCategory::HitAttributes,
			SlangParameterCategory::CallablePayload => ParameterCategory::CallablePayload,
			SlangParameterCategory::ShaderRecord => ParameterCategory::ShaderRecord,
			SlangParameterCategory::ExistentialTypeParam => ParameterCategory::ExistentialTypeParam,
			SlangParameterCategory::ExistentialObjectParam => {
				ParameterCategory::ExistentialObjectParam
			}
			SlangParameterCategory::SubElementRegisterSpace => {
				ParameterCategory::SubElementRegisterSpace
			}
			SlangParameterCategory::Subpass => ParameterCategory::InputAttachmentIndex,
			SlangParameterCategory::MetalArgumentBufferElement => {
				ParameterCategory::MetalArgumentBufferElement
			}
			SlangParameterCategory::MetalAttribute => ParameterCategory::MetalAttribute,
			_ => {
				unreachable!()
			}
		}
	}
}

impl From<ParameterCategory> for SlangParameterCategory {
	fn from(category: ParameterCategory) -> Self {
		match category {
			ParameterCategory::None => SlangParameterCategory::None,
			ParameterCategory::Mixed => SlangParameterCategory::Mixed,
			ParameterCategory::ConstantBuffer => SlangParameterCategory::ConstantBuffer,
			ParameterCategory::ShaderResource => SlangParameterCategory::ShaderResource,
			ParameterCategory::UnorderedAccess => SlangParameterCategory::UnorderedAccess,
			ParameterCategory::VaryingInput => SlangParameterCategory::VaryingInput,
			ParameterCategory::VaryingOutput => SlangParameterCategory::VaryingOutput,
			ParameterCategory::SamplerState => SlangParameterCategory::SamplerState,
			ParameterCategory::Uniform => SlangParameterCategory::Uniform,
			ParameterCategory::DescriptorTableSlot => SlangParameterCategory::DescriptorTableSlot,
			ParameterCategory::SpecializationConstant => {
				SlangParameterCategory::SpecializationConstant
			}
			ParameterCategory::PushConstantBuffer => SlangParameterCategory::PushConstantBuffer,
			ParameterCategory::RegisterSpace => SlangParameterCategory::RegisterSpace,
			ParameterCategory::GenericResource => SlangParameterCategory::Generic,
			ParameterCategory::RayPayload => SlangParameterCategory::RayPayload,
			ParameterCategory::HitAttributes => SlangParameterCategory::HitAttributes,
			ParameterCategory::CallablePayload => SlangParameterCategory::CallablePayload,
			ParameterCategory::ShaderRecord => SlangParameterCategory::ShaderRecord,
			ParameterCategory::ExistentialTypeParam => SlangParameterCategory::ExistentialTypeParam,
			ParameterCategory::ExistentialObjectParam => {
				SlangParameterCategory::ExistentialObjectParam
			}
			ParameterCategory::SubElementRegisterSpace => {
				SlangParameterCategory::SubElementRegisterSpace
			}
			ParameterCategory::InputAttachmentIndex => SlangParameterCategory::Subpass,
			ParameterCategory::MetalArgumentBufferElement => {
				SlangParameterCategory::MetalArgumentBufferElement
			}
			ParameterCategory::MetalAttribute => SlangParameterCategory::MetalAttribute,
			_ => {
				unreachable!()
			}
		}
	}
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResultCode(SlangResult);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResultFacility {
	Typed(ResultFacilityCode),
	PlainCode(i32),
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResultFacilityCode {
	WinGeneral = SLANG_FACILITY_WIN_GENERAL as i32,
	WinInterface = SLANG_FACILITY_WIN_INTERFACE as i32,
	WinAPI = SLANG_FACILITY_WIN_API as i32,
	SlangCore = SLANG_FACILITY_CORE as i32,
	SlangInternal = SLANG_FACILITY_INTERNAL as i32,
	SlangExternalBase = SLANG_FACILITY_EXTERNAL_BASE as i32,
}

impl ResultCode {
	// Win COM compatible Results

	/// OK indicates success
	pub const OK: ResultCode = ResultCode(0);
	/// FAIL is the generic failure code - meaning a serious error occurred and the call couldn't complete
	pub const FAIL: ResultCode = ResultCode::new_error(
		ResultFacility::Typed(ResultFacilityCode::WinGeneral),
		0x4005,
	);
	/// Functionality is not implemented
	pub const NOT_IMPLEMENTED: ResultCode = ResultCode::new_error(
		ResultFacility::Typed(ResultFacilityCode::WinGeneral),
		0x4001,
	);
	/// Interface not be found
	pub const INTERFACE_NOT_FOUND: ResultCode = ResultCode::new_error(
		ResultFacility::Typed(ResultFacilityCode::WinGeneral),
		0x4002,
	);
	/// Operation was aborted (did not correctly complete)
	pub const ABORTED: ResultCode = ResultCode::new_error(
		ResultFacility::Typed(ResultFacilityCode::WinGeneral),
		0x4004,
	);
	/// Indicates that a handle passed in as parameter to a method is invalid.
	pub const INVALID_HANDLE: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::WinAPI), 6);
	/// Indicates that an argument passed in as parameter to a method is invalid.
	pub const INVALID_ARG: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::WinAPI), 0x57);
	/// Operation could not complete - ran out of memory
	pub const OUT_OF_MEMORY: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::WinAPI), 0xE);

	// Slang specific results

	// Supplied buffer is too small to be able to complete
	pub const BUFFER_TOO_SMALL: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 1);
	/// Used to identify a Result that has yet to be initialized.
	pub const UNINITIALIZED: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 2);
	/// Returned from an async method meaning the output is invalid (thus an error), but a result for the request is pending, and will be returned on a subsequent call with the async handle.
	pub const PENDING: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 3);
	/// Indicates a file/resource could not be opened
	pub const FILE_CANNOT_OPEN: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 4);
	/// Indicates a file/resource could not be found
	pub const FILE_NOT_FOUND: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 5);
	/// An unhandled internal failure (typically from unhandled exception)
	pub const INTERNAL_FAIL: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 6);
	/// Could not complete because some underlying feature (hardware or software) was not available
	pub const NOT_AVAILABLE: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 7);
	/// Could not complete because the operation times out.
	pub const TIMEOUT: ResultCode =
		ResultCode::new_error(ResultFacility::Typed(ResultFacilityCode::SlangCore), 8);

	pub fn is_failed(&self) -> bool {
		self.0 < 0
	}

	pub fn is_succeeded(&self) -> bool {
		self.0 >= 0
	}

	/// Create a new error code from a facility and code
	///
	/// Create from cxx macro:
	///
	/// #define SLANG_MAKE_ERROR(fac, code)		((((int32_t)(fac)) << 16) | ((int32_t)(code)) | int32_t(0x80000000))
	pub const fn new_error(facility: ResultFacility, code: u32) -> Self {
		let facility = match facility {
			ResultFacility::Typed(c) => c as i32,
			ResultFacility::PlainCode(c) => c,
		};
		ResultCode((facility << 16 | code as i32) | (0x80000000u32 as i32))
	}

	/// Create a new success code from a facility and code
	///
	/// Create from cxx macro:
	///
	/// #define SLANG_MAKE_SUCCESS(fac, code)    ((((int32_t)(fac)) << 16) | ((int32_t)(code)))
	pub const fn new_success(facility: ResultFacility, code: u32) -> Self {
		let facility = match facility {
			ResultFacility::Typed(c) => c as i32,
			ResultFacility::PlainCode(c) => c,
		};
		ResultCode((facility << 16 | code as i32))
	}

	/// Get the facility part of the result code
	///
	/// Create from cxx macro:
	///
	/// #define SLANG_GET_RESULT_FACILITY(r)    ((int32_t)(((r) >> 16) & 0x7fff))
	pub fn facility(&self) -> ResultFacility {
		// determine facility code belong to ResultFacility::Typed or ResultFacility::PlainCode
		let facility = (self.0 >> 16) & 0x7fff;
		match facility as u32 {
			SLANG_FACILITY_WIN_GENERAL => ResultFacility::Typed(ResultFacilityCode::WinGeneral),
			SLANG_FACILITY_WIN_INTERFACE => ResultFacility::Typed(ResultFacilityCode::WinInterface),
			SLANG_FACILITY_WIN_API => ResultFacility::Typed(ResultFacilityCode::WinAPI),
			SLANG_FACILITY_CORE => ResultFacility::Typed(ResultFacilityCode::SlangCore),
			SLANG_FACILITY_INTERNAL => ResultFacility::Typed(ResultFacilityCode::SlangInternal),
			SLANG_FACILITY_EXTERNAL_BASE => {
				ResultFacility::Typed(ResultFacilityCode::SlangExternalBase)
			}
			_ => ResultFacility::PlainCode(facility),
		}
	}

	/// Get the code part of the result code
	///
	/// Create from cxx macro:
	///
	/// #define SLANG_GET_RESULT_CODE(r)        ((int32_t)((r) & 0xffff))
	pub fn code(&self) -> i32 {
		self.0 & 0xffff
	}

	/// Get the error type str of the result code
	fn error_type(&self) -> &str {
		match *self {
			ResultCode::FAIL => "WinGeneral(Generic Failure)",
			ResultCode::NOT_IMPLEMENTED => "WinGeneral(Not Implemented)",
			ResultCode::INTERFACE_NOT_FOUND => "WinGeneral(Interface Not Found)",
			ResultCode::ABORTED => "WinGeneral(Aborted)",
			ResultCode::INVALID_HANDLE => "WinAPI(Invalid Handle)",
			ResultCode::INVALID_ARG => "WinAPI(Invalid Argument)",
			ResultCode::OUT_OF_MEMORY => "WinAPI(Out of Memory)",
			ResultCode::BUFFER_TOO_SMALL => "SlangCore(Buffer Too Small)",
			ResultCode::UNINITIALIZED => "SlangCore(Uninitialized)",
			ResultCode::PENDING => "SlangCore(Pending)",
			ResultCode::FILE_CANNOT_OPEN => "SlangCore(File Cannot Open)",
			ResultCode::FILE_NOT_FOUND => "SlangCore(File Not Found)",
			ResultCode::INTERNAL_FAIL => "SlangCore(Internal Failure)",
			ResultCode::NOT_AVAILABLE => "SlangCore(Not Available)",
			ResultCode::TIMEOUT => "SlangCore(Timeout)",
			_ => "Unknown Error",
		}
	}
}

impl std::error::Error for ResultCode {}

impl std::fmt::Display for ResultCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{};Facility:{:?};Code:{:x}",
			self.error_type(),
			self.facility(),
			self.code()
		)
	}
}

impl std::fmt::Debug for ResultCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{};Facility:{:?};Code:{:x}",
			self.error_type(),
			self.facility(),
			self.code()
		)
	}
}

// Based on Slang version 2024.1.6

#[repr(C)]
pub struct IBlobVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub getBufferPointer: unsafe extern "stdcall" fn(*mut c_void) -> *const c_void,
	pub getBufferSize: unsafe extern "stdcall" fn(*mut c_void) -> usize,
}

#[repr(C)]
pub struct IGlobalSessionVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub createSession: unsafe extern "stdcall" fn(
		*mut c_void,
		desc: *const slang_SessionDesc,
		outSession: *mut *mut slang_ISession,
	) -> ResultCode,
	pub findProfile: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangProfileID,
	pub setDownstreamCompilerPath:
		unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, path: *const c_char),
	#[deprecated(note = "Use setLanguagePrelude instead")]
	pub setDownstreamCompilerPrelude: unsafe extern "stdcall" fn(
		*mut c_void,
		passThrough: SlangPassThrough,
		preludeText: *const c_char,
	),
	#[deprecated(note = "Use getLanguagePrelude instead")]
	pub getDownstreamCompilerPrelude: unsafe extern "stdcall" fn(
		*mut c_void,
		passThrough: SlangPassThrough,
		outPrelude: *mut *mut ISlangBlob,
	),
	pub getBuildTagString: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub setDefaultDownstreamCompiler: unsafe extern "stdcall" fn(
		*mut c_void,
		sourceLanguage: SlangSourceLanguage,
		defaultCompiler: SlangPassThrough,
	) -> ResultCode,
	pub getDefaultDownstreamCompiler: unsafe extern "stdcall" fn(
		*mut c_void,
		sourceLanguage: SlangSourceLanguage,
	) -> SlangPassThrough,
	pub setLanguagePrelude: unsafe extern "stdcall" fn(
		*mut c_void,
		sourceLanguage: SlangSourceLanguage,
		preludeText: *const c_char,
	),
	pub getLanguagePrelude: unsafe extern "stdcall" fn(
		*mut c_void,
		sourceLanguage: SlangSourceLanguage,
		outPrelude: *mut *mut ISlangBlob,
	),
	pub createCompileRequest:
		unsafe extern "stdcall" fn(*mut c_void, *mut *mut slang_ICompileRequest) -> ResultCode,
	pub addBuiltins: unsafe extern "stdcall" fn(
		*mut c_void,
		sourcePath: *const c_char,
		sourceString: *const c_char,
	),
	pub setSharedLibraryLoader:
		unsafe extern "stdcall" fn(*mut c_void, loader: *mut ISlangSharedLibraryLoader),
	pub getSharedLibraryLoader:
		unsafe extern "stdcall" fn(*mut c_void) -> *mut ISlangSharedLibraryLoader,
	pub checkCompileTargetSupport:
		unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget) -> ResultCode,
	pub checkPassThroughSupport:
		unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough) -> ResultCode,
	pub compileStdLib:
		unsafe extern "stdcall" fn(*mut c_void, flags: slang_CompileStdLibFlags) -> ResultCode,
	pub loadStdLib: unsafe extern "stdcall" fn(
		*mut c_void,
		stdLib: *const c_void,
		stdLibSizeInBytes: usize,
	) -> ResultCode,
	pub saveStdLib: unsafe extern "stdcall" fn(
		*mut c_void,
		archiveType: SlangArchiveType,
		outBlob: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub findCapability:
		unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangCapabilityID,
	pub setDownstreamCompilerForTransition: unsafe extern "stdcall" fn(
		*mut c_void,
		source: SlangCompileTarget,
		target: SlangCompileTarget,
		compiler: SlangPassThrough,
	),
	pub getDownstreamCompilerForTransition: unsafe extern "stdcall" fn(
		*mut c_void,
		source: SlangCompileTarget,
		target: SlangCompileTarget,
	) -> SlangPassThrough,
	pub getCompilerElapsedTime: unsafe extern "stdcall" fn(
		*mut c_void,
		outTotalTime: *mut f64,
		outDownstreamTime: *mut f64,
	),
	pub setSPIRVCoreGrammar:
		unsafe extern "stdcall" fn(*mut c_void, jsonPath: *const c_char) -> ResultCode,
	pub parseCommandLineArguments: unsafe extern "stdcall" fn(
		*mut c_void,
		argc: c_int,
		argv: *const *const c_char,
		outSessionDesc: *mut slang_SessionDesc,
		outAuxAllocation: *mut *mut ISlangUnknown,
	) -> ResultCode,
	pub getSessionDescDigest: unsafe extern "stdcall" fn(
		*mut c_void,
		sessionDesc: *const slang_SessionDesc,
		outBlob: *mut *mut ISlangBlob,
	) -> ResultCode,
}

#[repr(C)]
pub struct ISessionVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub getGlobalSession: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_IGlobalSession,
	pub loadModule: unsafe extern "stdcall" fn(
		*mut c_void,
		moduleName: *const c_char,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_IModule,
	pub loadModuleFromSource: unsafe extern "stdcall" fn(
		*mut c_void,
		moduleName: *const c_char,
		path: *const c_char,
		source: *mut ISlangBlob,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_IModule,
	pub createCompositeComponentType: unsafe extern "stdcall" fn(
		*mut c_void,
		componentTypes: *const *const slang_IComponentType,
		componentTypeCount: SlangInt,
		outCompositeComponentType: *mut *mut slang_IComponentType,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub specializeType: unsafe extern "stdcall" fn(
		*mut c_void,
		type_: *mut slang_TypeReflection,
		specializationArgs: *const slang_SpecializationArg,
		specializationArgCount: SlangInt,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_TypeReflection,
	pub getTypeLayout: unsafe extern "stdcall" fn(
		*mut c_void,
		type_: *mut slang_TypeReflection,
		targetIndex: SlangInt,
		rules: slang_LayoutRules,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_TypeLayoutReflection,
	pub getContainerType: unsafe extern "stdcall" fn(
		*mut c_void,
		elementType: *mut slang_TypeReflection,
		containerType: slang_ContainerType,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_TypeReflection,
	pub getDynamicType: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_TypeReflection,
	pub getTypeRTTIMangledName: unsafe extern "stdcall" fn(
		*mut c_void,
		type_: *mut slang_TypeReflection,
		outNameBlob: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub getTypeConformanceWitnessMangledName: unsafe extern "stdcall" fn(
		*mut c_void,
		type_: *mut slang_TypeReflection,
		interfaceType: *mut slang_TypeReflection,
		outNameBlob: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub getTypeConformanceWitnessSequentialID: unsafe extern "stdcall" fn(
		*mut c_void,
		type_: *mut slang_TypeReflection,
		interfaceType: *mut slang_TypeReflection,
		outId: *mut u32,
	) -> ResultCode,
	pub createCompileRequest: unsafe extern "stdcall" fn(
		*mut c_void,
		outCompileRequest: *mut *mut slang_ICompileRequest,
	) -> ResultCode,
	pub createTypeConformanceComponentType: unsafe extern "stdcall" fn(
		*mut c_void,
		type_: *mut slang_TypeReflection,
		interfaceType: *mut slang_TypeReflection,
		outConformance: *mut *mut slang_ITypeConformance,
		conformanceIdOverride: SlangInt,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub loadModuleFromIRBlob: unsafe extern "stdcall" fn(
		*mut c_void,
		moduleName: *const c_char,
		path: *const c_char,
		source: *mut ISlangBlob,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_IModule,
	pub getLoadedModuleCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt,
	pub getLoadedModule:
		unsafe extern "stdcall" fn(*mut c_void, index: SlangInt) -> *mut slang_IModule,
	pub isBinaryModuleUpToDate: unsafe extern "stdcall" fn(
		*mut c_void,
		modulePath: *const c_char,
		binaryModuleBlob: *mut ISlangBlob,
	) -> bool,
	pub loadModuleFromSourceString: unsafe extern "stdcall" fn(
		*mut c_void,
		moduleName: *const c_char,
		path: *const c_char,
		string: *const c_char,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut slang_IModule,
}

#[repr(C)]
pub struct IComponentTypeVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub getSession: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_ISession,
	pub getLayout: unsafe extern "stdcall" fn(
		*mut c_void,
		targetIndex: SlangInt,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> *mut ProgramLayout,
	pub getSpecializationParamCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt,
	pub getEntryPointCode: unsafe extern "stdcall" fn(
		*mut c_void,
		entryPointIndex: SlangInt,
		targetIndex: SlangInt,
		outCode: *mut *mut ISlangBlob,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub getResultAsFileSystem: unsafe extern "stdcall" fn(
		*mut c_void,
		entryPointIndex: SlangInt,
		targetIndex: SlangInt,
		outFileSystem: *mut *mut ISlangMutableFileSystem,
	) -> ResultCode,
	pub getEntryPointHash: unsafe extern "stdcall" fn(
		*mut c_void,
		entryPointIndex: SlangInt,
		targetIndex: SlangInt,
		outHash: *mut *mut ISlangBlob,
	),
	pub specialize: unsafe extern "stdcall" fn(
		*mut c_void,
		specializationArgs: *const slang_SpecializationArg,
		specializationArgCount: SlangInt,
		outSpecializedComponentType: *mut *mut slang_IComponentType,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub link: unsafe extern "stdcall" fn(
		*mut c_void,
		outLinkedComponentType: *mut *mut slang_IComponentType,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub getEntryPointHostCallable: unsafe extern "stdcall" fn(
		*mut c_void,
		entryPointIndex: c_int,
		targetIndex: c_int,
		outSharedLibrary: *mut *mut ISlangSharedLibrary,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub renameEntryPoint: unsafe extern "stdcall" fn(
		*mut c_void,
		newName: *const c_char,
		outEntryPoint: *mut *mut slang_IComponentType,
	) -> ResultCode,
	pub linkWithOptions: unsafe extern "stdcall" fn(
		*mut c_void,
		outLinkedComponentType: *mut *mut slang_IComponentType,
		compilerOptionEntryCount: u32,
		compilerOptionEntries: *mut slang_CompilerOptionEntry,
		outDiagnostics: *mut *mut ISlangBlob,
	) -> ResultCode,
}

#[repr(C)]
pub struct IEntryPointVtable {
	pub _base: IComponentTypeVtable,
}

#[repr(C)]
pub struct ITypeConformanceVtable {
	pub _base: IComponentTypeVtable,
}

#[repr(C)]
pub struct IModuleVtable {
	pub _base: IComponentTypeVtable,

	pub findEntryPointByName: unsafe extern "stdcall" fn(
		*mut c_void,
		name: *const c_char,
		outEntryPoint: *mut *mut slang_IEntryPoint,
	) -> ResultCode,
	pub getDefinedEntryPointCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt32,
	pub getDefinedEntryPoint: unsafe extern "stdcall" fn(
		*mut c_void,
		index: SlangInt32,
		outEntryPoint: *mut *mut slang_IEntryPoint,
	) -> ResultCode,
	pub serialize: unsafe extern "stdcall" fn(
		*mut c_void,
		outSerializedBlob: *mut *mut ISlangBlob,
	) -> ResultCode,
	pub writeToFile: unsafe extern "stdcall" fn(*mut c_void, fileName: *const c_char) -> ResultCode,
	pub getName: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub getFilePath: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub getUniqueIdentity: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
}
