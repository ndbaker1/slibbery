#![allow(unused_imports)]
#![allow(nonstandard_style)]
use libc::*;

pub mod types;
pub use types::*;

pub mod dl;
pub use dl::*;

// Generated function stubs
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceProfileInfoV(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: c_uint,
    arg3: *mut nvmlComputeInstanceProfileInfo_v2_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetComputeInstanceProfileInfoV: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: c_uint,
        arg3: *mut nvmlComputeInstanceProfileInfo_v2_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpuInstanceGetComputeInstanceProfileInfoV"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetComputeInstanceProfileInfoV");
    nvmlGpuInstanceGetComputeInstanceProfileInfoV(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRowRemapperHistogram(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlRowRemapperHistogramValues_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetRowRemapperHistogram: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlRowRemapperHistogramValues_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRowRemapperHistogram"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRowRemapperHistogram");
    nvmlDeviceGetRowRemapperHistogram(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetGpuOperationMode(
    arg0: nvmlDevice_t,
    arg1: nvmlGpuOperationMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetGpuOperationMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlGpuOperationMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetGpuOperationMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetGpuOperationMode");
    nvmlDeviceSetGpuOperationMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetEncoderCapacity(
    arg0: nvmlDevice_t,
    arg1: nvmlEncoderType_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetEncoderCapacity: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEncoderType_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetEncoderCapacity"));
    eprintln!("[CALL] {}", "nvmlDeviceGetEncoderCapacity");
    nvmlDeviceGetEncoderCapacity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetPowerManagementLimit(
    arg0: nvmlDevice_t,
    arg1: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetPowerManagementLimit: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetPowerManagementLimit"));
    eprintln!("[CALL] {}", "nvmlDeviceSetPowerManagementLimit");
    nvmlDeviceSetPowerManagementLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetGpcClkVfOffset(
    arg0: nvmlDevice_t,
    arg1: c_int,
) -> nvmlReturn_t {
    let nvmlDeviceSetGpcClkVfOffset: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetGpcClkVfOffset"));
    eprintln!("[CALL] {}", "nvmlDeviceSetGpcClkVfOffset");
    nvmlDeviceSetGpcClkVfOffset(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetFrameRateLimit(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetFrameRateLimit: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetFrameRateLimit"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetFrameRateLimit");
    nvmlVgpuTypeGetFrameRateLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTargetFanSpeed(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetTargetFanSpeed: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTargetFanSpeed"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTargetFanSpeed");
    nvmlDeviceGetTargetFanSpeed(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeKeyRotationThresholdInfo(
    arg0: *mut nvmlConfComputeGetKeyRotationThresholdInfo_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeKeyRotationThresholdInfo: extern "C" fn(
        arg0: *mut nvmlConfComputeGetKeyRotationThresholdInfo_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetConfComputeKeyRotationThresholdInfo"));
    eprintln!(
        "[CALL] {}",
        "nvmlSystemGetConfComputeKeyRotationThresholdInfo"
    );
    nvmlSystemGetConfComputeKeyRotationThresholdInfo(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFrameRateLimit(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetFrameRateLimit: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetFrameRateLimit"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetFrameRateLimit");
    nvmlVgpuInstanceGetFrameRateLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures_v4(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGridLicensableFeatures_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGridLicensableFeatures_v4: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGridLicensableFeatures_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures_v4"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures_v4");
    nvmlDeviceGetGridLicensableFeatures_v4(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPerformanceState(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPstates_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPerformanceState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPstates_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPerformanceState"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPerformanceState");
    nvmlDeviceGetPerformanceState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetCudaDriverVersion(arg0: *mut c_int) -> nvmlReturn_t {
    let nvmlSystemGetCudaDriverVersion: extern "C" fn(arg0: *mut c_int) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetCudaDriverVersion"));
    eprintln!("[CALL] {}", "nvmlSystemGetCudaDriverVersion");
    nvmlSystemGetCudaDriverVersion(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedGraphicsClocks(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_uint,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedGraphicsClocks: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_uint,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedGraphicsClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedGraphicsClocks");
    nvmlDeviceGetSupportedGraphicsClocks(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceClearAccountingPids(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceClearAccountingPids: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceClearAccountingPids"));
    eprintln!("[CALL] {}", "nvmlDeviceClearAccountingPids");
    nvmlDeviceClearAccountingPids(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetGpuLockedClocks(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceResetGpuLockedClocks: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceResetGpuLockedClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceResetGpuLockedClocks");
    nvmlDeviceResetGpuLockedClocks(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByPciBusId_v2(
    arg0: *const c_char,
    arg1: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetHandleByPciBusId_v2: extern "C" fn(
        arg0: *const c_char,
        arg1: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetHandleByPciBusId_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleByPciBusId_v2");
    nvmlDeviceGetHandleByPciBusId_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGspFirmwareMode(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetGspFirmwareMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGspFirmwareMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGspFirmwareMode");
    nvmlDeviceGetGspFirmwareMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceIsMigDeviceHandle(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceIsMigDeviceHandle: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceIsMigDeviceHandle"));
    eprintln!("[CALL] {}", "nvmlDeviceIsMigDeviceHandle");
    nvmlDeviceIsMigDeviceHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetNvlinkBwMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlNvlinkSetBwMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetNvlinkBwMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlNvlinkSetBwMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetNvlinkBwMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetNvlinkBwMode");
    nvmlDeviceSetNvlinkBwMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetVmDriverVersion(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetVmDriverVersion: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetVmDriverVersion"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetVmDriverVersion");
    nvmlVgpuInstanceGetVmDriverVersion(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetDriverVersion(
    arg0: *mut c_char,
    arg1: c_uint,
) -> nvmlReturn_t {
    eprintln!("[CALL] {}", "nvmlSystemGetDriverVersion");
    std::ptr::copy_nonoverlapping("590-fake\0".as_ptr() as *const c_char, arg0, arg1 as _);
    0
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTotalEccErrors(
    arg0: nvmlDevice_t,
    arg1: nvmlMemoryErrorType_t,
    arg2: nvmlEccCounterType_t,
    arg3: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetTotalEccErrors: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlMemoryErrorType_t,
        arg2: nvmlEccCounterType_t,
        arg3: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTotalEccErrors"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTotalEccErrors");
    nvmlDeviceGetTotalEccErrors(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstances(
    arg0: nvmlDevice_t,
    arg1: nvmlVgpuTypeId_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetMaxInstances: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlVgpuTypeId_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetMaxInstances"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetMaxInstances");
    nvmlVgpuTypeGetMaxInstances(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetCapabilities(
    arg0: nvmlVgpuTypeId_t,
    arg1: nvmlVgpuCapability_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetCapabilities: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: nvmlVgpuCapability_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetCapabilities"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetCapabilities");
    nvmlVgpuTypeGetCapabilities(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetDevices(
    arg0: nvmlUnit_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlUnitGetDevices: extern "C" fn(
        arg0: nvmlUnit_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetDevices"));
    eprintln!("[CALL] {}", "nvmlUnitGetDevices");
    nvmlUnitGetDevices(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetIrqNum(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetIrqNum: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetIrqNum"));
    eprintln!("[CALL] {}", "nvmlDeviceGetIrqNum");
    nvmlDeviceGetIrqNum(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetEncoderStats(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetEncoderStats: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetEncoderStats"));
    eprintln!("[CALL] {}", "nvmlDeviceGetEncoderStats");
    nvmlDeviceGetEncoderStats(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetApplicationsClocks(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceResetApplicationsClocks: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceResetApplicationsClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceResetApplicationsClocks");
    nvmlDeviceResetApplicationsClocks(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeGpusReadyState(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeGpusReadyState: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetConfComputeGpusReadyState"));
    eprintln!("[CALL] {}", "nvmlSystemGetConfComputeGpusReadyState");
    nvmlSystemGetConfComputeGpusReadyState(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetConfComputeUnprotectedMemSize(
    arg0: nvmlDevice_t,
    arg1: c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceSetConfComputeUnprotectedMemSize: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetConfComputeUnprotectedMemSize"));
    eprintln!("[CALL] {}", "nvmlDeviceSetConfComputeUnprotectedMemSize");
    nvmlDeviceSetConfComputeUnprotectedMemSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetCudaDriverVersion_v2(arg0: *mut c_int) -> nvmlReturn_t {
    eprintln!("[CALL] {}", "nvmlSystemGetCudaDriverVersion_v2");
    arg0.write(1320);
    0
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementLimit(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerManagementLimit: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerManagementLimit"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerManagementLimit");
    nvmlDeviceGetPowerManagementLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDefaultApplicationsClock(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetDefaultApplicationsClock: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDefaultApplicationsClock"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDefaultApplicationsClock");
    nvmlDeviceGetDefaultApplicationsClock(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCurrPcieLinkWidth(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetCurrPcieLinkWidth: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCurrPcieLinkWidth"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCurrPcieLinkWidth");
    nvmlDeviceGetCurrPcieLinkWidth(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetVgpuCompatibility(
    arg0: *mut nvmlVgpuMetadata_t,
    arg1: *mut nvmlVgpuPgpuMetadata_t,
    arg2: *mut nvmlVgpuPgpuCompatibility_t,
) -> nvmlReturn_t {
    let nvmlGetVgpuCompatibility: extern "C" fn(
        arg0: *mut nvmlVgpuMetadata_t,
        arg1: *mut nvmlVgpuPgpuMetadata_t,
        arg2: *mut nvmlVgpuPgpuCompatibility_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGetVgpuCompatibility"));
    eprintln!("[CALL] {}", "nvmlGetVgpuCompatibility");
    nvmlGetVgpuCompatibility(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementLimitConstraints(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerManagementLimitConstraints: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetPowerManagementLimitConstraints"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerManagementLimitConstraints");
    nvmlDeviceGetPowerManagementLimitConstraints(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetMigMode(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlReturn_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetMigMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlReturn_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetMigMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetMigMode");
    nvmlDeviceSetMigMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSampleFree(arg0: nvmlGpmSample_t) -> nvmlReturn_t {
    let nvmlGpmSampleFree: extern "C" fn(arg0: nvmlGpmSample_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmSampleFree"));
    eprintln!("[CALL] {}", "nvmlGpmSampleFree");
    nvmlGpmSampleFree(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRunningProcessDetailList(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlProcessDetailList_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetRunningProcessDetailList: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlProcessDetailList_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRunningProcessDetailList"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRunningProcessDetailList");
    nvmlDeviceGetRunningProcessDetailList(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetLedState(
    arg0: nvmlUnit_t,
    arg1: *mut nvmlLedState_t,
) -> nvmlReturn_t {
    let nvmlUnitGetLedState: extern "C" fn(
        arg0: nvmlUnit_t,
        arg1: *mut nvmlLedState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetLedState"));
    eprintln!("[CALL] {}", "nvmlUnitGetLedState");
    nvmlUnitGetLedState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemEventSetWait(
    arg0: *mut nvmlSystemEventSetWaitRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemEventSetWait: extern "C" fn(
        arg0: *mut nvmlSystemEventSetWaitRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemEventSetWait"));
    eprintln!("[CALL] {}", "nvmlSystemEventSetWait");
    nvmlSystemEventSetWait(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceProfileInfoV(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstanceProfileInfo_v2_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstanceProfileInfoV: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstanceProfileInfo_v2_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstanceProfileInfoV"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstanceProfileInfoV");
    nvmlDeviceGetGpuInstanceProfileInfoV(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetInforomConfigurationChecksum(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetInforomConfigurationChecksum: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetInforomConfigurationChecksum"));
    eprintln!("[CALL] {}", "nvmlDeviceGetInforomConfigurationChecksum");
    nvmlDeviceGetInforomConfigurationChecksum(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetTemperatureThreshold(
    arg0: nvmlDevice_t,
    arg1: nvmlTemperatureThresholds_t,
    arg2: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceSetTemperatureThreshold: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlTemperatureThresholds_t,
        arg2: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetTemperatureThreshold"));
    eprintln!("[CALL] {}", "nvmlDeviceSetTemperatureThreshold");
    nvmlDeviceSetTemperatureThreshold(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDriverModel_v2(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDriverModel_t,
    arg2: *mut nvmlDriverModel_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDriverModel_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDriverModel_t,
        arg2: *mut nvmlDriverModel_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDriverModel_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDriverModel_v2");
    nvmlDeviceGetDriverModel_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkCapability(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: nvmlNvLinkCapability_t,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkCapability: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: nvmlNvLinkCapability_t,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkCapability"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkCapability");
    nvmlDeviceGetNvLinkCapability(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmQueryIfStreamingEnabled(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlGpmQueryIfStreamingEnabled: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpmQueryIfStreamingEnabled"));
    eprintln!("[CALL] {}", "nvmlGpmQueryIfStreamingEnabled");
    nvmlGpmQueryIfStreamingEnabled(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemoryErrorCounter(
    arg0: nvmlDevice_t,
    arg1: nvmlMemoryErrorType_t,
    arg2: nvmlEccCounterType_t,
    arg3: nvmlMemoryLocation_t,
    arg4: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemoryErrorCounter: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlMemoryErrorType_t,
        arg2: nvmlEccCounterType_t,
        arg3: nvmlMemoryLocation_t,
        arg4: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemoryErrorCounter"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemoryErrorCounter");
    nvmlDeviceGetMemoryErrorCounter(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemEventSetCreate(
    arg0: *mut nvmlSystemEventSetCreateRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemEventSetCreate: extern "C" fn(
        arg0: *mut nvmlSystemEventSetCreateRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemEventSetCreate"));
    eprintln!("[CALL] {}", "nvmlSystemEventSetCreate");
    nvmlSystemEventSetCreate(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEncoderSessions(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlEncoderSessionInfo_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetEncoderSessions: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlEncoderSessionInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetEncoderSessions"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetEncoderSessions");
    nvmlVgpuInstanceGetEncoderSessions(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetEccMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
    arg2: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetEccMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
        arg2: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetEccMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetEccMode");
    nvmlDeviceGetEccMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmMigSampleGet(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: nvmlGpmSample_t,
) -> nvmlReturn_t {
    let nvmlGpmMigSampleGet: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: nvmlGpmSample_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpmMigSampleGet"));
    eprintln!("[CALL] {}", "nvmlGpmMigSampleGet");
    nvmlGpmMigSampleGet(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetPlacementId(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlVgpuPlacementId_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetPlacementId: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlVgpuPlacementId_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetPlacementId"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetPlacementId");
    nvmlVgpuInstanceGetPlacementId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetFanSpeedInfo(
    arg0: nvmlUnit_t,
    arg1: *mut nvmlUnitFanSpeeds_t,
) -> nvmlReturn_t {
    let nvmlUnitGetFanSpeedInfo: extern "C" fn(
        arg0: nvmlUnit_t,
        arg1: *mut nvmlUnitFanSpeeds_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetFanSpeedInfo"));
    eprintln!("[CALL] {}", "nvmlUnitGetFanSpeedInfo");
    nvmlUnitGetFanSpeedInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetInforomImageVersion(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetInforomImageVersion: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetInforomImageVersion"));
    eprintln!("[CALL] {}", "nvmlDeviceGetInforomImageVersion");
    nvmlDeviceGetInforomImageVersion(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMaxMigDeviceCount(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMaxMigDeviceCount: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMaxMigDeviceCount"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMaxMigDeviceCount");
    nvmlDeviceGetMaxMigDeviceCount(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstancesPerGpuInstance(
    arg0: *mut nvmlVgpuTypeMaxInstance_t,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetMaxInstancesPerGpuInstance: extern "C" fn(
        arg0: *mut nvmlVgpuTypeMaxInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetMaxInstancesPerGpuInstance"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetMaxInstancesPerGpuInstance");
    nvmlVgpuTypeGetMaxInstancesPerGpuInstance(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCudaComputeCapability(
    arg0: nvmlDevice_t,
    arg1: *mut c_int,
    arg2: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceGetCudaComputeCapability: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_int,
        arg2: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCudaComputeCapability"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCudaComputeCapability");
    nvmlDeviceGetCudaComputeCapability(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetInfo(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlGpuInstanceInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetInfo: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlGpuInstanceInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetInfo"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetInfo");
    nvmlGpuInstanceGetInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfoExt(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPciInfoExt_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPciInfoExt: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPciInfoExt_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPciInfoExt"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPciInfoExt");
    nvmlDeviceGetPciInfoExt(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetDefaultAutoBoostedClocksEnabled(
    arg0: nvmlDevice_t,
    arg1: nvmlEnableState_t,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetDefaultAutoBoostedClocksEnabled: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEnableState_t,
        arg2: c_uint,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceSetDefaultAutoBoostedClocksEnabled"));
    eprintln!("[CALL] {}", "nvmlDeviceSetDefaultAutoBoostedClocksEnabled");
    nvmlDeviceSetDefaultAutoBoostedClocksEnabled(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceClearAccountingPids(
    arg0: nvmlVgpuInstance_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceClearAccountingPids: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceClearAccountingPids"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceClearAccountingPids");
    nvmlVgpuInstanceClearAccountingPids(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPcieReplayCounter(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPcieReplayCounter: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPcieReplayCounter"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPcieReplayCounter");
    nvmlDeviceGetPcieReplayCounter(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetJpgUtilization(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetJpgUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetJpgUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetJpgUtilization");
    nvmlDeviceGetJpgUtilization(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetVgpuCapabilities(
    arg0: nvmlDevice_t,
    arg1: nvmlDeviceVgpuCapability_t,
    arg2: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetVgpuCapabilities: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlDeviceVgpuCapability_t,
        arg2: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetVgpuCapabilities"));
    eprintln!("[CALL] {}", "nvmlDeviceSetVgpuCapabilities");
    nvmlDeviceSetVgpuCapabilities(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMinMaxFanSpeed(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMinMaxFanSpeed: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMinMaxFanSpeed"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMinMaxFanSpeed");
    nvmlDeviceGetMinMaxFanSpeed(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetUUID(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetUUID: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetUUID"));
    eprintln!("[CALL] {}", "nvmlDeviceGetUUID");
    nvmlDeviceGetUUID(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetNvlinkBwMode(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlSystemGetNvlinkBwMode: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetNvlinkBwMode"));
    eprintln!("[CALL] {}", "nvmlSystemGetNvlinkBwMode");
    nvmlSystemGetNvlinkBwMode(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCurrentClocksEventReasons(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetCurrentClocksEventReasons: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCurrentClocksEventReasons"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCurrentClocksEventReasons");
    nvmlDeviceGetCurrentClocksEventReasons(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetUUID(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetUUID: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetUUID"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetUUID");
    nvmlVgpuInstanceGetUUID(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInit_v2() -> nvmlReturn_t {
    let nvmlInit_v2: extern "C" fn() -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlInit_v2"));
    eprintln!("[CALL] {}", "nvmlInit_v2");
    nvmlInit_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAutoBoostedClocksEnabled(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
    arg2: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetAutoBoostedClocksEnabled: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
        arg2: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAutoBoostedClocksEnabled"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAutoBoostedClocksEnabled");
    nvmlDeviceGetAutoBoostedClocksEnabled(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetApplicationsClock(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetApplicationsClock: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetApplicationsClock"));
    eprintln!("[CALL] {}", "nvmlDeviceGetApplicationsClock");
    nvmlDeviceGetApplicationsClock(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPdi(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPdi_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPdi: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut nvmlPdi_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetPdi"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPdi");
    nvmlDeviceGetPdi(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceValidateInforom(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceValidateInforom: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceValidateInforom"));
    eprintln!("[CALL] {}", "nvmlDeviceValidateInforom");
    nvmlDeviceValidateInforom(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMaxClockInfo(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMaxClockInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMaxClockInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMaxClockInfo");
    nvmlDeviceGetMaxClockInfo(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetRuntimeStateSize(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlVgpuRuntimeState_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetRuntimeStateSize: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlVgpuRuntimeState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetRuntimeStateSize"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetRuntimeStateSize");
    nvmlVgpuInstanceGetRuntimeStateSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetUtilizationRates(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlUtilization_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetUtilizationRates: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlUtilization_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetUtilizationRates"));
    eprintln!("[CALL] {}", "nvmlDeviceGetUtilizationRates");
    nvmlDeviceGetUtilizationRates(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerLog(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuSchedulerLog_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuSchedulerLog: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuSchedulerLog_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuSchedulerLog"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuSchedulerLog");
    nvmlDeviceGetVgpuSchedulerLog(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMinorNumber(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMinorNumber: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMinorNumber"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMinorNumber");
    nvmlDeviceGetMinorNumber(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetPowerMizerMode_v1(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDevicePowerMizerModes_v1_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetPowerMizerMode_v1: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDevicePowerMizerModes_v1_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetPowerMizerMode_v1"));
    eprintln!("[CALL] {}", "nvmlDeviceSetPowerMizerMode_v1");
    nvmlDeviceSetPowerMizerMode_v1(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetClockOffsets(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlClockOffset_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetClockOffsets: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlClockOffset_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetClockOffsets"));
    eprintln!("[CALL] {}", "nvmlDeviceGetClockOffsets");
    nvmlDeviceGetClockOffsets(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetExcludedDeviceCount(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlGetExcludedDeviceCount: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGetExcludedDeviceCount"));
    eprintln!("[CALL] {}", "nvmlGetExcludedDeviceCount");
    nvmlGetExcludedDeviceCount(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetViolationStatus(
    arg0: nvmlDevice_t,
    arg1: nvmlPerfPolicyType_t,
    arg2: *mut nvmlViolationTime_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetViolationStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlPerfPolicyType_t,
        arg2: *mut nvmlViolationTime_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetViolationStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetViolationStatus");
    nvmlDeviceGetViolationStatus(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkState(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkState"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkState");
    nvmlDeviceGetNvLinkState(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEncoderStats(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetEncoderStats: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetEncoderStats"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetEncoderStats");
    nvmlVgpuInstanceGetEncoderStats(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSetVgpuVersion(arg0: *mut nvmlVgpuVersion_t) -> nvmlReturn_t {
    let nvmlSetVgpuVersion: extern "C" fn(arg0: *mut nvmlVgpuVersion_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSetVgpuVersion"));
    eprintln!("[CALL] {}", "nvmlSetVgpuVersion");
    nvmlSetVgpuVersion(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRetiredPagesPendingStatus(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetRetiredPagesPendingStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRetiredPagesPendingStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRetiredPagesPendingStatus");
    nvmlDeviceGetRetiredPagesPendingStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetClass(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_char,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetClass: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_char,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetClass"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetClass");
    nvmlVgpuTypeGetClass(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuCapabilities(
    arg0: nvmlDevice_t,
    arg1: nvmlDeviceVgpuCapability_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuCapabilities: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlDeviceVgpuCapability_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuCapabilities"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuCapabilities");
    nvmlDeviceGetVgpuCapabilities(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetMemoryLockedClocks(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetMemoryLockedClocks: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetMemoryLockedClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceSetMemoryLockedClocks");
    nvmlDeviceSetMemoryLockedClocks(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuMetadata(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuPgpuMetadata_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuMetadata: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuPgpuMetadata_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuMetadata"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuMetadata");
    nvmlDeviceGetVgpuMetadata(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleBySerial(
    arg0: *const c_char,
    arg1: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetHandleBySerial: extern "C" fn(
        arg0: *const c_char,
        arg1: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetHandleBySerial"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleBySerial");
    nvmlDeviceGetHandleBySerial(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInitWithFlags(arg0: c_uint) -> nvmlReturn_t {
    let nvmlInitWithFlags: extern "C" fn(arg0: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlInitWithFlags"));
    eprintln!("[CALL] {}", "nvmlInitWithFlags");
    nvmlInitWithFlags(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeState(
    arg0: *mut nvmlConfComputeSystemState_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeState: extern "C" fn(
        arg0: *mut nvmlConfComputeSystemState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetConfComputeState"));
    eprintln!("[CALL] {}", "nvmlSystemGetConfComputeState");
    nvmlSystemGetConfComputeState(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceFreezeNvLinkUtilizationCounter(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
    arg3: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceFreezeNvLinkUtilizationCounter: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
        arg3: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceFreezeNvLinkUtilizationCounter"));
    eprintln!("[CALL] {}", "nvmlDeviceFreezeNvLinkUtilizationCounter");
    nvmlDeviceFreezeNvLinkUtilizationCounter(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFbUsage(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetFbUsage: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetFbUsage"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetFbUsage");
    nvmlVgpuInstanceGetFbUsage(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEccSramUniqueUncorrectedErrorCounts_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEccSramUniqueUncorrectedErrorCounts_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts"));
    eprintln!(
        "[CALL] {}",
        "nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts"
    );
    nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkVersion(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkVersion: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkVersion"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkVersion");
    nvmlDeviceGetNvLinkVersion(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileSetRequestedProfiles(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlWorkloadPowerProfileRequestedProfiles_t,
) -> nvmlReturn_t {
    let nvmlDeviceWorkloadPowerProfileSetRequestedProfiles: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlWorkloadPowerProfileRequestedProfiles_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym(
        "nvmlDeviceWorkloadPowerProfileSetRequestedProfiles",
    ));
    eprintln!(
        "[CALL] {}",
        "nvmlDeviceWorkloadPowerProfileSetRequestedProfiles"
    );
    nvmlDeviceWorkloadPowerProfileSetRequestedProfiles(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeProtectedMemoryUsage(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlMemory_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetConfComputeProtectedMemoryUsage: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlMemory_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetConfComputeProtectedMemoryUsage"));
    eprintln!("[CALL] {}", "nvmlDeviceGetConfComputeProtectedMemoryUsage");
    nvmlDeviceGetConfComputeProtectedMemoryUsage(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedEventTypes(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedEventTypes: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedEventTypes"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedEventTypes");
    nvmlDeviceGetSupportedEventTypes(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCurrentClockFreqs(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDeviceCurrentClockFreqs_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetCurrentClockFreqs: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDeviceCurrentClockFreqs_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCurrentClockFreqs"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCurrentClockFreqs");
    nvmlDeviceGetCurrentClockFreqs(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetModuleId(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetModuleId: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetModuleId"));
    eprintln!("[CALL] {}", "nvmlDeviceGetModuleId");
    nvmlDeviceGetModuleId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetFanControlPolicy(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: nvmlFanControlPolicy_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetFanControlPolicy: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: nvmlFanControlPolicy_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetFanControlPolicy"));
    eprintln!("[CALL] {}", "nvmlDeviceSetFanControlPolicy");
    nvmlDeviceSetFanControlPolicy(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetLicenseStatus(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetLicenseStatus: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetLicenseStatus"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetLicenseStatus");
    nvmlVgpuInstanceGetLicenseStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetVgpuDriverCapabilities(
    arg0: nvmlVgpuDriverCapability_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlGetVgpuDriverCapabilities: extern "C" fn(
        arg0: nvmlVgpuDriverCapability_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGetVgpuDriverCapabilities"));
    eprintln!("[CALL] {}", "nvmlGetVgpuDriverCapabilities");
    nvmlGetVgpuDriverCapabilities(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetCreate(arg0: *mut nvmlEventSet_t) -> nvmlReturn_t {
    let nvmlEventSetCreate: extern "C" fn(arg0: *mut nvmlEventSet_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlEventSetCreate"));
    eprintln!("[CALL] {}", "nvmlEventSetCreate");
    nvmlEventSetCreate(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBoardPartNumber(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetBoardPartNumber: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetBoardPartNumber"));
    eprintln!("[CALL] {}", "nvmlDeviceGetBoardPartNumber");
    nvmlDeviceGetBoardPartNumber(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceCreateGpuInstance(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstance_t,
) -> nvmlReturn_t {
    let nvmlDeviceCreateGpuInstance: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceCreateGpuInstance"));
    eprintln!("[CALL] {}", "nvmlDeviceCreateGpuInstance");
    nvmlDeviceCreateGpuInstance(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceSetEncoderCapacity(
    arg0: nvmlVgpuInstance_t,
    arg1: c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceSetEncoderCapacity: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceSetEncoderCapacity"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceSetEncoderCapacity");
    nvmlVgpuInstanceSetEncoderCapacity(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByIndex_v2(
    arg0: c_uint,
    arg1: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetHandleByIndex_v2: extern "C" fn(
        arg0: c_uint,
        arg1: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetHandleByIndex_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleByIndex_v2");
    nvmlDeviceGetHandleByIndex_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeCapabilities(
    arg0: *mut nvmlConfComputeSystemCaps_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeCapabilities: extern "C" fn(
        arg0: *mut nvmlConfComputeSystemCaps_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetConfComputeCapabilities"));
    eprintln!("[CALL] {}", "nvmlSystemGetConfComputeCapabilities");
    nvmlSystemGetConfComputeCapabilities(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvlinkBwMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlNvlinkGetBwMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvlinkBwMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlNvlinkGetBwMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvlinkBwMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvlinkBwMode");
    nvmlDeviceGetNvlinkBwMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeSettings(
    arg0: *mut nvmlSystemConfComputeSettings_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeSettings: extern "C" fn(
        arg0: *mut nvmlSystemConfComputeSettings_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetConfComputeSettings"));
    eprintln!("[CALL] {}", "nvmlSystemGetConfComputeSettings");
    nvmlSystemGetConfComputeSettings(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetFbReservation(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetFbReservation: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetFbReservation"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetFbReservation");
    nvmlVgpuTypeGetFbReservation(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceById(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: *mut nvmlComputeInstance_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetComputeInstanceById: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: *mut nvmlComputeInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetComputeInstanceById"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetComputeInstanceById");
    nvmlGpuInstanceGetComputeInstanceById(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemoryInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlMemory_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemoryInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlMemory_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemoryInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemoryInfo");
    nvmlDeviceGetMemoryInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRetiredPages_v2(
    arg0: nvmlDevice_t,
    arg1: nvmlPageRetirementCause_t,
    arg2: *mut c_uint,
    arg3: *mut c_ulonglong,
    arg4: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetRetiredPages_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlPageRetirementCause_t,
        arg2: *mut c_uint,
        arg3: *mut c_ulonglong,
        arg4: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRetiredPages_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRetiredPages_v2");
    nvmlDeviceGetRetiredPages_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetNVMLVersion(arg0: *mut c_char, arg1: c_uint) -> nvmlReturn_t {
    let nvmlSystemGetNVMLVersion: extern "C" fn(arg0: *mut c_char, arg1: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetNVMLVersion"));
    eprintln!("[CALL] {}", "nvmlSystemGetNVMLVersion");
    nvmlSystemGetNVMLVersion(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedMemoryClocks(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedMemoryClocks: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedMemoryClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedMemoryClocks");
    nvmlDeviceGetSupportedMemoryClocks(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceDiscoverGpus(arg0: *mut nvmlPciInfo_t) -> nvmlReturn_t {
    let nvmlDeviceDiscoverGpus: extern "C" fn(arg0: *mut nvmlPciInfo_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceDiscoverGpus"));
    eprintln!("[CALL] {}", "nvmlDeviceDiscoverGpus");
    nvmlDeviceDiscoverGpus(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFBCStats(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlFBCStats_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetFBCStats: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlFBCStats_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetFBCStats"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetFBCStats");
    nvmlVgpuInstanceGetFBCStats(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSerial(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetSerial: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSerial"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSerial");
    nvmlDeviceGetSerial(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetUnitInfo(
    arg0: nvmlUnit_t,
    arg1: *mut nvmlUnitInfo_t,
) -> nvmlReturn_t {
    let nvmlUnitGetUnitInfo: extern "C" fn(
        arg0: nvmlUnit_t,
        arg1: *mut nvmlUnitInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetUnitInfo"));
    eprintln!("[CALL] {}", "nvmlUnitGetUnitInfo");
    nvmlUnitGetUnitInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDevicePowerSmoothingActivatePresetProfile(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPowerSmoothingProfile_t,
) -> nvmlReturn_t {
    let nvmlDevicePowerSmoothingActivatePresetProfile: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPowerSmoothingProfile_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDevicePowerSmoothingActivatePresetProfile"));
    eprintln!("[CALL] {}", "nvmlDevicePowerSmoothingActivatePresetProfile");
    nvmlDevicePowerSmoothingActivatePresetProfile(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAccountingMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetAccountingMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAccountingMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAccountingMode");
    nvmlDeviceGetAccountingMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCapabilities(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDeviceCapabilities_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetCapabilities: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDeviceCapabilities_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCapabilities"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCapabilities");
    nvmlDeviceGetCapabilities(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMultiGpuBoard(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMultiGpuBoard: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMultiGpuBoard"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMultiGpuBoard");
    nvmlDeviceGetMultiGpuBoard(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuProcessUtilization(
    arg0: nvmlDevice_t,
    arg1: c_ulonglong,
    arg2: *mut c_uint,
    arg3: *mut nvmlVgpuProcessUtilizationSample_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuProcessUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_ulonglong,
        arg2: *mut c_uint,
        arg3: *mut nvmlVgpuProcessUtilizationSample_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuProcessUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuProcessUtilization");
    nvmlDeviceGetVgpuProcessUtilization(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceModifyDrainState(
    arg0: *mut nvmlPciInfo_t,
    arg1: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceModifyDrainState: extern "C" fn(
        arg0: *mut nvmlPciInfo_t,
        arg1: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceModifyDrainState"));
    eprintln!("[CALL] {}", "nvmlDeviceModifyDrainState");
    nvmlDeviceModifyDrainState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeGpuCertificate(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlConfComputeGpuCertificate_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetConfComputeGpuCertificate: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlConfComputeGpuCertificate_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetConfComputeGpuCertificate"));
    eprintln!("[CALL] {}", "nvmlDeviceGetConfComputeGpuCertificate");
    nvmlDeviceGetConfComputeGpuCertificate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceSetVgpuSchedulerState(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlVgpuSchedulerState_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceSetVgpuSchedulerState: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlVgpuSchedulerState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceSetVgpuSchedulerState"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceSetVgpuSchedulerState");
    nvmlGpuInstanceSetVgpuSchedulerState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFieldValues(
    arg0: nvmlDevice_t,
    arg1: c_int,
    arg2: *mut nvmlFieldValue_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetFieldValues: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_int,
        arg2: *mut nvmlFieldValue_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFieldValues"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFieldValues");
    nvmlDeviceGetFieldValues(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetPersistenceMode(
    arg0: nvmlDevice_t,
    arg1: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetPersistenceMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetPersistenceMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetPersistenceMode");
    nvmlDeviceSetPersistenceMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceQueryDrainState(
    arg0: *mut nvmlPciInfo_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceQueryDrainState: extern "C" fn(
        arg0: *mut nvmlPciInfo_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceQueryDrainState"));
    eprintln!("[CALL] {}", "nvmlDeviceQueryDrainState");
    nvmlDeviceQueryDrainState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetAccountingStats(
    arg0: nvmlVgpuInstance_t,
    arg1: c_uint,
    arg2: *mut nvmlAccountingStats_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetAccountingStats: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: c_uint,
        arg2: *mut nvmlAccountingStats_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetAccountingStats"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetAccountingStats");
    nvmlVgpuInstanceGetAccountingStats(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkErrorCounter(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: nvmlNvLinkErrorCounter_t,
    arg3: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkErrorCounter: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: nvmlNvLinkErrorCounter_t,
        arg3: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkErrorCounter"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkErrorCounter");
    nvmlDeviceGetNvLinkErrorCounter(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFBCStats(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlFBCStats_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetFBCStats: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlFBCStats_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFBCStats"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFBCStats");
    nvmlDeviceGetFBCStats(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCurrPcieLinkGeneration(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetCurrPcieLinkGeneration: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCurrPcieLinkGeneration"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCurrPcieLinkGeneration");
    nvmlDeviceGetCurrPcieLinkGeneration(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerState(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuSchedulerGetState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuSchedulerState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuSchedulerGetState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuSchedulerState"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuSchedulerState");
    nvmlDeviceGetVgpuSchedulerState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetThermalSettings(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuThermalSettings_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetThermalSettings: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuThermalSettings_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetThermalSettings"));
    eprintln!("[CALL] {}", "nvmlDeviceGetThermalSettings");
    nvmlDeviceGetThermalSettings(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlComputeInstanceDestroy(arg0: nvmlComputeInstance_t) -> nvmlReturn_t {
    let nvmlComputeInstanceDestroy: extern "C" fn(arg0: nvmlComputeInstance_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlComputeInstanceDestroy"));
    eprintln!("[CALL] {}", "nvmlComputeInstanceDestroy");
    nvmlComputeInstanceDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceClearCpuAffinity(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceClearCpuAffinity: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceClearCpuAffinity"));
    eprintln!("[CALL] {}", "nvmlDeviceClearCpuAffinity");
    nvmlDeviceClearCpuAffinity(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetProcessesUtilizationInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlProcessesUtilizationInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetProcessesUtilizationInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlProcessesUtilizationInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetProcessesUtilizationInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetProcessesUtilizationInfo");
    nvmlDeviceGetProcessesUtilizationInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetDeviceID(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_ulonglong,
    arg2: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetDeviceID: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_ulonglong,
        arg2: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetDeviceID"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetDeviceID");
    nvmlVgpuTypeGetDeviceID(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCoolerInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlCoolerInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetCoolerInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlCoolerInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCoolerInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCoolerInfo");
    nvmlDeviceGetCoolerInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceRemoveGpu_v2(
    arg0: *mut nvmlPciInfo_t,
    arg1: nvmlDetachGpuState_t,
    arg2: nvmlPcieLinkState_t,
) -> nvmlReturn_t {
    let nvmlDeviceRemoveGpu_v2: extern "C" fn(
        arg0: *mut nvmlPciInfo_t,
        arg1: nvmlDetachGpuState_t,
        arg2: nvmlPcieLinkState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceRemoveGpu_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceRemoveGpu_v2");
    nvmlDeviceRemoveGpu_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetInforomVersion(
    arg0: nvmlDevice_t,
    arg1: nvmlInforomObject_t,
    arg2: *mut c_char,
    arg3: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetInforomVersion: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlInforomObject_t,
        arg2: *mut c_char,
        arg3: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetInforomVersion"));
    eprintln!("[CALL] {}", "nvmlDeviceGetInforomVersion");
    nvmlDeviceGetInforomVersion(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceId(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstanceId: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstanceId"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstanceId");
    nvmlDeviceGetGpuInstanceId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByUUIDV(
    arg0: *const nvmlUUID_t,
    arg1: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetHandleByUUIDV: extern "C" fn(
        arg0: *const nvmlUUID_t,
        arg1: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetHandleByUUIDV"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleByUUIDV");
    nvmlDeviceGetHandleByUUIDV(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedVgpus(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlVgpuTypeId_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedVgpus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlVgpuTypeId_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedVgpus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedVgpus");
    nvmlDeviceGetSupportedVgpus(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetNvLinkUtilizationControl(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
    arg3: *mut nvmlNvLinkUtilizationControl_t,
    arg4: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetNvLinkUtilizationControl: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
        arg3: *mut nvmlNvLinkUtilizationControl_t,
        arg4: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetNvLinkUtilizationControl"));
    eprintln!("[CALL] {}", "nvmlDeviceSetNvLinkUtilizationControl");
    nvmlDeviceSetNvLinkUtilizationControl(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMPSComputeRunningProcesses_v3(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlProcessInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetMPSComputeRunningProcesses_v3: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlProcessInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMPSComputeRunningProcesses_v3"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMPSComputeRunningProcesses_v3");
    nvmlDeviceGetMPSComputeRunningProcesses_v3(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDynamicPstatesInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGpuDynamicPstatesInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDynamicPstatesInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGpuDynamicPstatesInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDynamicPstatesInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDynamicPstatesInfo");
    nvmlDeviceGetDynamicPstatesInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerCapabilities(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuSchedulerCapabilities_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuSchedulerCapabilities: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuSchedulerCapabilities_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuSchedulerCapabilities"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuSchedulerCapabilities");
    nvmlDeviceGetVgpuSchedulerCapabilities(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceOnSameBoard(
    arg0: nvmlDevice_t,
    arg1: nvmlDevice_t,
    arg2: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceOnSameBoard: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlDevice_t,
        arg2: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceOnSameBoard"));
    eprintln!("[CALL] {}", "nvmlDeviceOnSameBoard");
    nvmlDeviceOnSameBoard(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceById(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstance_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstanceById: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstanceById"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstanceById");
    nvmlDeviceGetGpuInstanceById(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTemperatureV(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlTemperature_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetTemperatureV: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlTemperature_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTemperatureV"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTemperatureV");
    nvmlDeviceGetTemperatureV(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPcieSpeed(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPcieSpeed: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPcieSpeed"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPcieSpeed");
    nvmlDeviceGetPcieSpeed(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetMdevUUID(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetMdevUUID: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetMdevUUID"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetMdevUUID");
    nvmlVgpuInstanceGetMdevUUID(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTopologyNearestGpus(
    arg0: nvmlDevice_t,
    arg1: nvmlGpuTopologyLevel_t,
    arg2: *mut c_uint,
    arg3: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetTopologyNearestGpus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlGpuTopologyLevel_t,
        arg2: *mut c_uint,
        arg3: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTopologyNearestGpus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTopologyNearestGpus");
    nvmlDeviceGetTopologyNearestGpus(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFanSpeedRPM(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlFanSpeedInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetFanSpeedRPM: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlFanSpeedInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFanSpeedRPM"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFanSpeedRPM");
    nvmlDeviceGetFanSpeedRPM(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAddressingMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDeviceAddressingMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetAddressingMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDeviceAddressingMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAddressingMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAddressingMode");
    nvmlDeviceGetAddressingMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstancePossiblePlacements_v2(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstancePlacement_t,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstancePossiblePlacements_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstancePlacement_t,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetGpuInstancePossiblePlacements_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstancePossiblePlacements_v2");
    nvmlDeviceGetGpuInstancePossiblePlacements_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpcClkVfOffset(
    arg0: nvmlDevice_t,
    arg1: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpcClkVfOffset: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpcClkVfOffset"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpcClkVfOffset");
    nvmlDeviceGetGpcClkVfOffset(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemClkVfOffset(
    arg0: nvmlDevice_t,
    arg1: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemClkVfOffset: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemClkVfOffset"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemClkVfOffset");
    nvmlDeviceGetMemClkVfOffset(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBridgeChipInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlBridgeChipHierarchy_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetBridgeChipInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlBridgeChipHierarchy_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetBridgeChipInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetBridgeChipInfo");
    nvmlDeviceGetBridgeChipInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetExcludedDeviceInfoByIndex(
    arg0: c_uint,
    arg1: *mut nvmlExcludedDeviceInfo_t,
) -> nvmlReturn_t {
    let nvmlGetExcludedDeviceInfoByIndex: extern "C" fn(
        arg0: c_uint,
        arg1: *mut nvmlExcludedDeviceInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGetExcludedDeviceInfoByIndex"));
    eprintln!("[CALL] {}", "nvmlGetExcludedDeviceInfoByIndex");
    nvmlGetExcludedDeviceInfoByIndex(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetLastBBXFlushTime(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
    arg2: *mut c_ulong,
) -> nvmlReturn_t {
    let nvmlDeviceGetLastBBXFlushTime: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
        arg2: *mut c_ulong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetLastBBXFlushTime"));
    eprintln!("[CALL] {}", "nvmlDeviceGetLastBBXFlushTime");
    nvmlDeviceGetLastBBXFlushTime(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetDriverBranch(
    arg0: *mut nvmlSystemDriverBranchInfo_t,
    arg1: c_uint,
) -> nvmlReturn_t {
    let nvmlSystemGetDriverBranch: extern "C" fn(
        arg0: *mut nvmlSystemDriverBranchInfo_t,
        arg1: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetDriverBranch"));
    eprintln!("[CALL] {}", "nvmlSystemGetDriverBranch");
    nvmlSystemGetDriverBranch(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetEccMode(
    arg0: nvmlDevice_t,
    arg1: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetEccMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetEccMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetEccMode");
    nvmlDeviceSetEccMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstancePossiblePlacements(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: *mut nvmlComputeInstancePlacement_t,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetComputeInstancePossiblePlacements: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: *mut nvmlComputeInstancePlacement_t,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym(
        "nvmlGpuInstanceGetComputeInstancePossiblePlacements",
    ));
    eprintln!(
        "[CALL] {}",
        "nvmlGpuInstanceGetComputeInstancePossiblePlacements"
    );
    nvmlGpuInstanceGetComputeInstancePossiblePlacements(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeInstanceId(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetComputeInstanceId: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetComputeInstanceId"));
    eprintln!("[CALL] {}", "nvmlDeviceGetComputeInstanceId");
    nvmlDeviceGetComputeInstanceId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAttributes_v2(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDeviceAttributes_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetAttributes_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDeviceAttributes_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAttributes_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAttributes_v2");
    nvmlDeviceGetAttributes_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetAccountingPids(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetAccountingPids: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetAccountingPids"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetAccountingPids");
    nvmlVgpuInstanceGetAccountingPids(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlComputeMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetComputeMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlComputeMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetComputeMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetComputeMode");
    nvmlDeviceGetComputeMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstancesPerVm(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetMaxInstancesPerVm: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetMaxInstancesPerVm"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetMaxInstancesPerVm");
    nvmlVgpuTypeGetMaxInstancesPerVm(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerMizerMode_v1(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDevicePowerMizerModes_v1_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerMizerMode_v1: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDevicePowerMizerModes_v1_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerMizerMode_v1"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerMizerMode_v1");
    nvmlDeviceGetPowerMizerMode_v1(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkRemotePciInfo_v2(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlPciInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkRemotePciInfo_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlPciInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkRemotePciInfo_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkRemotePciInfo_v2");
    nvmlDeviceGetNvLinkRemotePciInfo_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuMaxPcieLinkGeneration(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuMaxPcieLinkGeneration: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuMaxPcieLinkGeneration"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuMaxPcieLinkGeneration");
    nvmlDeviceGetGpuMaxPcieLinkGeneration(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRetiredPages(
    arg0: nvmlDevice_t,
    arg1: nvmlPageRetirementCause_t,
    arg2: *mut c_uint,
    arg3: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetRetiredPages: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlPageRetirementCause_t,
        arg2: *mut c_uint,
        arg3: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRetiredPages"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRetiredPages");
    nvmlDeviceGetRetiredPages(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetEnforcedPowerLimit(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetEnforcedPowerLimit: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetEnforcedPowerLimit"));
    eprintln!("[CALL] {}", "nvmlDeviceGetEnforcedPowerLimit");
    nvmlDeviceGetEnforcedPowerLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo_v3(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPciInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPciInfo_v3: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPciInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPciInfo_v3"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPciInfo_v3");
    nvmlDeviceGetPciInfo_v3(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetOfaUtilization(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetOfaUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetOfaUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetOfaUtilization");
    nvmlDeviceGetOfaUtilization(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceRemainingCapacity(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetComputeInstanceRemainingCapacity: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym(
        "nvmlGpuInstanceGetComputeInstanceRemainingCapacity",
    ));
    eprintln!(
        "[CALL] {}",
        "nvmlGpuInstanceGetComputeInstanceRemainingCapacity"
    );
    nvmlGpuInstanceGetComputeInstanceRemainingCapacity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuFabricInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGpuFabricInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuFabricInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGpuFabricInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuFabricInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuFabricInfo");
    nvmlDeviceGetGpuFabricInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBrand(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlBrandType_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetBrand: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlBrandType_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetBrand"));
    eprintln!("[CALL] {}", "nvmlDeviceGetBrand");
    nvmlDeviceGetBrand(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPlatformInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPlatformInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPlatformInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPlatformInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPlatformInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPlatformInfo");
    nvmlDeviceGetPlatformInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCreatableVgpus(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlVgpuTypeId_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetCreatableVgpus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlVgpuTypeId_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCreatableVgpus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCreatableVgpus");
    nvmlDeviceGetCreatableVgpus(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetProcessName(
    arg0: c_uint,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlSystemGetProcessName: extern "C" fn(
        arg0: c_uint,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetProcessName"));
    eprintln!("[CALL] {}", "nvmlSystemGetProcessName");
    nvmlSystemGetProcessName(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPersistenceMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPersistenceMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPersistenceMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPersistenceMode");
    nvmlDeviceGetPersistenceMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetArchitecture(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDeviceArchitecture_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetArchitecture: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDeviceArchitecture_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetArchitecture"));
    eprintln!("[CALL] {}", "nvmlDeviceGetArchitecture");
    nvmlDeviceGetArchitecture(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCpuAffinityWithinScope(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_ulong,
    arg3: nvmlAffinityScope_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetCpuAffinityWithinScope: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_ulong,
        arg3: nvmlAffinityScope_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCpuAffinityWithinScope"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCpuAffinityWithinScope");
    nvmlDeviceGetCpuAffinityWithinScope(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMarginTemperature(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlMarginTemperature_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetMarginTemperature: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlMarginTemperature_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMarginTemperature"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMarginTemperature");
    nvmlDeviceGetMarginTemperature(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceCreateGpuInstanceWithPlacement(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *const nvmlGpuInstancePlacement_t,
    arg3: *mut nvmlGpuInstance_t,
) -> nvmlReturn_t {
    let nvmlDeviceCreateGpuInstanceWithPlacement: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *const nvmlGpuInstancePlacement_t,
        arg3: *mut nvmlGpuInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceCreateGpuInstanceWithPlacement"));
    eprintln!("[CALL] {}", "nvmlDeviceCreateGpuInstanceWithPlacement");
    nvmlDeviceCreateGpuInstanceWithPlacement(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvlinkSupportedBwModes(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlNvlinkSupportedBwModes_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvlinkSupportedBwModes: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlNvlinkSupportedBwModes_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvlinkSupportedBwModes"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvlinkSupportedBwModes");
    nvmlDeviceGetNvlinkSupportedBwModes(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetDefaultFanSpeed_v2(
    arg0: nvmlDevice_t,
    arg1: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetDefaultFanSpeed_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetDefaultFanSpeed_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceSetDefaultFanSpeed_v2");
    nvmlDeviceSetDefaultFanSpeed_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceClearFieldValues(
    arg0: nvmlDevice_t,
    arg1: c_int,
    arg2: *mut nvmlFieldValue_t,
) -> nvmlReturn_t {
    let nvmlDeviceClearFieldValues: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_int,
        arg2: *mut nvmlFieldValue_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceClearFieldValues"));
    eprintln!("[CALL] {}", "nvmlDeviceClearFieldValues");
    nvmlDeviceClearFieldValues(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRemappedRows(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
    arg3: *mut c_uint,
    arg4: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetRemappedRows: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
        arg3: *mut c_uint,
        arg4: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRemappedRows"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRemappedRows");
    nvmlDeviceGetRemappedRows(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTotalEnergyConsumption(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetTotalEnergyConsumption: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTotalEnergyConsumption"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTotalEnergyConsumption");
    nvmlDeviceGetTotalEnergyConsumption(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetClockInfo(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetClockInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetClockInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetClockInfo");
    nvmlDeviceGetClockInfo(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuInstancesUtilizationInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuInstancesUtilizationInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuInstancesUtilizationInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuInstancesUtilizationInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuInstancesUtilizationInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuInstancesUtilizationInfo");
    nvmlDeviceGetVgpuInstancesUtilizationInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTopologyCommonAncestor(
    arg0: nvmlDevice_t,
    arg1: nvmlDevice_t,
    arg2: *mut nvmlGpuTopologyLevel_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetTopologyCommonAncestor: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlDevice_t,
        arg2: *mut nvmlGpuTopologyLevel_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTopologyCommonAncestor"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTopologyCommonAncestor");
    nvmlDeviceGetTopologyCommonAncestor(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetFanSpeed_v2(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetFanSpeed_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetFanSpeed_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceSetFanSpeed_v2");
    nvmlDeviceSetFanSpeed_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstances(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: *mut nvmlComputeInstance_t,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetComputeInstances: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: *mut nvmlComputeInstance_t,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetComputeInstances"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetComputeInstances");
    nvmlGpuInstanceGetComputeInstances(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBusType(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlBusType_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetBusType: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlBusType_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetBusType"));
    eprintln!("[CALL] {}", "nvmlDeviceGetBusType");
    nvmlDeviceGetBusType(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuProcessesUtilizationInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuProcessesUtilizationInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuProcessesUtilizationInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuProcessesUtilizationInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuProcessesUtilizationInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuProcessesUtilizationInfo");
    nvmlDeviceGetVgpuProcessesUtilizationInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMaxPcieLinkWidth(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMaxPcieLinkWidth: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMaxPcieLinkWidth"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMaxPcieLinkWidth");
    nvmlDeviceGetMaxPcieLinkWidth(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetPsuInfo(
    arg0: nvmlUnit_t,
    arg1: *mut nvmlPSUInfo_t,
) -> nvmlReturn_t {
    let nvmlUnitGetPsuInfo: extern "C" fn(
        arg0: nvmlUnit_t,
        arg1: *mut nvmlPSUInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetPsuInfo"));
    eprintln!("[CALL] {}", "nvmlUnitGetPsuInfo");
    nvmlUnitGetPsuInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeGpuAttestationReport(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlConfComputeGpuAttestationReport_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetConfComputeGpuAttestationReport: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlConfComputeGpuAttestationReport_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetConfComputeGpuAttestationReport"));
    eprintln!("[CALL] {}", "nvmlDeviceGetConfComputeGpuAttestationReport");
    nvmlDeviceGetConfComputeGpuAttestationReport(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetActiveVgpus(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlActiveVgpuInstanceInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetActiveVgpus: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlActiveVgpuInstanceInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetActiveVgpus"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetActiveVgpus");
    nvmlGpuInstanceGetActiveVgpus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGraphicsRunningProcesses_v3(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlProcessInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGraphicsRunningProcesses_v3: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlProcessInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGraphicsRunningProcesses_v3"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGraphicsRunningProcesses_v3");
    nvmlDeviceGetGraphicsRunningProcesses_v3(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSetStreamingEnabled(
    arg0: nvmlDevice_t,
    arg1: c_uint,
) -> nvmlReturn_t {
    let nvmlGpmSetStreamingEnabled: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpmSetStreamingEnabled"));
    eprintln!("[CALL] {}", "nvmlGpmSetStreamingEnabled");
    nvmlGpmSetStreamingEnabled(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVirtualizationMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGpuVirtualizationMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVirtualizationMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGpuVirtualizationMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVirtualizationMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVirtualizationMode");
    nvmlDeviceGetVirtualizationMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerSource(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPowerSource_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerSource: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPowerSource_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerSource"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerSource");
    nvmlDeviceGetPowerSource(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetComputeMode(
    arg0: nvmlDevice_t,
    arg1: nvmlComputeMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetComputeMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlComputeMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetComputeMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetComputeMode");
    nvmlDeviceSetComputeMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAccountingBufferSize(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetAccountingBufferSize: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAccountingBufferSize"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAccountingBufferSize");
    nvmlDeviceGetAccountingBufferSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlComputeInstanceGetInfo_v2(
    arg0: nvmlComputeInstance_t,
    arg1: *mut nvmlComputeInstanceInfo_t,
) -> nvmlReturn_t {
    let nvmlComputeInstanceGetInfo_v2: extern "C" fn(
        arg0: nvmlComputeInstance_t,
        arg1: *mut nvmlComputeInstanceInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlComputeInstanceGetInfo_v2"));
    eprintln!("[CALL] {}", "nvmlComputeInstanceGetInfo_v2");
    nvmlComputeInstanceGetInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCount_v2(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlDeviceGetCount_v2: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetCount_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCount_v2");
    nvmlDeviceGetCount_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceRegisterEvents(
    arg0: nvmlDevice_t,
    arg1: c_ulonglong,
    arg2: nvmlEventSet_t,
) -> nvmlReturn_t {
    let nvmlDeviceRegisterEvents: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_ulonglong,
        arg2: nvmlEventSet_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceRegisterEvents"));
    eprintln!("[CALL] {}", "nvmlDeviceRegisterEvents");
    nvmlDeviceRegisterEvents(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetTemperature(
    arg0: nvmlUnit_t,
    arg1: c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlUnitGetTemperature: extern "C" fn(
        arg0: nvmlUnit_t,
        arg1: c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetTemperature"));
    eprintln!("[CALL] {}", "nvmlUnitGetTemperature");
    nvmlUnitGetTemperature(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmQueryDeviceSupport(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGpmSupport_t,
) -> nvmlReturn_t {
    let nvmlGpmQueryDeviceSupport: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGpmSupport_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpmQueryDeviceSupport"));
    eprintln!("[CALL] {}", "nvmlGpmQueryDeviceSupport");
    nvmlGpmQueryDeviceSupport(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemoryBusWidth(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemoryBusWidth: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemoryBusWidth"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemoryBusWidth");
    nvmlDeviceGetMemoryBusWidth(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkRemoteDeviceType(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlIntNvLinkDeviceType_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkRemoteDeviceType: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlIntNvLinkDeviceType_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkRemoteDeviceType"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkRemoteDeviceType");
    nvmlDeviceGetNvLinkRemoteDeviceType(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuTypeCreatablePlacements(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlVgpuCreatablePlacementInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetVgpuTypeCreatablePlacements: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlVgpuCreatablePlacementInfo_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpuInstanceGetVgpuTypeCreatablePlacements"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetVgpuTypeCreatablePlacements");
    nvmlGpuInstanceGetVgpuTypeCreatablePlacements(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerUsage(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerUsage: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerUsage"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerUsage");
    nvmlDeviceGetPowerUsage(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedPerformanceStates(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPstates_t,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedPerformanceStates: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPstates_t,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedPerformanceStates"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedPerformanceStates");
    nvmlDeviceGetSupportedPerformanceStates(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkUtilizationCounter(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
    arg3: *mut c_ulonglong,
    arg4: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkUtilizationCounter: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
        arg3: *mut c_ulonglong,
        arg4: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkUtilizationCounter"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkUtilizationCounter");
    nvmlDeviceGetNvLinkUtilizationCounter(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFBCSessions(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlFBCSessionInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetFBCSessions: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlFBCSessionInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFBCSessions"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFBCSessions");
    nvmlDeviceGetFBCSessions(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetVgpuSchedulerState(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuSchedulerSetState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetVgpuSchedulerState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuSchedulerSetState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetVgpuSchedulerState"));
    eprintln!("[CALL] {}", "nvmlDeviceSetVgpuSchedulerState");
    nvmlDeviceSetVgpuSchedulerState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNumaNodeId(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetNumaNodeId: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNumaNodeId"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNumaNodeId");
    nvmlDeviceGetNumaNodeId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDisplayActive(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDisplayActive: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDisplayActive"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDisplayActive");
    nvmlDeviceGetDisplayActive(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByUUID(
    arg0: *const c_char,
    arg1: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetHandleByUUID: extern "C" fn(
        arg0: *const c_char,
        arg1: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetHandleByUUID"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleByUUID");
    nvmlDeviceGetHandleByUUID(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDetailedEccErrors(
    arg0: nvmlDevice_t,
    arg1: nvmlMemoryErrorType_t,
    arg2: nvmlEccCounterType_t,
    arg3: *mut nvmlEccErrorCounts_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDetailedEccErrors: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlMemoryErrorType_t,
        arg2: nvmlEccCounterType_t,
        arg3: *mut nvmlEccErrorCounts_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDetailedEccErrors"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDetailedEccErrors");
    nvmlDeviceGetDetailedEccErrors(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeMemSizeInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlConfComputeMemSizeInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetConfComputeMemSizeInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlConfComputeMemSizeInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetConfComputeMemSizeInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetConfComputeMemSizeInfo");
    nvmlDeviceGetConfComputeMemSizeInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedClocksThrottleReasons(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedClocksThrottleReasons: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedClocksThrottleReasons"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedClocksThrottleReasons");
    nvmlDeviceGetSupportedClocksThrottleReasons(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuTypeSupportedPlacements(
    arg0: nvmlDevice_t,
    arg1: nvmlVgpuTypeId_t,
    arg2: *mut nvmlVgpuPlacementList_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuTypeSupportedPlacements: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlVgpuTypeId_t,
        arg2: *mut nvmlVgpuPlacementList_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuTypeSupportedPlacements"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuTypeSupportedPlacements");
    nvmlDeviceGetVgpuTypeSupportedPlacements(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetClkMonStatus(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlClkMonStatus_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetClkMonStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlClkMonStatus_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetClkMonStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetClkMonStatus");
    nvmlDeviceGetClkMonStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetEncoderUtilization(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetEncoderUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetEncoderUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetEncoderUtilization");
    nvmlDeviceGetEncoderUtilization(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemEventSetFree(
    arg0: *mut nvmlSystemEventSetFreeRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemEventSetFree: extern "C" fn(
        arg0: *mut nvmlSystemEventSetFreeRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemEventSetFree"));
    eprintln!("[CALL] {}", "nvmlSystemEventSetFree");
    nvmlSystemEventSetFree(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetWait_v2(
    arg0: nvmlEventSet_t,
    arg1: *mut nvmlEventData_t,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlEventSetWait_v2: extern "C" fn(
        arg0: nvmlEventSet_t,
        arg1: *mut nvmlEventData_t,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlEventSetWait_v2"));
    eprintln!("[CALL] {}", "nvmlEventSetWait_v2");
    nvmlEventSetWait_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVbiosVersion(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetVbiosVersion: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVbiosVersion"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVbiosVersion");
    nvmlDeviceGetVbiosVersion(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTemperature(
    arg0: nvmlDevice_t,
    arg1: nvmlTemperatureSensors_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetTemperature: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlTemperatureSensors_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTemperature"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTemperature");
    nvmlDeviceGetTemperature(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuFabricInfoV(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGpuFabricInfoV_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuFabricInfoV: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGpuFabricInfoV_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuFabricInfoV"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuFabricInfoV");
    nvmlDeviceGetGpuFabricInfoV(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetC2cModeInfoV(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlC2cModeInfo_v1_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetC2cModeInfoV: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlC2cModeInfo_v1_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetC2cModeInfoV"));
    eprintln!("[CALL] {}", "nvmlDeviceGetC2cModeInfoV");
    nvmlDeviceGetC2cModeInfoV(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemSetNvlinkBwMode(arg0: c_uint) -> nvmlReturn_t {
    let nvmlSystemSetNvlinkBwMode: extern "C" fn(arg0: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemSetNvlinkBwMode"));
    eprintln!("[CALL] {}", "nvmlSystemSetNvlinkBwMode");
    nvmlSystemSetNvlinkBwMode(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceProfileInfo(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: c_uint,
    arg3: *mut nvmlComputeInstanceProfileInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetComputeInstanceProfileInfo: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: c_uint,
        arg3: *mut nvmlComputeInstanceProfileInfo_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpuInstanceGetComputeInstanceProfileInfo"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetComputeInstanceProfileInfo");
    nvmlGpuInstanceGetComputeInstanceProfileInfo(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmMetricsGet(arg0: *mut nvmlGpmMetricsGet_t) -> nvmlReturn_t {
    let nvmlGpmMetricsGet: extern "C" fn(arg0: *mut nvmlGpmMetricsGet_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmMetricsGet"));
    eprintln!("[CALL] {}", "nvmlGpmMetricsGet");
    nvmlGpmMetricsGet(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetMemClkVfOffset(
    arg0: nvmlDevice_t,
    arg1: c_int,
) -> nvmlReturn_t {
    let nvmlDeviceSetMemClkVfOffset: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetMemClkVfOffset"));
    eprintln!("[CALL] {}", "nvmlDeviceSetMemClkVfOffset");
    nvmlDeviceSetMemClkVfOffset(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMigMode(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMigMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMigMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMigMode");
    nvmlDeviceGetMigMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetNvLinkErrorCounters(
    arg0: nvmlDevice_t,
    arg1: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceResetNvLinkErrorCounters: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceResetNvLinkErrorCounters"));
    eprintln!("[CALL] {}", "nvmlDeviceResetNvLinkErrorCounters");
    nvmlDeviceResetNvLinkErrorCounters(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetTemperatureThreshold(
    arg0: nvmlDevice_t,
    arg1: nvmlTemperatureThresholds_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetTemperatureThreshold: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlTemperatureThresholds_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetTemperatureThreshold"));
    eprintln!("[CALL] {}", "nvmlDeviceGetTemperatureThreshold");
    nvmlDeviceGetTemperatureThreshold(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEncoderCapacity(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetEncoderCapacity: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetEncoderCapacity"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetEncoderCapacity");
    nvmlVgpuInstanceGetEncoderCapacity(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemSetConfComputeGpusReadyState(arg0: c_uint) -> nvmlReturn_t {
    let nvmlSystemSetConfComputeGpusReadyState: extern "C" fn(arg0: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemSetConfComputeGpusReadyState"));
    eprintln!("[CALL] {}", "nvmlSystemSetConfComputeGpusReadyState");
    nvmlSystemSetConfComputeGpusReadyState(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceCreateComputeInstance(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: *mut nvmlComputeInstance_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceCreateComputeInstance: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: *mut nvmlComputeInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceCreateComputeInstance"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceCreateComputeInstance");
    nvmlGpuInstanceCreateComputeInstance(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFanControlPolicy_v2(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlFanControlPolicy_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetFanControlPolicy_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlFanControlPolicy_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFanControlPolicy_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFanControlPolicy_v2");
    nvmlDeviceGetFanControlPolicy_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuSchedulerState(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlVgpuSchedulerStateInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetVgpuSchedulerState: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlVgpuSchedulerStateInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetVgpuSchedulerState"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetVgpuSchedulerState");
    nvmlGpuInstanceGetVgpuSchedulerState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDefaultEccMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDefaultEccMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDefaultEccMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDefaultEccMode");
    nvmlDeviceGetDefaultEccMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerManagementMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerManagementMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerManagementMode");
    nvmlDeviceGetPowerManagementMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetFree(arg0: nvmlEventSet_t) -> nvmlReturn_t {
    let nvmlEventSetFree: extern "C" fn(arg0: nvmlEventSet_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlEventSetFree"));
    eprintln!("[CALL] {}", "nvmlEventSetFree");
    nvmlEventSetFree(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetBAR1Info(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut nvmlVgpuTypeBar1Info_t,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetBAR1Info: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut nvmlVgpuTypeBar1Info_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetBAR1Info"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetBAR1Info");
    nvmlVgpuTypeGetBAR1Info(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetClock(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: nvmlClockId_t,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetClock: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: nvmlClockId_t,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetClock"));
    eprintln!("[CALL] {}", "nvmlDeviceGetClock");
    nvmlDeviceGetClock(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFanSpeed(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetFanSpeed: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFanSpeed"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFanSpeed");
    nvmlDeviceGetFanSpeed(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDeviceHandleFromMigDeviceHandle(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDeviceHandleFromMigDeviceHandle: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDevice_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetDeviceHandleFromMigDeviceHandle"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDeviceHandleFromMigDeviceHandle");
    nvmlDeviceGetDeviceHandleFromMigDeviceHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetCpuAffinity(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceSetCpuAffinity: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceSetCpuAffinity"));
    eprintln!("[CALL] {}", "nvmlDeviceSetCpuAffinity");
    nvmlDeviceSetCpuAffinity(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetAccountingMode(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetAccountingMode: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetAccountingMode"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetAccountingMode");
    nvmlVgpuInstanceGetAccountingMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileGetCurrentProfiles(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlWorkloadPowerProfileCurrentProfiles_t,
) -> nvmlReturn_t {
    let nvmlDeviceWorkloadPowerProfileGetCurrentProfiles: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlWorkloadPowerProfileCurrentProfiles_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceWorkloadPowerProfileGetCurrentProfiles"));
    eprintln!(
        "[CALL] {}",
        "nvmlDeviceWorkloadPowerProfileGetCurrentProfiles"
    );
    nvmlDeviceWorkloadPowerProfileGetCurrentProfiles(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetClockOffsets(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlClockOffset_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetClockOffsets: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlClockOffset_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetClockOffsets"));
    eprintln!("[CALL] {}", "nvmlDeviceSetClockOffsets");
    nvmlDeviceSetClockOffsets(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDisplayMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDisplayMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDisplayMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDisplayMode");
    nvmlDeviceGetDisplayMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemRegisterEvents(
    arg0: *mut nvmlSystemRegisterEventRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemRegisterEvents: extern "C" fn(
        arg0: *mut nvmlSystemRegisterEventRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemRegisterEvents"));
    eprintln!("[CALL] {}", "nvmlSystemRegisterEvents");
    nvmlSystemRegisterEvents(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetGspHeapSize(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetGspHeapSize: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetGspHeapSize"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetGspHeapSize");
    nvmlVgpuTypeGetGspHeapSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetAutoBoostedClocksEnabled(
    arg0: nvmlDevice_t,
    arg1: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetAutoBoostedClocksEnabled: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetAutoBoostedClocksEnabled"));
    eprintln!("[CALL] {}", "nvmlDeviceSetAutoBoostedClocksEnabled");
    nvmlDeviceSetAutoBoostedClocksEnabled(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerState(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPstates_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPstates_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerState"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerState");
    nvmlDeviceGetPowerState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBAR1MemoryInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlBAR1Memory_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetBAR1MemoryInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlBAR1Memory_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetBAR1MemoryInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetBAR1MemoryInfo");
    nvmlDeviceGetBAR1MemoryInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPerformanceModes(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDevicePerfModes_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPerformanceModes: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDevicePerfModes_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPerformanceModes"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPerformanceModes");
    nvmlDeviceGetPerformanceModes(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileGetProfilesInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlWorkloadPowerProfileProfilesInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceWorkloadPowerProfileGetProfilesInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlWorkloadPowerProfileProfilesInfo_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceWorkloadPowerProfileGetProfilesInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceWorkloadPowerProfileGetProfilesInfo");
    nvmlDeviceWorkloadPowerProfileGetProfilesInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlShutdown() -> nvmlReturn_t {
    let nvmlShutdown: extern "C" fn() -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlShutdown"));
    eprintln!("[CALL] {}", "nvmlShutdown");
    nvmlShutdown()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetType(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlVgpuTypeId_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetType: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlVgpuTypeId_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetType"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetType");
    nvmlVgpuInstanceGetType(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetGpuInstanceProfileId(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetGpuInstanceProfileId: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetGpuInstanceProfileId"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetGpuInstanceProfileId");
    nvmlVgpuTypeGetGpuInstanceProfileId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCurrentClocksThrottleReasons(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetCurrentClocksThrottleReasons: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCurrentClocksThrottleReasons"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCurrentClocksThrottleReasons");
    nvmlDeviceGetCurrentClocksThrottleReasons(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceProfileInfoByIdV(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstanceProfileInfo_v2_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstanceProfileInfoByIdV: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstanceProfileInfo_v2_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstanceProfileInfoByIdV"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstanceProfileInfoByIdV");
    nvmlDeviceGetGpuInstanceProfileInfoByIdV(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetP2PStatus(
    arg0: nvmlDevice_t,
    arg1: nvmlDevice_t,
    arg2: nvmlGpuP2PCapsIndex_t,
    arg3: *mut nvmlGpuP2PStatus_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetP2PStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlDevice_t,
        arg2: nvmlGpuP2PCapsIndex_t,
        arg3: *mut nvmlGpuP2PStatus_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetP2PStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetP2PStatus");
    nvmlDeviceGetP2PStatus(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemClkMinMaxVfOffset(
    arg0: nvmlDevice_t,
    arg1: *mut c_int,
    arg2: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemClkMinMaxVfOffset: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_int,
        arg2: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemClkMinMaxVfOffset"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemClkMinMaxVfOffset");
    nvmlDeviceGetMemClkMinMaxVfOffset(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetAccountingMode(
    arg0: nvmlDevice_t,
    arg1: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetAccountingMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetAccountingMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetAccountingMode");
    nvmlDeviceSetAccountingMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeRunningProcesses_v3(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlProcessInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetComputeRunningProcesses_v3: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlProcessInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetComputeRunningProcesses_v3"));
    eprintln!("[CALL] {}", "nvmlDeviceGetComputeRunningProcesses_v3");
    nvmlDeviceGetComputeRunningProcesses_v3(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCpuAffinity(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_ulong,
) -> nvmlReturn_t {
    let nvmlDeviceGetCpuAffinity: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_ulong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetCpuAffinity"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCpuAffinity");
    nvmlDeviceGetCpuAffinity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlNvLinkInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlNvLinkInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkInfo");
    nvmlDeviceGetNvLinkInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHostVgpuMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlHostVgpuMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetHostVgpuMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlHostVgpuMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetHostVgpuMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHostVgpuMode");
    nvmlDeviceGetHostVgpuMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceClearEccErrorCounts(
    arg0: nvmlDevice_t,
    arg1: nvmlEccCounterType_t,
) -> nvmlReturn_t {
    let nvmlDeviceClearEccErrorCounts: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlEccCounterType_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceClearEccErrorCounts"));
    eprintln!("[CALL] {}", "nvmlDeviceClearEccErrorCounts");
    nvmlDeviceClearEccErrorCounts(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAPIRestriction(
    arg0: nvmlDevice_t,
    arg1: nvmlRestrictedAPI_t,
    arg2: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetAPIRestriction: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlRestrictedAPI_t,
        arg2: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAPIRestriction"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAPIRestriction");
    nvmlDeviceGetAPIRestriction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlErrorString(arg0: nvmlReturn_t) -> *const c_char {
    let nvmlErrorString: extern "C" fn(arg0: nvmlReturn_t) -> *const c_char =
        std::mem::transmute(get_sym("nvmlErrorString"));
    eprintln!("[CALL] {}", "nvmlErrorString");
    nvmlErrorString(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuHeterogeneousMode(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetVgpuHeterogeneousMode: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlVgpuHeterogeneousMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetVgpuHeterogeneousMode"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetVgpuHeterogeneousMode");
    nvmlGpuInstanceGetVgpuHeterogeneousMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetProcessUtilization(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlProcessUtilizationSample_t,
    arg2: *mut c_uint,
    arg3: c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetProcessUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlProcessUtilizationSample_t,
        arg2: *mut c_uint,
        arg3: c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetProcessUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetProcessUtilization");
    nvmlDeviceGetProcessUtilization(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetNvLinkDeviceLowPowerThreshold(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlNvLinkPowerThres_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetNvLinkDeviceLowPowerThreshold: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlNvLinkPowerThres_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetNvLinkDeviceLowPowerThreshold"));
    eprintln!("[CALL] {}", "nvmlDeviceSetNvLinkDeviceLowPowerThreshold");
    nvmlDeviceSetNvLinkDeviceLowPowerThreshold(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNumFans(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetNumFans: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetNumFans"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNumFans");
    nvmlDeviceGetNumFans(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSampleAlloc(arg0: *mut nvmlGpmSample_t) -> nvmlReturn_t {
    let nvmlGpmSampleAlloc: extern "C" fn(arg0: *mut nvmlGpmSample_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmSampleAlloc"));
    eprintln!("[CALL] {}", "nvmlGpmSampleAlloc");
    nvmlGpmSampleAlloc(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuTypeCreatablePlacements(
    arg0: nvmlDevice_t,
    arg1: nvmlVgpuTypeId_t,
    arg2: *mut nvmlVgpuPlacementList_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuTypeCreatablePlacements: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlVgpuTypeId_t,
        arg2: *mut nvmlVgpuPlacementList_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuTypeCreatablePlacements"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuTypeCreatablePlacements");
    nvmlDeviceGetVgpuTypeCreatablePlacements(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMinMaxClockOfPState(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: nvmlPstates_t,
    arg3: *mut c_uint,
    arg4: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMinMaxClockOfPState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: nvmlPstates_t,
        arg3: *mut c_uint,
        arg4: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMinMaxClockOfPState"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMinMaxClockOfPState");
    nvmlDeviceGetMinMaxClockOfPState(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetNumDisplayHeads(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetNumDisplayHeads: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetNumDisplayHeads"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetNumDisplayHeads");
    nvmlVgpuTypeGetNumDisplayHeads(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFBCSessions(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlFBCSessionInfo_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetFBCSessions: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlFBCSessionInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetFBCSessions"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetFBCSessions");
    nvmlVgpuInstanceGetFBCSessions(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEccMode(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetEccMode: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetEccMode"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetEccMode");
    nvmlVgpuInstanceGetEccMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDevicePowerSmoothingUpdatePresetProfileParam(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPowerSmoothingProfile_t,
) -> nvmlReturn_t {
    let nvmlDevicePowerSmoothingUpdatePresetProfileParam: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPowerSmoothingProfile_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDevicePowerSmoothingUpdatePresetProfileParam"));
    eprintln!(
        "[CALL] {}",
        "nvmlDevicePowerSmoothingUpdatePresetProfileParam"
    );
    nvmlDevicePowerSmoothingUpdatePresetProfileParam(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetDriverModel(
    arg0: nvmlDevice_t,
    arg1: nvmlDriverModel_t,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetDriverModel: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlDriverModel_t,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetDriverModel"));
    eprintln!("[CALL] {}", "nvmlDeviceSetDriverModel");
    nvmlDeviceSetDriverModel(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetLicenseInfo_v2(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlVgpuLicenseInfo_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetLicenseInfo_v2: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlVgpuLicenseInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetLicenseInfo_v2"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetLicenseInfo_v2");
    nvmlVgpuInstanceGetLicenseInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitSetLedState(
    arg0: nvmlUnit_t,
    arg1: nvmlLedColor_t,
) -> nvmlReturn_t {
    let nvmlUnitSetLedState: extern "C" fn(arg0: nvmlUnit_t, arg1: nvmlLedColor_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlUnitSetLedState"));
    eprintln!("[CALL] {}", "nvmlUnitSetLedState");
    nvmlUnitSetLedState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetCreatableVgpus(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlVgpuTypeIdInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetCreatableVgpus: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlVgpuTypeIdInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetCreatableVgpus"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetCreatableVgpus");
    nvmlGpuInstanceGetCreatableVgpus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDecoderUtilization(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetDecoderUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDecoderUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDecoderUtilization");
    nvmlDeviceGetDecoderUtilization(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkUtilizationControl(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
    arg3: *mut nvmlNvLinkUtilizationControl_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetNvLinkUtilizationControl: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
        arg3: *mut nvmlNvLinkUtilizationControl_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNvLinkUtilizationControl"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkUtilizationControl");
    nvmlDeviceGetNvLinkUtilizationControl(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetApplicationsClocks(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetApplicationsClocks: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetApplicationsClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceSetApplicationsClocks");
    nvmlDeviceSetApplicationsClocks(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetMetadata(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut nvmlVgpuMetadata_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetMetadata: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut nvmlVgpuMetadata_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetMetadata"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetMetadata");
    nvmlVgpuInstanceGetMetadata(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetIndex(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t {
    let nvmlDeviceGetIndex: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetIndex"));
    eprintln!("[CALL] {}", "nvmlDeviceGetIndex");
    nvmlDeviceGetIndex(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpcClkMinMaxVfOffset(
    arg0: nvmlDevice_t,
    arg1: *mut c_int,
    arg2: *mut c_int,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpcClkMinMaxVfOffset: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_int,
        arg2: *mut c_int,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpcClkMinMaxVfOffset"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpcClkMinMaxVfOffset");
    nvmlDeviceGetGpcClkMinMaxVfOffset(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetFanSpeed_v2(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetFanSpeed_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetFanSpeed_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetFanSpeed_v2");
    nvmlDeviceGetFanSpeed_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetRepairStatus(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlRepairStatus_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetRepairStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlRepairStatus_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetRepairStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetRepairStatus");
    nvmlDeviceGetRepairStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSampleGet(
    arg0: nvmlDevice_t,
    arg1: nvmlGpmSample_t,
) -> nvmlReturn_t {
    let nvmlGpmSampleGet: extern "C" fn(arg0: nvmlDevice_t, arg1: nvmlGpmSample_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmSampleGet"));
    eprintln!("[CALL] {}", "nvmlGpmSampleGet");
    nvmlGpmSampleGet(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementDefaultLimit(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPowerManagementDefaultLimit: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPowerManagementDefaultLimit"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPowerManagementDefaultLimit");
    nvmlDeviceGetPowerManagementDefaultLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGspFirmwareVersion(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
) -> nvmlReturn_t {
    let nvmlDeviceGetGspFirmwareVersion: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGspFirmwareVersion"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGspFirmwareVersion");
    nvmlDeviceGetGspFirmwareVersion(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemoryInfo_v2(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlMemory_v2_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemoryInfo_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlMemory_v2_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemoryInfo_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemoryInfo_v2");
    nvmlDeviceGetMemoryInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetGpuPciId(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_char,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetGpuPciId: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_char,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetGpuPciId"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetGpuPciId");
    nvmlVgpuInstanceGetGpuPciId(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceDestroy(arg0: nvmlGpuInstance_t) -> nvmlReturn_t {
    let nvmlGpuInstanceDestroy: extern "C" fn(arg0: nvmlGpuInstance_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpuInstanceDestroy"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceDestroy");
    nvmlGpuInstanceDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetGpuInstanceId(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetGpuInstanceId: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetGpuInstanceId"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetGpuInstanceId");
    nvmlVgpuInstanceGetGpuInstanceId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetName(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_char,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetName: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_char,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetName"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetName");
    nvmlVgpuTypeGetName(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetVirtualizationMode(
    arg0: nvmlDevice_t,
    arg1: nvmlGpuVirtualizationMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetVirtualizationMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlGpuVirtualizationMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetVirtualizationMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetVirtualizationMode");
    nvmlDeviceSetVirtualizationMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetCount(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlUnitGetCount: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlUnitGetCount"));
    eprintln!("[CALL] {}", "nvmlUnitGetCount");
    nvmlUnitGetCount(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBoardId(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetBoardId: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetBoardId"));
    eprintln!("[CALL] {}", "nvmlDeviceGetBoardId");
    nvmlDeviceGetBoardId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetMemoryLockedClocks(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceResetMemoryLockedClocks: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceResetMemoryLockedClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceResetMemoryLockedClocks");
    nvmlDeviceResetMemoryLockedClocks(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetPowerManagementLimit_v2(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPowerValue_v2_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetPowerManagementLimit_v2: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPowerValue_v2_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetPowerManagementLimit_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceSetPowerManagementLimit_v2");
    nvmlDeviceSetPowerManagementLimit_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSramEccErrorStatus(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlEccSramErrorStatus_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetSramEccErrorStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlEccSramErrorStatus_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSramEccErrorStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSramEccErrorStatus");
    nvmlDeviceGetSramEccErrorStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuOperationMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlGpuOperationMode_t,
    arg2: *mut nvmlGpuOperationMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuOperationMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlGpuOperationMode_t,
        arg2: *mut nvmlGpuOperationMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuOperationMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuOperationMode");
    nvmlDeviceGetGpuOperationMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDramEncryptionMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlDramEncryptionInfo_t,
    arg2: *mut nvmlDramEncryptionInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetDramEncryptionMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlDramEncryptionInfo_t,
        arg2: *mut nvmlDramEncryptionInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetDramEncryptionMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDramEncryptionMode");
    nvmlDeviceGetDramEncryptionMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuHeterogeneousMode(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuHeterogeneousMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlVgpuHeterogeneousMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuHeterogeneousMode"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuHeterogeneousMode");
    nvmlDeviceGetVgpuHeterogeneousMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetGpuLockedClocks(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceSetGpuLockedClocks: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetGpuLockedClocks"));
    eprintln!("[CALL] {}", "nvmlDeviceSetGpuLockedClocks");
    nvmlDeviceSetGpuLockedClocks(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDevicePowerSmoothingSetState(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPowerSmoothingState_t,
) -> nvmlReturn_t {
    let nvmlDevicePowerSmoothingSetState: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPowerSmoothingState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDevicePowerSmoothingSetState"));
    eprintln!("[CALL] {}", "nvmlDevicePowerSmoothingSetState");
    nvmlDevicePowerSmoothingSetState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileClearRequestedProfiles(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlWorkloadPowerProfileRequestedProfiles_t,
) -> nvmlReturn_t {
    let nvmlDeviceWorkloadPowerProfileClearRequestedProfiles: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlWorkloadPowerProfileRequestedProfiles_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym(
        "nvmlDeviceWorkloadPowerProfileClearRequestedProfiles",
    ));
    eprintln!(
        "[CALL] {}",
        "nvmlDeviceWorkloadPowerProfileClearRequestedProfiles"
    );
    nvmlDeviceWorkloadPowerProfileClearRequestedProfiles(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPgpuMetadataString(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPgpuMetadataString: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPgpuMetadataString"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPgpuMetadataString");
    nvmlDeviceGetPgpuMetadataString(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSupportedClocksEventReasons(
    arg0: nvmlDevice_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlDeviceGetSupportedClocksEventReasons: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSupportedClocksEventReasons"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSupportedClocksEventReasons");
    nvmlDeviceGetSupportedClocksEventReasons(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetFramebufferSize(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_ulonglong,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetFramebufferSize: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_ulonglong,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetFramebufferSize"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetFramebufferSize");
    nvmlVgpuTypeGetFramebufferSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNumGpuCores(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetNumGpuCores: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetNumGpuCores"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNumGpuCores");
    nvmlDeviceGetNumGpuCores(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetHandleByIndex(
    arg0: c_uint,
    arg1: *mut nvmlUnit_t,
) -> nvmlReturn_t {
    let nvmlUnitGetHandleByIndex: extern "C" fn(
        arg0: c_uint,
        arg1: *mut nvmlUnit_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlUnitGetHandleByIndex"));
    eprintln!("[CALL] {}", "nvmlUnitGetHandleByIndex");
    nvmlUnitGetHandleByIndex(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMaxPcieLinkGeneration(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMaxPcieLinkGeneration: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMaxPcieLinkGeneration"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMaxPcieLinkGeneration");
    nvmlDeviceGetMaxPcieLinkGeneration(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetEncoderSessions(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlEncoderSessionInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetEncoderSessions: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlEncoderSessionInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetEncoderSessions"));
    eprintln!("[CALL] {}", "nvmlDeviceGetEncoderSessions");
    nvmlDeviceGetEncoderSessions(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetSamples(
    arg0: nvmlDevice_t,
    arg1: nvmlSamplingType_t,
    arg2: c_ulonglong,
    arg3: *mut nvmlValueType_t,
    arg4: *mut c_uint,
    arg5: *mut nvmlSample_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetSamples: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlSamplingType_t,
        arg2: c_ulonglong,
        arg3: *mut nvmlValueType_t,
        arg4: *mut c_uint,
        arg5: *mut nvmlSample_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetSamples"));
    eprintln!("[CALL] {}", "nvmlDeviceGetSamples");
    nvmlDeviceGetSamples(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMaxCustomerBoostClock(
    arg0: nvmlDevice_t,
    arg1: nvmlClockType_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetMaxCustomerBoostClock: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlClockType_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMaxCustomerBoostClock"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMaxCustomerBoostClock");
    nvmlDeviceGetMaxCustomerBoostClock(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPcieThroughput(
    arg0: nvmlDevice_t,
    arg1: nvmlPcieUtilCounter_t,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPcieThroughput: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlPcieUtilCounter_t,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPcieThroughput"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPcieThroughput");
    nvmlDeviceGetPcieThroughput(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPcieLinkMaxSpeed(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetPcieLinkMaxSpeed: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetPcieLinkMaxSpeed"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPcieLinkMaxSpeed");
    nvmlDeviceGetPcieLinkMaxSpeed(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceProfileInfo(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstanceProfileInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstanceProfileInfo: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstanceProfileInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstanceProfileInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstanceProfileInfo");
    nvmlDeviceGetGpuInstanceProfileInfo(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMigDeviceHandleByIndex(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetMigDeviceHandleByIndex: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMigDeviceHandleByIndex"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMigDeviceHandleByIndex");
    nvmlDeviceGetMigDeviceHandleByIndex(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetActiveVgpus(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut nvmlVgpuInstance_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetActiveVgpus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut nvmlVgpuInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetActiveVgpus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetActiveVgpus");
    nvmlDeviceGetActiveVgpus(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetNvLinkUtilizationCounter(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceResetNvLinkUtilizationCounter: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceResetNvLinkUtilizationCounter"));
    eprintln!("[CALL] {}", "nvmlDeviceResetNvLinkUtilizationCounter");
    nvmlDeviceResetNvLinkUtilizationCounter(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetHicVersion(
    arg0: *mut c_uint,
    arg1: *mut nvmlHwbcEntry_t,
) -> nvmlReturn_t {
    let nvmlSystemGetHicVersion: extern "C" fn(
        arg0: *mut c_uint,
        arg1: *mut nvmlHwbcEntry_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetHicVersion"));
    eprintln!("[CALL] {}", "nvmlSystemGetHicVersion");
    nvmlSystemGetHicVersion(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstances(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlGpuInstance_t,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstances: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlGpuInstance_t,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstances"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstances");
    nvmlDeviceGetGpuInstances(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetVgpuHeterogeneousMode(
    arg0: nvmlDevice_t,
    arg1: *const nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetVgpuHeterogeneousMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *const nvmlVgpuHeterogeneousMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetVgpuHeterogeneousMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetVgpuHeterogeneousMode");
    nvmlDeviceSetVgpuHeterogeneousMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetAPIRestriction(
    arg0: nvmlDevice_t,
    arg1: nvmlRestrictedAPI_t,
    arg2: nvmlEnableState_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetAPIRestriction: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: nvmlRestrictedAPI_t,
        arg2: nvmlEnableState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetAPIRestriction"));
    eprintln!("[CALL] {}", "nvmlDeviceSetAPIRestriction");
    nvmlDeviceSetAPIRestriction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetName(
    arg0: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetName: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetName"));
    eprintln!("[CALL] {}", "nvmlDeviceGetName");
    nvmlDeviceGetName(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAccountingStats(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut nvmlAccountingStats_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetAccountingStats: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut nvmlAccountingStats_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAccountingStats"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAccountingStats");
    nvmlDeviceGetAccountingStats(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetResolution(
    arg0: nvmlVgpuTypeId_t,
    arg1: c_uint,
    arg2: *mut c_uint,
    arg3: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetResolution: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: c_uint,
        arg2: *mut c_uint,
        arg3: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetResolution"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetResolution");
    nvmlVgpuTypeGetResolution(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceSetVgpuHeterogeneousMode(
    arg0: nvmlGpuInstance_t,
    arg1: *const nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceSetVgpuHeterogeneousMode: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *const nvmlVgpuHeterogeneousMode_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceSetVgpuHeterogeneousMode"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceSetVgpuHeterogeneousMode");
    nvmlGpuInstanceSetVgpuHeterogeneousMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuSchedulerLog(
    arg0: nvmlGpuInstance_t,
    arg1: *mut nvmlVgpuSchedulerLogInfo_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceGetVgpuSchedulerLog: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: *mut nvmlVgpuSchedulerLogInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGpuInstanceGetVgpuSchedulerLog"));
    eprintln!("[CALL] {}", "nvmlGpuInstanceGetVgpuSchedulerLog");
    nvmlGpuInstanceGetVgpuSchedulerLog(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetLicense(
    arg0: nvmlVgpuTypeId_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetLicense: extern "C" fn(
        arg0: nvmlVgpuTypeId_t,
        arg1: *mut c_char,
        arg2: c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetLicense"));
    eprintln!("[CALL] {}", "nvmlVgpuTypeGetLicense");
    nvmlVgpuTypeGetLicense(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetTopologyGpuSet(
    arg0: c_uint,
    arg1: *mut c_uint,
    arg2: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    let nvmlSystemGetTopologyGpuSet: extern "C" fn(
        arg0: c_uint,
        arg1: *mut c_uint,
        arg2: *mut nvmlDevice_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetTopologyGpuSet"));
    eprintln!("[CALL] {}", "nvmlSystemGetTopologyGpuSet");
    nvmlSystemGetTopologyGpuSet(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetDramEncryptionMode(
    arg0: nvmlDevice_t,
    arg1: *const nvmlDramEncryptionInfo_t,
) -> nvmlReturn_t {
    let nvmlDeviceSetDramEncryptionMode: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *const nvmlDramEncryptionInfo_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceSetDramEncryptionMode"));
    eprintln!("[CALL] {}", "nvmlDeviceSetDramEncryptionMode");
    nvmlDeviceSetDramEncryptionMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAdaptiveClockInfoStatus(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetAdaptiveClockInfoStatus: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAdaptiveClockInfoStatus"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAdaptiveClockInfoStatus");
    nvmlDeviceGetAdaptiveClockInfoStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetVmID(
    arg0: nvmlVgpuInstance_t,
    arg1: *mut c_char,
    arg2: c_uint,
    arg3: *mut nvmlVgpuVmIdType_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceGetVmID: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
        arg1: *mut c_char,
        arg2: c_uint,
        arg3: *mut nvmlVgpuVmIdType_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceGetVmID"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetVmID");
    nvmlVgpuInstanceGetVmID(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAccountingPids(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetAccountingPids: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetAccountingPids"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAccountingPids");
    nvmlDeviceGetAccountingPids(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemSetConfComputeKeyRotationThresholdInfo(
    arg0: *mut nvmlConfComputeSetKeyRotationThresholdInfo_t,
) -> nvmlReturn_t {
    let nvmlSystemSetConfComputeKeyRotationThresholdInfo: extern "C" fn(
        arg0: *mut nvmlConfComputeSetKeyRotationThresholdInfo_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemSetConfComputeKeyRotationThresholdInfo"));
    eprintln!(
        "[CALL] {}",
        "nvmlSystemSetConfComputeKeyRotationThresholdInfo"
    );
    nvmlSystemSetConfComputeKeyRotationThresholdInfo(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceReadWritePRM_v1(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPRMTLV_v1_t,
) -> nvmlReturn_t {
    let nvmlDeviceReadWritePRM_v1: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: *mut nvmlPRMTLV_v1_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceReadWritePRM_v1"));
    eprintln!("[CALL] {}", "nvmlDeviceReadWritePRM_v1");
    nvmlDeviceReadWritePRM_v1(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceCreateComputeInstanceWithPlacement(
    arg0: nvmlGpuInstance_t,
    arg1: c_uint,
    arg2: *const nvmlComputeInstancePlacement_t,
    arg3: *mut nvmlComputeInstance_t,
) -> nvmlReturn_t {
    let nvmlGpuInstanceCreateComputeInstanceWithPlacement: extern "C" fn(
        arg0: nvmlGpuInstance_t,
        arg1: c_uint,
        arg2: *const nvmlComputeInstancePlacement_t,
        arg3: *mut nvmlComputeInstance_t,
    ) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpuInstanceCreateComputeInstanceWithPlacement"));
    eprintln!(
        "[CALL] {}",
        "nvmlGpuInstanceCreateComputeInstanceWithPlacement"
    );
    nvmlGpuInstanceCreateComputeInstanceWithPlacement(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetVgpuVersion(
    arg0: *mut nvmlVgpuVersion_t,
    arg1: *mut nvmlVgpuVersion_t,
) -> nvmlReturn_t {
    let nvmlGetVgpuVersion: extern "C" fn(
        arg0: *mut nvmlVgpuVersion_t,
        arg1: *mut nvmlVgpuVersion_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlGetVgpuVersion"));
    eprintln!("[CALL] {}", "nvmlGetVgpuVersion");
    nvmlGetVgpuVersion(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceRemainingCapacity(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetGpuInstanceRemainingCapacity: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_uint,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetGpuInstanceRemainingCapacity"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstanceRemainingCapacity");
    nvmlDeviceGetGpuInstanceRemainingCapacity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemoryAffinity(
    arg0: nvmlDevice_t,
    arg1: c_uint,
    arg2: *mut c_ulong,
    arg3: nvmlAffinityScope_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetMemoryAffinity: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_uint,
        arg2: *mut c_ulong,
        arg3: nvmlAffinityScope_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetMemoryAffinity"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMemoryAffinity");
    nvmlDeviceGetMemoryAffinity(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVgpuUtilization(
    arg0: nvmlDevice_t,
    arg1: c_ulonglong,
    arg2: *mut nvmlValueType_t,
    arg3: *mut c_uint,
    arg4: *mut nvmlVgpuInstanceUtilizationSample_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetVgpuUtilization: extern "C" fn(
        arg0: nvmlDevice_t,
        arg1: c_ulonglong,
        arg2: *mut nvmlValueType_t,
        arg3: *mut c_uint,
        arg4: *mut nvmlVgpuInstanceUtilizationSample_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlDeviceGetVgpuUtilization"));
    eprintln!("[CALL] {}", "nvmlDeviceGetVgpuUtilization");
    nvmlDeviceGetVgpuUtilization(arg0, arg1, arg2, arg3, arg4)
}

// Unknown function stubs (functions not found in header)
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGraphicsRunningProcesses() {
    let nvmlDeviceGetGraphicsRunningProcesses: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGraphicsRunningProcesses"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGraphicsRunningProcesses");
    nvmlDeviceGetGraphicsRunningProcesses()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures_v2() {
    let nvmlDeviceGetGridLicensableFeatures_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures_v2");
    nvmlDeviceGetGridLicensableFeatures_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetWait() {
    let nvmlEventSetWait: extern "C" fn() = std::mem::transmute(get_sym("nvmlEventSetWait"));
    eprintln!("[CALL] {}", "nvmlEventSetWait");
    nvmlEventSetWait()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo_v2() {
    let nvmlDeviceGetPciInfo_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetPciInfo_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPciInfo_v2");
    nvmlDeviceGetPciInfo_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDriverModel() {
    let nvmlDeviceGetDriverModel: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetDriverModel"));
    eprintln!("[CALL] {}", "nvmlDeviceGetDriverModel");
    nvmlDeviceGetDriverModel()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHostname_v1() {
    let nvmlDeviceGetHostname_v1: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetHostname_v1"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHostname_v1");
    nvmlDeviceGetHostname_v1()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeGetProcessInfo() {
    let nvmlEscapeGetProcessInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlEscapeGetProcessInfo"));
    eprintln!("[CALL] {}", "nvmlEscapeGetProcessInfo");
    nvmlEscapeGetProcessInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGraphicsRunningProcesses_v2() {
    let nvmlDeviceGetGraphicsRunningProcesses_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGraphicsRunningProcesses_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGraphicsRunningProcesses_v2");
    nvmlDeviceGetGraphicsRunningProcesses_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetBlacklistDeviceInfoByIndex() {
    let nvmlGetBlacklistDeviceInfoByIndex: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlGetBlacklistDeviceInfoByIndex"));
    eprintln!("[CALL] {}", "nvmlGetBlacklistDeviceInfoByIndex");
    nvmlGetBlacklistDeviceInfoByIndex()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMPSComputeRunningProcesses() {
    let nvmlDeviceGetMPSComputeRunningProcesses: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetMPSComputeRunningProcesses"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMPSComputeRunningProcesses");
    nvmlDeviceGetMPSComputeRunningProcesses()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInternalGetExportTable() -> c_int {
    eprintln!("[CALL] {}", "nvmlInternalGetExportTable");
    0
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCount() {
    let nvmlDeviceGetCount: extern "C" fn() = std::mem::transmute(get_sym("nvmlDeviceGetCount"));
    eprintln!("[CALL] {}", "nvmlDeviceGetCount");
    nvmlDeviceGetCount()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetLicenseInfo() {
    let nvmlVgpuInstanceGetLicenseInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlVgpuInstanceGetLicenseInfo"));
    eprintln!("[CALL] {}", "nvmlVgpuInstanceGetLicenseInfo");
    nvmlVgpuInstanceGetLicenseInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetHostname_v1() {
    let nvmlDeviceSetHostname_v1: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceSetHostname_v1"));
    eprintln!("[CALL] {}", "nvmlDeviceSetHostname_v1");
    nvmlDeviceSetHostname_v1()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInit() {
    let nvmlInit: extern "C" fn() = std::mem::transmute(get_sym("nvmlInit"));
    eprintln!("[CALL] {}", "nvmlInit");
    nvmlInit()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeRunningProcesses_v2() {
    let nvmlDeviceGetComputeRunningProcesses_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetComputeRunningProcesses_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetComputeRunningProcesses_v2");
    nvmlDeviceGetComputeRunningProcesses_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlThunkInit() {
    let nvmlThunkInit: extern "C" fn() = std::mem::transmute(get_sym("nvmlThunkInit"));
    eprintln!("[CALL] {}", "nvmlThunkInit");
    nvmlThunkInit()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstancePossiblePlacements() {
    let nvmlDeviceGetGpuInstancePossiblePlacements: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGpuInstancePossiblePlacements"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGpuInstancePossiblePlacements");
    nvmlDeviceGetGpuInstancePossiblePlacements()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmFree() {
    let nvmlEscapeRmFree: extern "C" fn() = std::mem::transmute(get_sym("nvmlEscapeRmFree"));
    eprintln!("[CALL] {}", "nvmlEscapeRmFree");
    nvmlEscapeRmFree()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlThunkEscape() {
    let nvmlThunkEscape: extern "C" fn() = std::mem::transmute(get_sym("nvmlThunkEscape"));
    eprintln!("[CALL] {}", "nvmlThunkEscape");
    nvmlThunkEscape()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlRetry_NvRmControl() {
    let nvmlRetry_NvRmControl: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlRetry_NvRmControl"));
    eprintln!("[CALL] {}", "nvmlRetry_NvRmControl");
    nvmlRetry_NvRmControl()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures() {
    let nvmlDeviceGetGridLicensableFeatures: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures");
    nvmlDeviceGetGridLicensableFeatures()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetBlacklistDeviceCount() {
    let nvmlGetBlacklistDeviceCount: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlGetBlacklistDeviceCount"));
    eprintln!("[CALL] {}", "nvmlGetBlacklistDeviceCount");
    nvmlGetBlacklistDeviceCount()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures_v3() {
    let nvmlDeviceGetGridLicensableFeatures_v3: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures_v3"));
    eprintln!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures_v3");
    nvmlDeviceGetGridLicensableFeatures_v3()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmAllocRoot() {
    let nvmlEscapeRmAllocRoot: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlEscapeRmAllocRoot"));
    eprintln!("[CALL] {}", "nvmlEscapeRmAllocRoot");
    nvmlEscapeRmAllocRoot()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeGetMemoryInfo() {
    let nvmlEscapeGetMemoryInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlEscapeGetMemoryInfo"));
    eprintln!("[CALL] {}", "nvmlEscapeGetMemoryInfo");
    nvmlEscapeGetMemoryInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceRemoveGpu() {
    let nvmlDeviceRemoveGpu: extern "C" fn() = std::mem::transmute(get_sym("nvmlDeviceRemoveGpu"));
    eprintln!("[CALL] {}", "nvmlDeviceRemoveGpu");
    nvmlDeviceRemoveGpu()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeRunningProcesses() {
    let nvmlDeviceGetComputeRunningProcesses: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetComputeRunningProcesses"));
    eprintln!("[CALL] {}", "nvmlDeviceGetComputeRunningProcesses");
    nvmlDeviceGetComputeRunningProcesses()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByPciBusId() {
    let nvmlDeviceGetHandleByPciBusId: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetHandleByPciBusId"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleByPciBusId");
    nvmlDeviceGetHandleByPciBusId()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlThunkEscapeWithDevice() {
    let nvmlThunkEscapeWithDevice: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlThunkEscapeWithDevice"));
    eprintln!("[CALL] {}", "nvmlThunkEscapeWithDevice");
    nvmlThunkEscapeWithDevice()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByIndex() {
    let nvmlDeviceGetHandleByIndex: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetHandleByIndex"));
    eprintln!("[CALL] {}", "nvmlDeviceGetHandleByIndex");
    nvmlDeviceGetHandleByIndex()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmControl() {
    let nvmlEscapeRmControl: extern "C" fn() = std::mem::transmute(get_sym("nvmlEscapeRmControl"));
    eprintln!("[CALL] {}", "nvmlEscapeRmControl");
    nvmlEscapeRmControl()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAttributes() {
    let nvmlDeviceGetAttributes: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetAttributes"));
    eprintln!("[CALL] {}", "nvmlDeviceGetAttributes");
    nvmlDeviceGetAttributes()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmAlloc() {
    let nvmlEscapeRmAlloc: extern "C" fn() = std::mem::transmute(get_sym("nvmlEscapeRmAlloc"));
    eprintln!("[CALL] {}", "nvmlEscapeRmAlloc");
    nvmlEscapeRmAlloc()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkRemotePciInfo() {
    let nvmlDeviceGetNvLinkRemotePciInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetNvLinkRemotePciInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetNvLinkRemotePciInfo");
    nvmlDeviceGetNvLinkRemotePciInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceReadPRMCounters_v1() {
    let nvmlDeviceReadPRMCounters_v1: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceReadPRMCounters_v1"));
    eprintln!("[CALL] {}", "nvmlDeviceReadPRMCounters_v1");
    nvmlDeviceReadPRMCounters_v1()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMPSComputeRunningProcesses_v2() {
    let nvmlDeviceGetMPSComputeRunningProcesses_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetMPSComputeRunningProcesses_v2"));
    eprintln!("[CALL] {}", "nvmlDeviceGetMPSComputeRunningProcesses_v2");
    nvmlDeviceGetMPSComputeRunningProcesses_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlComputeInstanceGetInfo() {
    let nvmlComputeInstanceGetInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlComputeInstanceGetInfo"));
    eprintln!("[CALL] {}", "nvmlComputeInstanceGetInfo");
    nvmlComputeInstanceGetInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo() {
    let nvmlDeviceGetPciInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetPciInfo"));
    eprintln!("[CALL] {}", "nvmlDeviceGetPciInfo");
    nvmlDeviceGetPciInfo()
}
