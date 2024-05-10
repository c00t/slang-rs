use std::{pin::Pin, ptr::null};

use autocxx::prelude::*;

include_cpp! {
	#include "slang.h"
	// TODO: slang_ParameterCategory and SlangParameterCategory should be merged in slang.h in upstream.
	extern_cpp_type!("SlangParameterCategory",crate::cbind::SlangParameterCategory)
	pod!("SlangParameterCategory")
	extern_cpp_type!("SlangStage",crate::SlangStage)
	pod!("SlangStage")
	extern_cpp_type!("SlangResourceShape",crate::SlangResourceShape)
	pod!("SlangResourceShape")
	extern_cpp_type!("SlangResourceAccess",crate::SlangResourceAccess)
	pod!("SlangResourceAccess")
	extern_cpp_type!("SlangMatrixLayoutMode",crate::SlangMatrixLayoutMode)
	pod!("SlangMatrixLayoutMode")
	extern_cpp_type!("slang::ParameterCategory",crate::cbind::slang_ParameterCategory)
	pod!("slang::ParameterCategory")
	extern_cpp_type!("slang::BindingType",crate::slang_BindingType)
	pod!("slang::BindingType")
	// extern_cpp_type!("slang::TypeReflection::Kind",crate::slang_TypeReflection_Kind)
	// pod!("slang::TypeReflection::Kind")
	extern_cpp_type!("slang::LayoutRules",crate::slang_LayoutRules)
	pod!("slang::LayoutRules")
	extern_cpp_type!("SlangResult",crate::ResultCode)
	pod!("SlangResult")
	generate!("slang::ShaderReflection")
	generate!("slang::ProgramLayout")
	generate!("slang::TypeLayoutReflection")
	generate!("slang::EntryPointReflection")
	generate!("slang::VariableLayoutReflection")
	generate!("slang::VariableReflection")
	generate!("slang::UserAttribute")
	generate!("slang::TypeParameterReflection")

	generate!("slang::BufferReflection")
	generate!("slang::DeclReflection")
	generate!("slang::FunctionReflection")
}

use ffi::slang::{BufferReflection, DeclReflection, FunctionReflection};
// reexport
pub use ffi::slang::{
	EntryPointReflection, Modifier, Modifier_ID, ProgramLayout, TypeLayoutReflection,
	TypeParameterReflection, TypeReflection, UserAttribute, VariableLayoutReflection,
	VariableReflection,
};

