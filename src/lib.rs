use slang_sys as sys;
use std::ffi::{CStr, CString};
use std::pin::Pin;
use std::ptr::{null, null_mut};

pub use sys::{
    slang_CompilerOptionName as CompilerOptionName, slang_SessionDesc as SessionDesc,
    slang_TargetDesc as TargetDesc, EntryPointReflection, ProgramLayout, ResultCode,
    SlangCapabilityID, SlangCompileTarget as CompileTarget,
    SlangDebugInfoFormat as DebugInfoFormat, SlangDebugInfoLevel as DebugInfoLevel,
    SlangFloatingPointMode as FloatingPointMode, SlangLineDirectiveMode as LineDirectiveMode,
    SlangMatrixLayoutMode as MatrixLayoutMode, SlangOptimizationLevel as OptimizationLevel,
    SlangProfileID, SlangSourceLanguage as SourceLanguage, SlangStage as Stage, SlangUUID as UUID,
    TypeLayoutReflection, TypeParameterReflection, TypeReflection, UserAttribute,
    VariableLayoutReflection, VariableReflection,
};

macro_rules! vcall {
    ($self:expr, $method:ident($($args:expr),*)) => {
        unsafe { ($self.vtable().$method)($self.as_raw(), $($args),*) }
    };
}

const fn uuid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> UUID {
    UUID {
        data1,
        data2,
        data3,
        data4,
    }
}