unsafe impl cxx::ExternType for crate::cbind::slang_ParameterCategory {
	type Id = cxx::type_id!("slang::ParameterCategory");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::cbind::SlangParameterCategory {
	type Id = cxx::type_id!("SlangParameterCategory");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::SlangStage {
	type Id = cxx::type_id!("SlangStage");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::SlangResourceShape {
	type Id = cxx::type_id!("SlangResourceShape");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::SlangMatrixLayoutMode {
	type Id = cxx::type_id!("SlangMatrixLayoutMode");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::SlangResourceAccess {
	type Id = cxx::type_id!("SlangResourceAccess");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::slang_BindingType {
	type Id = cxx::type_id!("slang::BindingType");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::slang_LayoutRules {
	type Id = cxx::type_id!("slang::LayoutRules");
	type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::ResultCode {
	type Id = cxx::type_id!("SlangResult");
	type Kind = cxx::kind::Trivial;
}

impl From<ffi::slang::TypeReflection_Kind> for crate::slang_TypeReflection_Kind {
	fn from(value: ffi::slang::TypeReflection_Kind) -> Self {
		match value {
			ffi::slang::TypeReflection_Kind::None => Self::None,
			ffi::slang::TypeReflection_Kind::Struct => Self::Struct,
			ffi::slang::TypeReflection_Kind::Array => Self::Array,
			ffi::slang::TypeReflection_Kind::Matrix => Self::Matrix,
			ffi::slang::TypeReflection_Kind::Vector => Self::Vector,
			ffi::slang::TypeReflection_Kind::Scalar => Self::Scalar,
			ffi::slang::TypeReflection_Kind::ConstantBuffer => Self::ConstantBuffer,
			ffi::slang::TypeReflection_Kind::Resource => Self::Resource,
			ffi::slang::TypeReflection_Kind::SamplerState => Self::SamplerState,
			ffi::slang::TypeReflection_Kind::TextureBuffer => Self::TextureBuffer,
			ffi::slang::TypeReflection_Kind::ShaderStorageBuffer => Self::ShaderStorageBuffer,
			ffi::slang::TypeReflection_Kind::ParameterBlock => Self::ParameterBlock,
			ffi::slang::TypeReflection_Kind::GenericTypeParameter => Self::GenericTypeParameter,
			ffi::slang::TypeReflection_Kind::Interface => Self::Interface,
			ffi::slang::TypeReflection_Kind::OutputStream => Self::OutputStream,
			ffi::slang::TypeReflection_Kind::Specialized => Self::Specialized,
			ffi::slang::TypeReflection_Kind::Feedback => Self::Feedback,
			ffi::slang::TypeReflection_Kind::Pointer => Self::Pointer,
			ffi::slang::TypeReflection_Kind::DynamicResource => Self::DynamicResource,
		}
	}
}

impl From<crate::slang_TypeReflection_Kind> for ffi::slang::TypeReflection_Kind {
	fn from(value: crate::slang_TypeReflection_Kind) -> Self {
		match value {
			crate::slang_TypeReflection_Kind::None => Self::None,
			crate::slang_TypeReflection_Kind::Struct => Self::Struct,
			crate::slang_TypeReflection_Kind::Array => Self::Array,
			crate::slang_TypeReflection_Kind::Matrix => Self::Matrix,
			crate::slang_TypeReflection_Kind::Vector => Self::Vector,
			crate::slang_TypeReflection_Kind::Scalar => Self::Scalar,
			crate::slang_TypeReflection_Kind::ConstantBuffer => Self::ConstantBuffer,
			crate::slang_TypeReflection_Kind::Resource => Self::Resource,
			crate::slang_TypeReflection_Kind::SamplerState => Self::SamplerState,
			crate::slang_TypeReflection_Kind::TextureBuffer => Self::TextureBuffer,
			crate::slang_TypeReflection_Kind::ShaderStorageBuffer => Self::ShaderStorageBuffer,
			crate::slang_TypeReflection_Kind::ParameterBlock => Self::ParameterBlock,
			crate::slang_TypeReflection_Kind::GenericTypeParameter => Self::GenericTypeParameter,
			crate::slang_TypeReflection_Kind::Interface => Self::Interface,
			crate::slang_TypeReflection_Kind::OutputStream => Self::OutputStream,
			crate::slang_TypeReflection_Kind::Specialized => Self::Specialized,
			crate::slang_TypeReflection_Kind::Feedback => Self::Feedback,
			crate::slang_TypeReflection_Kind::Pointer => Self::Pointer,
			crate::slang_TypeReflection_Kind::DynamicResource => Self::DynamicResource,
		}
	}
}

impl From<ffi::slang::TypeReflection_ScalarType> for crate::slang_TypeReflection_ScalarType {
	fn from(value: ffi::slang::TypeReflection_ScalarType) -> Self {
		match value {
			ffi::slang::TypeReflection_ScalarType::None => Self::None,
			ffi::slang::TypeReflection_ScalarType::Void => Self::Void,
			ffi::slang::TypeReflection_ScalarType::Bool => Self::Bool,
			ffi::slang::TypeReflection_ScalarType::Int32 => Self::Int32,
			ffi::slang::TypeReflection_ScalarType::UInt32 => Self::UInt32,
			ffi::slang::TypeReflection_ScalarType::Int64 => Self::Int64,
			ffi::slang::TypeReflection_ScalarType::UInt64 => Self::UInt64,
			ffi::slang::TypeReflection_ScalarType::Float16 => Self::Float16,
			ffi::slang::TypeReflection_ScalarType::Float32 => Self::Float32,
			ffi::slang::TypeReflection_ScalarType::Float64 => Self::Float64,
			ffi::slang::TypeReflection_ScalarType::Int8 => Self::Int8,
			ffi::slang::TypeReflection_ScalarType::UInt8 => Self::UInt8,
			ffi::slang::TypeReflection_ScalarType::Int16 => Self::Int16,
			ffi::slang::TypeReflection_ScalarType::UInt16 => Self::UInt16,
		}
	}
}

impl From<crate::slang_TypeReflection_ScalarType> for ffi::slang::TypeReflection_ScalarType {
	fn from(value: crate::slang_TypeReflection_ScalarType) -> Self {
		match value {
			crate::slang_TypeReflection_ScalarType::None => Self::None,
			crate::slang_TypeReflection_ScalarType::Void => Self::Void,
			crate::slang_TypeReflection_ScalarType::Bool => Self::Bool,
			crate::slang_TypeReflection_ScalarType::Int32 => Self::Int32,
			crate::slang_TypeReflection_ScalarType::UInt32 => Self::UInt32,
			crate::slang_TypeReflection_ScalarType::Int64 => Self::Int64,
			crate::slang_TypeReflection_ScalarType::UInt64 => Self::UInt64,
			crate::slang_TypeReflection_ScalarType::Float16 => Self::Float16,
			crate::slang_TypeReflection_ScalarType::Float32 => Self::Float32,
			crate::slang_TypeReflection_ScalarType::Float64 => Self::Float64,
			crate::slang_TypeReflection_ScalarType::Int8 => Self::Int8,
			crate::slang_TypeReflection_ScalarType::UInt8 => Self::UInt8,
			crate::slang_TypeReflection_ScalarType::Int16 => Self::Int16,
			crate::slang_TypeReflection_ScalarType::UInt16 => Self::UInt16,
		}
	}
}

/// Shader reflection
///
/// # CAUTION
///
/// 1. `getSession` is not implemented as safe method.
/// 2. `get` is not implemented as safe method, because `SlangCompileRequest` is deprecated.
/// 3. `specializeType` is not implemented, because `autocxx` does not support double pointer.
impl ProgramLayout {
	pub fn get_parameter_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getParameterCount() }.0.into()
	}

	pub fn get_type_parameter_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getTypeParameterCount() }.0.into()
	}

	pub fn get_type_parameter_by_index(
		self: Pin<&mut Self>,
		index: u32,
	) -> Option<Pin<&mut TypeParameterReflection>> {
		let result = unsafe { self.getTypeParameterByIndex(c_uint(index.into())) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { Pin::new_unchecked(&mut *result) })
		}
	}

	/// Find a type parameter by name
	///
	/// # CAUTION
	///
	/// Because `name` literal is exchanged between Rust and C++(nul-terminated), there is an allocation overhead.
	pub fn find_type_parameter(
		self: Pin<&mut Self>,
		name: &str,
	) -> Option<Pin<&mut TypeParameterReflection>> {
		let name = std::ffi::CString::new(name).unwrap();
		let result = unsafe { self.findTypeParameter(name.as_ptr()) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { Pin::new_unchecked(&mut *result) })
		}
	}

	pub fn get_parameter_by_index(
		self: Pin<&mut Self>,
		index: u32,
	) -> Option<Pin<&mut VariableLayoutReflection>> {
		let result = unsafe { self.getParameterByIndex(c_uint(index.into())) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { Pin::new_unchecked(&mut *result) })
		}
	}

	pub fn get_entry_point_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getEntryPointCount() }
	}

	pub fn get_entry_point_by_index(
		self: Pin<&mut Self>,
		index: u64,
	) -> Option<Pin<&mut EntryPointReflection>> {
		let result = unsafe { self.getEntryPointByIndex(index) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { Pin::new_unchecked(&mut *result) })
		}
	}

	pub fn get_global_constant_buffer_binding(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getGlobalConstantBufferBinding() }
	}

	pub fn get_global_constant_buffer_size(self: Pin<&mut Self>) -> usize {
		unsafe { self.getGlobalConstantBufferSize() }
	}

	/// Find a type by name
	///
	/// # CAUTION
	///
	/// Because `name` literal is exchanged between Rust and C++(nul-terminated), there is an allocation overhead.
	pub fn find_type_by_name(self: Pin<&mut Self>, name: &str) -> Option<Pin<&mut TypeReflection>> {
		let name = std::ffi::CString::new(name).unwrap();
		let result = unsafe { self.findTypeByName(name.as_ptr()) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { Pin::new_unchecked(&mut *result) })
		}
	}

	pub fn get_type_layout(
		self: Pin<&mut Self>,
		typeref: Pin<&mut TypeReflection>,
		rules: crate::slang_LayoutRules,
	) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getTypeLayout(typeref.get_unchecked_mut(), rules)) }
	}

	pub fn find_entry_point_by_name(
		self: Pin<&mut Self>,
		name: &str,
	) -> Option<Pin<&mut EntryPointReflection>> {
		let name = std::ffi::CString::new(name).unwrap();
		let result = unsafe { self.findEntryPointByName(name.as_ptr()) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { Pin::new_unchecked(&mut *result) })
		}
	}

	pub fn get_hashed_string_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getHashedStringCount() }
	}

	pub fn get_hashed_string(self: Pin<&mut Self>, index: u64) -> &str {
		unsafe {
			let mut count = 0;
			let name = self.getHashedString(index, &mut count);
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_global_params_type_layout(self: Pin<&mut Self>) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getGlobalParamsTypeLayout()) }
	}

	pub fn get_global_params_var_layout(
		self: Pin<&mut Self>,
	) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getGlobalParamsVarLayout()) }
	}
}

impl TypeLayoutReflection {
	pub fn get_type(self: Pin<&mut Self>) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getType()) }
	}

	pub fn get_kind(self: Pin<&mut Self>) -> crate::slang_TypeReflection_Kind {
		unsafe { self.getKind() }.into()
	}

	pub fn get_size(self: Pin<&mut Self>, category: crate::ParameterCategory) -> usize {
		unsafe { self.getSize(category.into()) }
	}

	pub fn get_stride(self: Pin<&mut Self>, category: crate::ParameterCategory) -> usize {
		unsafe { self.getStride(category.into()) }
	}

	pub fn get_alignment(self: Pin<&mut Self>, category: crate::ParameterCategory) -> i32 {
		unsafe { self.getAlignment(category.into()) }
	}

	pub fn get_field_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getFieldCount() }.0.into()
	}

	pub fn get_field_by_index(
		self: Pin<&mut Self>,
		index: u32,
	) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getFieldByIndex(c_uint(index.into()))) }
	}

	pub fn find_field_index_by_name(self: Pin<&mut Self>, name: &str) -> i64 {
		let name = std::ffi::CString::new(name).unwrap();
		unsafe { self.findFieldIndexByName(name.as_ptr(), null()) }
	}

	pub fn get_explicit_counter(self: Pin<&mut Self>) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getExplicitCounter()) }
	}

	pub fn is_array(self: Pin<&mut Self>) -> bool {
		unsafe { self.isArray() }
	}

	pub fn unwrap_array(self: Pin<&mut Self>) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.unwrapArray()) }
	}

	pub fn get_element_count(self: Pin<&mut Self>) -> usize {
		unsafe { self.getElementCount() }
	}

	pub fn get_total_array_element_count(self: Pin<&mut Self>) -> usize {
		unsafe { self.getTotalArrayElementCount() }
	}

	pub fn get_element_stride(self: Pin<&mut Self>, category: crate::ParameterCategory) -> usize {
		unsafe { self.getElementStride(category.into()) }
	}

	pub fn get_element_type_layout(self: Pin<&mut Self>) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getElementTypeLayout()) }
	}

	pub fn get_element_var_layout(self: Pin<&mut Self>) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getElementVarLayout()) }
	}

	pub fn get_container_var_layout(self: Pin<&mut Self>) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getContainerVarLayout()) }
	}

	pub fn get_parameter_category(self: Pin<&mut Self>) -> crate::ParameterCategory {
		unsafe { self.getParameterCategory() }
	}

	pub fn get_category_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getCategoryCount() }.0.into()
	}

	pub fn get_category_by_index(self: Pin<&mut Self>, index: u32) -> crate::ParameterCategory {
		unsafe { self.getCategoryByIndex(c_uint(index.into())) }
	}

	pub fn get_row_count(self: Pin<&mut Self>) -> u32 {
		unsafe { self.getRowCount() }.0.into()
	}

	pub fn get_column_count(self: Pin<&mut Self>) -> u32 {
		unsafe { self.getColumnCount() }.0.into()
	}

	pub fn get_scalar_type(self: Pin<&mut Self>) -> crate::slang_TypeReflection_ScalarType {
		unsafe { self.getScalarType() }.into()
	}

	pub fn get_resource_result_type(self: Pin<&mut Self>) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getResourceResultType()) }
	}

	pub fn get_resource_shape(self: Pin<&mut Self>) -> crate::SlangResourceShape {
		unsafe { self.getResourceShape() }
	}

	pub fn get_resource_access(self: Pin<&mut Self>) -> crate::SlangResourceAccess {
		unsafe { self.getResourceAccess() }
	}

	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_matrix_layout_mode(self: Pin<&mut Self>) -> crate::SlangMatrixLayoutMode {
		unsafe { self.getMatrixLayoutMode() }
	}

	pub fn get_generic_param_index(self: Pin<&mut Self>) -> i32 {
		unsafe { self.getGenericParamIndex() }.0.into()
	}

	pub fn get_pending_data_type_layout(self: Pin<&mut Self>) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getPendingDataTypeLayout()) }
	}

	pub fn get_specialized_type_pending_data_var_layout(
		self: Pin<&mut Self>,
	) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getSpecializedTypePendingDataVarLayout()) }
	}

	pub fn get_binding_range_count(self: Pin<&mut Self>) -> i64 {
		unsafe { self.getBindingRangeCount() }
	}

	pub fn get_binding_range_type(self: Pin<&mut Self>, index: i64) -> crate::slang_BindingType {
		unsafe { self.getBindingRangeType(index) }
	}

	pub fn is_binding_range_specializable(self: Pin<&mut Self>, index: i64) -> bool {
		unsafe { self.isBindingRangeSpecializable(index) }
	}

	pub fn get_binding_range_binding_count(self: Pin<&mut Self>, index: i64) -> i64 {
		unsafe { self.getBindingRangeBindingCount(index) }
	}

	pub fn get_field_binding_range_offset(self: Pin<&mut Self>, fieldIndex: i64) -> i64 {
		unsafe { self.getFieldBindingRangeOffset(fieldIndex) }
	}

	pub fn get_explicit_counter_binding_range_offset(self: Pin<&mut Self>) -> i64 {
		unsafe { self.getExplicitCounterBindingRangeOffset() }
	}

	pub fn get_binding_range_leaf_type_layout(
		self: Pin<&mut Self>,
		index: i64,
	) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getBindingRangeLeafTypeLayout(index)) }
	}

	pub fn get_binding_range_leaf_variable(
		self: Pin<&mut Self>,
		index: i64,
	) -> Pin<&mut VariableReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getBindingRangeLeafVariable(index)) }
	}

	pub fn get_binding_range_descriptor_set_index(self: Pin<&mut Self>, index: i64) -> i64 {
		unsafe { self.getBindingRangeDescriptorSetIndex(index) }
	}

	pub fn get_binding_range_first_descriptor_range_index(self: Pin<&mut Self>, index: i64) -> i64 {
		unsafe { self.getBindingRangeFirstDescriptorRangeIndex(index) }
	}

	pub fn get_binding_range_descriptor_range_count(self: Pin<&mut Self>, index: i64) -> i64 {
		unsafe { self.getBindingRangeDescriptorRangeCount(index) }
	}

	pub fn get_descriptor_set_count(self: Pin<&mut Self>) -> i64 {
		unsafe { self.getDescriptorSetCount() }
	}

	pub fn get_descriptor_set_space_offset(self: Pin<&mut Self>, set_index: i64) -> i64 {
		unsafe { self.getDescriptorSetSpaceOffset(set_index) }
	}

	pub fn get_descriptor_set_descriptor_range_count(self: Pin<&mut Self>, set_index: i64) -> i64 {
		unsafe { self.getDescriptorSetDescriptorRangeCount(set_index) }
	}

	pub fn get_descriptor_set_descriptor_range_index_offset(
		self: Pin<&mut Self>,
		set_index: i64,
		range_index: i64,
	) -> i64 {
		unsafe { self.getDescriptorSetDescriptorRangeIndexOffset(set_index, range_index) }
	}

	pub fn get_descriptor_set_descriptor_range_descriptor_count(
		self: Pin<&mut Self>,
		set_index: i64,
		range_index: i64,
	) -> i64 {
		unsafe { self.getDescriptorSetDescriptorRangeDescriptorCount(set_index, range_index) }
	}

	pub fn get_descriptor_set_descriptor_range_type(
		self: Pin<&mut Self>,
		set_index: i64,
		range_index: i64,
	) -> crate::slang_BindingType {
		unsafe { self.getDescriptorSetDescriptorRangeType(set_index, range_index) }
	}

	pub fn get_descriptor_set_descriptor_range_category(
		self: Pin<&mut Self>,
		set_index: i64,
		range_index: i64,
	) -> crate::ParameterCategory {
		unsafe { self.getDescriptorSetDescriptorRangeCategory(set_index, range_index) }
	}

	pub fn get_subobject_range_count(self: Pin<&mut Self>) -> i64 {
		unsafe { self.getSubObjectRangeCount() }
	}

	pub fn get_subobject_range_binding_range_index(self: Pin<&mut Self>, range_index: i64) -> i64 {
		unsafe { self.getSubObjectRangeBindingRangeIndex(range_index) }
	}

	pub fn get_subobject_range_space_offset(self: Pin<&mut Self>, range_index: i64) -> i64 {
		unsafe { self.getSubObjectRangeSpaceOffset(range_index) }
	}

	pub fn get_subobject_range_offset(
		self: Pin<&mut Self>,
		range_index: i64,
	) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getSubObjectRangeOffset(range_index)) }
	}
}