pub enum Error {
    Code(ResultCode),
    Blob(Blob),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Code(code) => write!(f, "{}", code),
            Error::Blob(blob) => write!(f, "{}", blob.as_str().unwrap()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct ProfileID(sys::SlangProfileID);

impl ProfileID {
    pub const UNKNOWN: ProfileID = ProfileID(sys::SlangProfileID::SlangProfileUnknown);
}

pub struct CapabilityID(sys::SlangCapabilityID);

impl CapabilityID {
    pub const UNKNOWN: CapabilityID = CapabilityID(sys::SlangCapabilityID::SlangCapabilityUnknown);
}

unsafe trait Interface: Sized {
    type Vtable;
    const IID: UUID;

    #[inline(always)]
    unsafe fn vtable(&self) -> &Self::Vtable {
        &**(self.as_raw() as *mut *mut Self::Vtable)
    }

    #[inline(always)]
    unsafe fn as_raw<T>(&self) -> *mut T {
        std::mem::transmute_copy(self)
    }

    fn as_unknown(&self) -> &IUnknown {
        // SAFETY: It is always safe to treat an `Interface` as an `IUnknown`.
        unsafe { std::mem::transmute(self) }
    }
}

pub unsafe trait Downcast<T> {
    fn downcast(&self) -> &T;
}

#[repr(transparent)]
pub struct IUnknown(std::ptr::NonNull<std::ffi::c_void>);

unsafe impl Interface for IUnknown {
    type Vtable = sys::ISlangUnknown__bindgen_vtable;
    const IID: UUID = uuid(
        0x00000000,
        0x0000,
        0x0000,
        [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        vcall!(self, ISlangUnknown_addRef());
        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        vcall!(self, ISlangUnknown_release());
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Blob(IUnknown);

unsafe impl Interface for Blob {
    type Vtable = sys::IBlobVtable;
    const IID: UUID = uuid(
        0x8ba5fb08,
        0x5195,
        0x40e2,
        [0xac, 0x58, 0x0d, 0x98, 0x9c, 0x3a, 0x01, 0x02],
    );
}

impl Blob {
    pub fn as_slice(&self) -> &[u8] {
        let ptr = vcall!(self, getBufferPointer());
        let size = vcall!(self, getBufferSize());
        unsafe { std::slice::from_raw_parts(ptr as *const u8, size) }
    }

    pub fn as_str(&self) -> std::result::Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_slice())
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct GlobalSession(IUnknown);

unsafe impl Interface for GlobalSession {
    type Vtable = sys::IGlobalSessionVtable;
    const IID: UUID = uuid(
        0xc140b5fd,
        0x0c78,
        0x452e,
        [0xba, 0x7c, 0x1a, 0x1e, 0x70, 0xc7, 0xf7, 0x1c],
    );
}

impl GlobalSession {
    pub fn new() -> Result<GlobalSession> {
        let mut global_session = null_mut();
        let result = unsafe {
            sys::slang_createGlobalSession(sys::SLANG_API_VERSION as _, &mut global_session)
        };
        if result.is_failed() {
            return Err(Error::Code(result));
        } else {
            Ok(GlobalSession(IUnknown(
                std::ptr::NonNull::new(global_session as *mut _)
                    .expect("Result conflict with null ptr!"),
            )))
        }
    }

    pub fn new_without_std_lib() -> Result<GlobalSession> {
        let mut global_session = null_mut();
        let result = unsafe {
            sys::slang_createGlobalSessionWithoutStdLib(
                sys::SLANG_API_VERSION as _,
                &mut global_session,
            )
        };
        if result.is_failed() {
            return Err(Error::Code(result));
        } else {
            Ok(GlobalSession(IUnknown(
                std::ptr::NonNull::new(global_session as *mut _)
                    .expect("Result conflict with null ptr!"),
            )))
        }
    }

    pub fn create_session(&self, desc: &SessionDesc) -> Result<Session> {
        let mut session = null_mut();
        let result = vcall!(self, createSession(desc, &mut session));

        if result.is_failed() {
            return Err(Error::Code(result));
        }

        let session = Session(IUnknown(
            std::ptr::NonNull::new(session as *mut _).expect("Result conflict with null ptr!"),
        ));

        // TODO: Without adding an extra reference, the code crashes when Session is dropped.
        // Investigate why this is happening, the current solution could cause a memory leak.
        //
        // Note: cupofc0t
        // According to https://shader-slang.com/slang/user-guide/compiling.html#using-the-compilation-api
        // We should use a `ComPtr` to wrap around the `Session` object to handle the reference counting,
        // I think it's correct to add an extra reference when IUnknown is created from a raw pointer on rust side.
        // unsafe { (session.as_unknown().vtable().ISlangUnknown_addRef)(session.as_raw()) };

        Ok(session)
    }

    pub fn find_profile(&self, name: &str) -> ProfileID {
        let name = CString::new(name).unwrap();
        ProfileID(vcall!(self, findProfile(name.as_ptr())))
    }

    pub fn find_capability(&self, name: &str) -> CapabilityID {
        let name = CString::new(name).unwrap();
        CapabilityID(vcall!(self, findCapability(name.as_ptr())))
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Session(IUnknown);

unsafe impl Interface for Session {
    type Vtable = sys::ISessionVtable;
    const IID: UUID = uuid(
        0x67618701,
        0xd116,
        0x468f,
        [0xab, 0x3b, 0x47, 0x4b, 0xed, 0xce, 0x0e, 0x3d],
    );
}

impl Session {
    pub fn load_module(&self, name: &str) -> Result<Module> {
        let name = CString::new(name).unwrap();
        let mut diagnostics = null_mut();

        let module = vcall!(self, loadModule(name.as_ptr(), &mut diagnostics));

        if module.is_null() {
            let blob = Blob(IUnknown(
                std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
            ));
            Err(Error::Blob(blob))
        } else {
            let module = Module(IUnknown(std::ptr::NonNull::new(module as *mut _).unwrap()));
            unsafe { (module.as_unknown().vtable().ISlangUnknown_addRef)(module.as_raw()) };
            Ok(module)
        }
    }

    pub fn create_composite_component_type(
        &self,
        components: &[&ComponentType],
    ) -> Result<ComponentType> {
        let components: Vec<*mut std::ffi::c_void> =
            unsafe { components.iter().map(|c| c.as_raw()).collect() };

        let mut composite_component_type = null_mut();
        let mut diagnostics = null_mut();
        let res = vcall!(
            self,
            createCompositeComponentType(
                components.as_ptr() as _,
                components.len() as _,
                &mut composite_component_type,
                &mut diagnostics
            )
        );

        if res.is_failed() {
            let blob = Blob(IUnknown(
                std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
            ));
            return Err(Error::Blob(blob));
        }
        Ok(ComponentType(IUnknown(
            std::ptr::NonNull::new(composite_component_type as *mut _).unwrap(),
        )))
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ComponentType(IUnknown);

unsafe impl Interface for ComponentType {
    type Vtable = sys::IComponentTypeVtable;
    const IID: UUID = uuid(
        0x5bc42be8,
        0x5c50,
        0x4929,
        [0x9e, 0x5e, 0xd1, 0x5e, 0x7c, 0x24, 0x01, 0x5f],
    );
}

impl ComponentType {
    pub fn link(&self) -> Result<ComponentType> {
        let mut linked_component_type = null_mut();
        let mut diagnostics = null_mut();
        let result = vcall!(self, link(&mut linked_component_type, &mut diagnostics));

        if result.is_failed() {
            let blob = Blob(IUnknown(
                std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
            ));
            return Err(Error::Blob(blob));
        }
        Ok(ComponentType(IUnknown(
            std::ptr::NonNull::new(linked_component_type as *mut _).unwrap(),
        )))
    }

    pub fn get_entry_point_code(&self, index: i64, target: i64) -> Result<Vec<u8>> {
        let mut code = null_mut();
        let mut diagnostics = null_mut();
        let res = vcall!(
            self,
            getEntryPointCode(index, target, &mut code, &mut diagnostics)
        );
        if res.is_failed() {
            let blob = Blob(IUnknown(
                std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
            ));
            return Err(Error::Blob(blob));
        }
        let blob = Blob(IUnknown(std::ptr::NonNull::new(code as *mut _).unwrap()));
        Ok(Vec::from(blob.as_slice()))
    }

    /// Get the reflection layout of this component type.
    ///
    /// According to Slang Docs:
    /// In the current Slang API, the ProgramLayout type is not reference-counted.
    /// Currently, the lifetime of a ProgramLayout is tied to the IComponentType that returned it.
    /// An application must ensure that it retains the given IComponentType for as long as it uses the ProgramLayout.
    pub fn get_layout(&self, target_index: i64) -> Pin<&mut ProgramLayout> {
        let mut diagnostics = null_mut();
        let layout = vcall!(self, getLayout(target_index, &mut diagnostics));
        unsafe { Pin::new_unchecked(layout.as_mut().unwrap()) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct EntryPoint(IUnknown);

unsafe impl Interface for EntryPoint {
    type Vtable = sys::IEntryPointVtable;
    const IID: UUID = uuid(
        0x8f241361,
        0xf5bd,
        0x4ca0,
        [0xa3, 0xac, 0x02, 0xf7, 0xfa, 0x24, 0x02, 0xb8],
    );
}

unsafe impl Downcast<ComponentType> for EntryPoint {
    fn downcast(&self) -> &ComponentType {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct TypeConformance(IUnknown);

unsafe impl Interface for TypeConformance {
    type Vtable = sys::ITypeConformanceVtable;
    const IID: UUID = uuid(
        0x73eb3147,
        0xe544,
        0x41b5,
        [0xb8, 0xf0, 0xa2, 0x44, 0xdf, 0x21, 0x94, 0x0b],
    );
}

unsafe impl Downcast<ComponentType> for TypeConformance {
    fn downcast(&self) -> &ComponentType {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Module(IUnknown);

unsafe impl Interface for Module {
    type Vtable = sys::IModuleVtable;
    const IID: UUID = uuid(
        0x0c720e64,
        0x8722,
        0x4d31,
        [0x89, 0x90, 0x63, 0x8a, 0x98, 0xb1, 0xc2, 0x79],
    );
}

unsafe impl Downcast<ComponentType> for Module {
    fn downcast(&self) -> &ComponentType {
        unsafe { std::mem::transmute(self) }
    }
}

impl Module {
    pub fn find_entry_point_by_name(&self, name: &str) -> Result<EntryPoint> {
        let name = CString::new(name).unwrap();
        let mut entry_point = null_mut();
        let result = vcall!(self, findEntryPointByName(name.as_ptr(), &mut entry_point));
        if result.is_failed() {
            return Err(Error::Code(result));
        }

        Ok(EntryPoint(IUnknown(
            std::ptr::NonNull::new(entry_point as *mut _).expect("Result conflict with null ptr!"),
        )))
    }

    pub fn name(&self) -> &str {
        let name = vcall!(self, getName());
        unsafe { CStr::from_ptr(name).to_str().unwrap() }
    }

    pub fn file_path(&self) -> &str {
        let path = vcall!(self, getFilePath());
        unsafe { CStr::from_ptr(path).to_str().unwrap() }
    }

    pub fn unique_identity(&self) -> &str {
        let identity = vcall!(self, getUniqueIdentity());
        unsafe { CStr::from_ptr(identity).to_str().unwrap() }
    }
}

pub struct TargetDescBuilder {
    inner: TargetDesc,
}

impl TargetDescBuilder {
    pub fn new() -> TargetDescBuilder {
        Self {
            inner: TargetDesc {
                structureSize: std::mem::size_of::<TargetDesc>(),
                ..unsafe { std::mem::zeroed() }
            },
        }
    }

    pub fn format(mut self, format: CompileTarget) -> Self {
        self.inner.format = format;
        self
    }

    pub fn profile(mut self, profile: ProfileID) -> Self {
        self.inner.profile = profile.0;
        self
    }

    pub fn options(mut self, options: &OptionsBuilder) -> Self {
        self.inner.compilerOptionEntries = options.options.as_ptr() as _;
        self.inner.compilerOptionEntryCount = options.options.len() as _;
        self
    }
}

impl std::ops::Deref for TargetDescBuilder {
    type Target = TargetDesc;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct SessionDescBuilder {
    inner: SessionDesc,
}

impl SessionDescBuilder {
    pub fn new() -> SessionDescBuilder {
        Self {
            inner: SessionDesc {
                structureSize: std::mem::size_of::<SessionDesc>(),
                ..unsafe { std::mem::zeroed() }
            },
        }
    }

    pub fn targets(mut self, targets: &[TargetDesc]) -> Self {
        self.inner.targets = targets.as_ptr();
        self.inner.targetCount = targets.len() as _;
        self
    }

    pub fn search_paths(mut self, paths: &[*const i8]) -> Self {
        self.inner.searchPaths = paths.as_ptr();
        self.inner.searchPathCount = paths.len() as _;
        self
    }

    pub fn options(mut self, options: &OptionsBuilder) -> Self {
        self.inner.compilerOptionEntries = options.options.as_ptr() as _;
        self.inner.compilerOptionEntryCount = options.options.len() as _;
        self
    }
}

impl std::ops::Deref for SessionDescBuilder {
    type Target = SessionDesc;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

macro_rules! option {
    ($name:ident, $func:ident($p_name:ident: $p_type:ident)) => {
        #[inline(always)]
        pub fn $func(self, $p_name: $p_type) -> Self {
            self.push_ints(CompilerOptionName::$name, $p_name as _, 0)
        }
    };

    ($name:ident, $func:ident($p_name:ident: &str)) => {
        #[inline(always)]
        pub fn $func(self, $p_name: &str) -> Self {
            self.push_str1(CompilerOptionName::$name, $p_name)
        }
    };

    ($name:ident, $func:ident($p_name1:ident: &str, $p_name2:ident: &str)) => {
        #[inline(always)]
        pub fn $func(self, $p_name1: &str, $p_name2: &str) -> Self {
            self.push_str2(CompilerOptionName::$name, $p_name1, $p_name2)
        }
    };
}

pub struct OptionsBuilder {
    strings: Vec<CString>,
    options: Vec<sys::slang_CompilerOptionEntry>,
}

impl OptionsBuilder {
    pub fn new() -> OptionsBuilder {
        OptionsBuilder {
            strings: Vec::new(),
            options: Vec::new(),
        }
    }

    pub fn push_ints(mut self, name: CompilerOptionName, i0: i32, i1: i32) -> Self {
        self.options.push(sys::slang_CompilerOptionEntry {
            name,
            value: sys::slang_CompilerOptionValue {
                kind: sys::slang_CompilerOptionValueKind::Int,
                intValue0: i0,
                intValue1: i1,
                stringValue0: null(),
                stringValue1: null(),
            },
        });

        self
    }

    pub fn push_strings(mut self, name: CompilerOptionName, s0: *const i8, s1: *const i8) -> Self {
        self.options.push(sys::slang_CompilerOptionEntry {
            name,
            value: sys::slang_CompilerOptionValue {
                kind: sys::slang_CompilerOptionValueKind::String,
                intValue0: 0,
                intValue1: 0,
                stringValue0: s0,
                stringValue1: s1,
            },
        });

        self
    }

    pub fn push_str1(mut self, name: CompilerOptionName, s0: &str) -> Self {
        let s0 = CString::new(s0).unwrap();
        let s0_ptr = s0.as_ptr();
        self.strings.push(s0);

        self.push_strings(name, s0_ptr, null())
    }

    pub fn push_str2(mut self, name: CompilerOptionName, s0: &str, s1: &str) -> Self {
        let s0 = CString::new(s0).unwrap();
        let s0_ptr = s0.as_ptr();
        self.strings.push(s0);

        let s1 = CString::new(s1).unwrap();
        let s1_ptr = s1.as_ptr();
        self.strings.push(s1);

        self.push_strings(name, s0_ptr, s1_ptr)
    }
}

impl OptionsBuilder {
    option!(MacroDefine, macro_define(key: &str, value: &str));
    option!(Include, include(path: &str));
    option!(Language, language(language: SourceLanguage));
    option!(MatrixLayoutColumn, matrix_layout_column(enable: bool));
    option!(MatrixLayoutRow, matrix_layout_row(enable: bool));

    #[inline(always)]
    pub fn profile(self, profile: ProfileID) -> Self {
        self.push_ints(CompilerOptionName::Profile, profile.0 as _, 0)
    }

    option!(Stage, stage(stage: Stage));
    option!(Target, target(target: CompileTarget));
    option!(WarningsAsErrors, warnings_as_errors(warning_codes: &str));
    option!(DisableWarnings, disable_warnings(warning_codes: &str));
    option!(EnableWarning, enable_warning(warning_code: &str));
    option!(DisableWarning, disable_warning(warning_code: &str));
    option!(ReportDownstreamTime, report_downstream_time(enable: bool));
    option!(ReportPerfBenchmark, report_perf_benchmark(enable: bool));
    option!(SkipSPIRVValidation, skip_spirv_validation(enable: bool));

    // Target
    #[inline(always)]
    pub fn capability(self, capability: CapabilityID) -> Self {
        self.push_ints(CompilerOptionName::Capability, capability.0 as _, 0)
    }

    option!(DefaultImageFormatUnknown, default_image_format_unknown(enable: bool));
    option!(DisableDynamicDispatch, disable_dynamic_dispatch(enable: bool));
    option!(DisableSpecialization, disable_specialization(enable: bool));
    option!(FloatingPointMode, floating_point_mode(mode: FloatingPointMode));
    option!(DebugInformation, debug_information(level: DebugInfoLevel));
    option!(LineDirectiveMode, line_directive_mode(mode: LineDirectiveMode));
    option!(Optimization, optimization(level: OptimizationLevel));
    option!(Obfuscate, obfuscate(enable: bool));

    #[inline(always)]
    pub fn vulkan_bind_shift(self, kind: u8, set: i32, shift: i32) -> Self {
        // intvalue0: higher 8 bits for kind, lower bits is set
        // intvalue1: shift
        let intvalue0 = (((kind as u32) << 24) | set as u32) as i32;
        self.push_ints(CompilerOptionName::VulkanBindShift, intvalue0, shift)
    }

    #[inline(always)]
    pub fn vulkan_bind_globals(self, index: i32, set: i32) -> Self {
        // intvalue0: index
        // intvalue1: set
        self.push_ints(CompilerOptionName::VulkanBindGlobals, index, set)
    }

    option!(VulkanInvertY, vulkan_invert_y(enable: bool));
    option!(VulkanUseDxPositionW, vulkan_use_dx_position_w(enable: bool));
    option!(VulkanUseEntryPointName, vulkan_use_entry_point_name(enable: bool));
    option!(VulkanUseGLLayout, vulkan_use_gl_layout(enable: bool));
    option!(VulkanEmitReflection, vulkan_emit_reflection(enable: bool));
    option!(GLSLForceScalarLayout, glsl_force_scalar_layout(enable: bool));
    option!(EnableEffectAnnotations, enable_effect_annotations(enable: bool));
    option!(EmitSpirvViaGLSL, emit_spirv_via_glsl(enable: bool));
    option!(EmitSpirvDirectly, emit_spirv_directly(enable: bool));
    option!(SPIRVCoreGrammarJSON, spirv_core_grammar_json(path: &str));
    option!(IncompleteLibrary, incomplete_library(enable: bool));

    #[inline(always)]
    pub fn downstream_args(self, compiler_name: &str, arguments: &[&str]) -> Self {
        // contact arguments into a single string, separated by `\n`
        let mut args = String::new();
        for arg in arguments {
            args.push_str(arg);
            args.push('\n');
        }
        self.push_str2(CompilerOptionName::DownstreamArgs, compiler_name, &args)
    }

    option!(DumpIntermediates, dump_intermediates(enable: bool));
    option!(DumpIntermediatePrefix, dump_intermediate_prefix(file_prefix: &str));

    option!(DebugInformationFormat, debug_info_format(format: DebugInfoFormat));

    #[inline(always)]
    pub fn vulkan_bind_shift_all(self, kind: i32, shift: i32) -> Self {
        self.push_ints(CompilerOptionName::VulkanBindShiftAll, kind, shift)
    }
    option!(GenerateWholeProgram, generate_whole_program(enable: bool));
    option!(UseUpToDateBinaryModule, use_up_to_date_binary_module(enable: bool));

    // Debugging
    option!(NoCodeGen, no_code_gen(enable: bool));

    // Experimental
    option!(NoMangle, no_mangle(enable: bool));
    option!(ValidateUniformity, validate_uniformity(enable: bool));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compute_shader_compiles() -> Result<()> {
        let global_session = GlobalSession::new().unwrap();
        let search_path = std::ffi::CString::new("shaders/").unwrap();
        // All compiler options are available through this builder.
        let session_options = OptionsBuilder::new()
            .optimization(OptimizationLevel::High)
            .matrix_layout_row(true);

        let target_desc = TargetDescBuilder::new()
            .format(CompileTarget::Spirv)
            .profile(global_session.find_profile("glsl_450"));

        let session_desc = SessionDescBuilder::new()
            .targets(&[*target_desc])
            .search_paths(&[search_path.as_ptr()])
            .options(&session_options);

        let session = global_session.create_session(&session_desc).unwrap();

        let module = session.load_module("testcompute").unwrap();

        let entry_point = module.find_entry_point_by_name("computeMain").unwrap();

        let program = session
            .create_composite_component_type(&[module.downcast(), entry_point.downcast()])?;

        let linked_program = program.link()?;

        let mut reflection = linked_program.get_layout(0);
        let entrypoint_count = reflection.as_mut().get_entry_point_count();
        assert_eq!(entrypoint_count, 1);

        let params_count = reflection.as_mut().get_parameter_count();
        assert_eq!(params_count, 3);

        Ok(())
    }
}