impl EntryPointReflection {
	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_name_override(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getNameOverride();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_parameter_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getParameterCount() }.0.into()
	}

	pub fn get_parameter_by_index(
		self: Pin<&mut Self>,
		index: u32,
	) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getParameterByIndex(c_uint(index.into()))) }
	}

	pub fn get_stage(self: Pin<&mut Self>) -> crate::SlangStage {
		unsafe { self.getStage() }
	}

	pub fn get_compute_threadgroup_size<const COUNT: usize>(self: Pin<&mut Self>) -> [u64; COUNT] {
		let mut axis = [0u64; COUNT];
		unsafe { self.getComputeThreadGroupSize(COUNT as u64, axis.as_mut_ptr()) };
		axis
	}

	pub fn get_compute_wave_size(self: Pin<&mut Self>) -> u64 {
		let mut wave_size = 0;
		unsafe {
			self.getComputeWaveSize(&mut wave_size);
		}
		wave_size
	}

	pub fn uses_any_sample_rate_input(self: Pin<&mut Self>) -> bool {
		unsafe { self.usesAnySampleRateInput() }
	}

	pub fn get_var_layout(self: Pin<&mut Self>) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getVarLayout()) }
	}

	pub fn get_type_layout(self: Pin<&mut Self>) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getTypeLayout()) }
	}

	pub fn get_result_var_layout(self: Pin<&mut Self>) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getResultVarLayout()) }
	}

	pub fn has_default_constant_buffer(self: Pin<&mut Self>) -> bool {
		unsafe { self.hasDefaultConstantBuffer() }
	}
}

/// Variable layout's reflection
///
/// # CAUTION
///
/// `findModifier` is not implemented as safe method, because `Modifier` is an opaque type.
impl VariableLayoutReflection {
	pub fn get_variable(self: Pin<&mut Self>) -> Pin<&mut VariableReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getVariable()) }
	}

	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_type_layout(self: Pin<&mut Self>) -> Pin<&mut TypeLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getTypeLayout()) }
	}

	pub fn get_category(self: Pin<&mut Self>) -> crate::ParameterCategory {
		unsafe { self.getCategory() }
	}

	pub fn get_category_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getCategoryCount() }.0.into()
	}

	pub fn get_category_by_index(self: Pin<&mut Self>, index: u32) -> crate::ParameterCategory {
		unsafe { self.getCategoryByIndex(c_uint(index.into())) }
	}

	pub fn get_offset(self: Pin<&mut Self>, category: crate::ParameterCategory) -> usize {
		unsafe { self.getOffset(category.into()) }
	}

	pub fn get_type(self: Pin<&mut Self>) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getType()) }
	}

	pub fn get_binding_index(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getBindingIndex() }.0.into()
	}

	pub fn get_binding_space(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getBindingSpace() }.0.into()
	}

	pub fn get_binding_space_with_category(
		self: Pin<&mut Self>,
		category: crate::ParameterCategory,
	) -> usize {
		unsafe { self.getBindingSpace1(category.into()) }
	}

	pub fn get_semantic_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getSemanticName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_semantic_index(self: Pin<&mut Self>) -> usize {
		unsafe { self.getSemanticIndex() }
	}

	pub fn get_stage(self: Pin<&mut Self>) -> crate::SlangStage {
		unsafe { self.getStage() }
	}

	pub fn get_pending_data_layout(self: Pin<&mut Self>) -> Pin<&mut VariableLayoutReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getPendingDataLayout()) }
	}
}

impl VariableReflection {
	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_type(self: Pin<&mut Self>) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getType()) }
	}

	pub fn get_user_attribute_count(self: Pin<&mut Self>) -> u64 {
		unsafe { self.getUserAttributeCount() }.0.into()
	}

	pub fn get_user_attribute_by_index(
		self: Pin<&mut Self>,
		index: u32,
	) -> Pin<&mut UserAttribute> {
		unsafe { Pin::new_unchecked(&mut *self.getUserAttributeByIndex(c_uint(index.into()))) }
	}
}

impl UserAttribute {
	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_argument_count(self: Pin<&mut Self>) -> u32 {
		unsafe { self.getArgumentCount() }
	}

	pub fn get_argument_type(self: Pin<&mut Self>, index: u32) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getArgumentType(index)) }
	}

	pub fn get_argument_value_int(self: Pin<&mut Self>, index: u32) -> Option<i32> {
		let mut out = autocxx::c_int(0);
		let result = unsafe { self.getArgumentValueInt(index, &mut out) };
		if result.is_succeeded() {
			Some(out.0)
		} else {
			None
		}
	}

	pub fn get_argument_value_float(self: Pin<&mut Self>, index: u32) -> Option<f32> {
		let mut out = 0.0;
		let result = unsafe { self.getArgumentValueFloat(index, &mut out) };
		if result.is_succeeded() {
			Some(out)
		} else {
			None
		}
	}

	pub fn get_argument_value_string(self: Pin<&mut Self>, index: u32) -> Option<&str> {
		let mut count = 0;
		let result = unsafe { self.getArgumentValueString(index, &mut count) };
		if result.is_null() {
			None
		} else {
			Some(unsafe { std::ffi::CStr::from_ptr(result).to_str().unwrap() })
		}
	}
}

impl TypeParameterReflection {
	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_index(self: Pin<&mut Self>) -> u32 {
		unsafe { self.getIndex() }.0.into()
	}

	pub fn get_constraint_count(self: Pin<&mut Self>) -> u32 {
		unsafe { self.getConstraintCount() }.0.into()
	}

	pub fn get_constraint_by_index(self: Pin<&mut Self>, index: i32) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getConstraintByIndex(c_int(index.into()))) }
	}
}

impl BufferReflection {

}

impl From<ffi::slang::DeclReflection_Kind> for crate::slang_DeclReflection_Kind {
	fn from(value: ffi::slang::DeclReflection_Kind) -> Self {
		match value {
			ffi::slang::DeclReflection_Kind::Unsupported => Self::Unsupported,
			ffi::slang::DeclReflection_Kind::Struct => Self::Struct,
			ffi::slang::DeclReflection_Kind::Func => Self::Func,
			ffi::slang::DeclReflection_Kind::Module => Self::Module,
			ffi::slang::DeclReflection_Kind::Generic => Self::Generic,
			ffi::slang::DeclReflection_Kind::Variable => Self::Variable,
			ffi::slang::DeclReflection_Kind::Namespace => Self::Namespace,
		}
	}
}

impl DeclReflection {
	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}

	pub fn get_kind(self: Pin<&mut Self>) -> crate::slang_DeclReflection_Kind {
		unsafe { self.getKind() }.into()
	}

	pub fn get_children_count(self: Pin<&mut Self>) -> u32 {
		unsafe { self.getChildrenCount() }.0.into()
	}

	pub fn get_child(self: Pin<&mut Self>, index: u32) -> Pin<&mut DeclReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getChild(c_uint(index.into()))) }
	}

	pub fn get_type(self: Pin<&mut Self>) -> Pin<&mut TypeReflection> {
		unsafe { Pin::new_unchecked(&mut *self.getType()) }
	}

	pub fn as_variable(self: Pin<&mut Self>) -> Pin<&mut VariableReflection> {
		unsafe { Pin::new_unchecked(&mut *self.asVariable()) }
	}

	
}

impl FunctionReflection {
	pub fn get_name(self: Pin<&mut Self>) -> &str {
		unsafe {
			let name = self.getName();
			std::ffi::CStr::from_ptr(name).to_str().unwrap()
		}
	}
}