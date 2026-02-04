#![allow(unused_imports)]
#![allow(nonstandard_style)]
use libc::*;

pub mod types;
pub use types::*;

pub mod dl;
pub use dl::*;

// The function we want to run at load time.
#[no_mangle]
pub extern "C" fn custom_init_function() {
    use std::str::FromStr;

    let level_filter = std::fs::read_to_string("/etc/dyll/options/level-filter")
        .map(|f| log::LevelFilter::from_str(&f.trim()).unwrap())
        .unwrap_or(log::LevelFilter::Debug);

    env_logger::builder().filter_level(level_filter).init();

    log::debug!("--- MOCK INIT ---");
}

// A static reference to the initialization function pointer is placed
// in the .init_array section using linker directives.
#[used]
// This attribute ensures the compiler doesn't optimize away the static item if it thinks it's unused.
#[link_section = ".init_array"]
pub static INITIALIZER: extern "C" fn() = custom_init_function;

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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetComputeInstanceProfileInfoV");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRowRemapperHistogram");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetGpuOperationMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetEncoderCapacity");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetPowerManagementLimit");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetGpcClkVfOffset");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetFrameRateLimit");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTargetFanSpeed");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetFrameRateLimit");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures_v4");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPerformanceState");
    nvmlDeviceGetPerformanceState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetCudaDriverVersion(arg0: *mut c_int) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlSystemGetCudaDriverVersion");
    // use the same impl.
    nvmlSystemGetCudaDriverVersion_v2(arg0)
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedGraphicsClocks");
    nvmlDeviceGetSupportedGraphicsClocks(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceClearAccountingPids(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceClearAccountingPids: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceClearAccountingPids"));
    log::debug!("[CALL] {}", "nvmlDeviceClearAccountingPids");
    nvmlDeviceClearAccountingPids(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetGpuLockedClocks(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceResetGpuLockedClocks: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceResetGpuLockedClocks"));
    log::debug!("[CALL] {}", "nvmlDeviceResetGpuLockedClocks");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleByPciBusId_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGspFirmwareMode");
    nvmlDeviceGetGspFirmwareMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceIsMigDeviceHandle(
    _: nvmlDevice_t,
    is_mig: *mut c_uint,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceIsMigDeviceHandle");
    // we aren't going to support mig devices.
    *is_mig = NVML_DEVICE_MIG_DISABLE;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceSetNvlinkBwMode");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetVmDriverVersion");
    nvmlVgpuInstanceGetVmDriverVersion(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetDriverVersion(
    arg0: *mut c_char,
    arg1: c_uint,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlSystemGetDriverVersion");
    std::ptr::copy_nonoverlapping("580.126.09\0".as_ptr() as *const c_char, arg0, arg1 as _);
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTotalEccErrors");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetMaxInstances");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetCapabilities");
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
    log::debug!("[CALL] {}", "nvmlUnitGetDevices");
    nvmlUnitGetDevices(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetIrqNum(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetIrqNum: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetIrqNum"));
    log::debug!("[CALL] {}", "nvmlDeviceGetIrqNum");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetEncoderStats");
    nvmlDeviceGetEncoderStats(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetApplicationsClocks(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceResetApplicationsClocks: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceResetApplicationsClocks"));
    log::debug!("[CALL] {}", "nvmlDeviceResetApplicationsClocks");
    nvmlDeviceResetApplicationsClocks(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeGpusReadyState(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeGpusReadyState: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetConfComputeGpusReadyState"));
    log::debug!("[CALL] {}", "nvmlSystemGetConfComputeGpusReadyState");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetConfComputeUnprotectedMemSize");
    nvmlDeviceSetConfComputeUnprotectedMemSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetCudaDriverVersion_v2(arg0: *mut c_int) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlSystemGetCudaDriverVersion_v2");
    *arg0 = 13200;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerManagementLimit");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDefaultApplicationsClock");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCurrPcieLinkWidth");
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
    log::debug!("[CALL] {}", "nvmlGetVgpuCompatibility");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerManagementLimitConstraints");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetMigMode");
    nvmlDeviceSetMigMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSampleFree(arg0: nvmlGpmSample_t) -> nvmlReturn_t {
    let nvmlGpmSampleFree: extern "C" fn(arg0: nvmlGpmSample_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmSampleFree"));
    log::debug!("[CALL] {}", "nvmlGpmSampleFree");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRunningProcessDetailList");
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
    log::debug!("[CALL] {}", "nvmlUnitGetLedState");
    nvmlUnitGetLedState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemEventSetWait(
    arg0: *mut nvmlSystemEventSetWaitRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemEventSetWait: extern "C" fn(
        arg0: *mut nvmlSystemEventSetWaitRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemEventSetWait"));
    log::debug!("[CALL] {}", "nvmlSystemEventSetWait");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstanceProfileInfoV");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetInforomConfigurationChecksum");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetTemperatureThreshold");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDriverModel_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkCapability");
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
    log::debug!("[CALL] {}", "nvmlGpmQueryIfStreamingEnabled");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMemoryErrorCounter");
    nvmlDeviceGetMemoryErrorCounter(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemEventSetCreate(
    arg0: *mut nvmlSystemEventSetCreateRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemEventSetCreate: extern "C" fn(
        arg0: *mut nvmlSystemEventSetCreateRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemEventSetCreate"));
    log::debug!("[CALL] {}", "nvmlSystemEventSetCreate");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetEncoderSessions");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetEccMode");
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
    log::debug!("[CALL] {}", "nvmlGpmMigSampleGet");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetPlacementId");
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
    log::debug!("[CALL] {}", "nvmlUnitGetFanSpeedInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetInforomImageVersion");
    nvmlDeviceGetInforomImageVersion(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMaxMigDeviceCount(
    _: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetMaxMigDeviceCount");
    *arg1 = 0;
    NVML_SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstancesPerGpuInstance(
    arg0: *mut nvmlVgpuTypeMaxInstance_t,
) -> nvmlReturn_t {
    let nvmlVgpuTypeGetMaxInstancesPerGpuInstance: extern "C" fn(
        arg0: *mut nvmlVgpuTypeMaxInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuTypeGetMaxInstancesPerGpuInstance"));
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetMaxInstancesPerGpuInstance");
    nvmlVgpuTypeGetMaxInstancesPerGpuInstance(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCudaComputeCapability(
    arg0: nvmlDevice_t,
    arg1: *mut c_int,
    arg2: *mut c_int,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetCudaComputeCapability");
    // cuda compute 12.0 added with blackwell support.
    *arg1 = 12;
    *arg2 = 0;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPciInfoExt");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetDefaultAutoBoostedClocksEnabled");
    nvmlDeviceSetDefaultAutoBoostedClocksEnabled(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceClearAccountingPids(
    arg0: nvmlVgpuInstance_t,
) -> nvmlReturn_t {
    let nvmlVgpuInstanceClearAccountingPids: extern "C" fn(
        arg0: nvmlVgpuInstance_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlVgpuInstanceClearAccountingPids"));
    log::debug!("[CALL] {}", "nvmlVgpuInstanceClearAccountingPids");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPcieReplayCounter");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetJpgUtilization");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetVgpuCapabilities");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMinMaxFanSpeed");
    nvmlDeviceGetMinMaxFanSpeed(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetUUID(
    dev: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    // turns the pointer value of the device handle into a UUID, that way any device coming through
    // gets its own unique id.
    let uuid = uuid::Uuid::from_u128(dev.addr() as _).to_string();
    log::debug!("[CALL] {}", "nvmlDeviceGetUUID");
    std::ptr::copy_nonoverlapping(
        format!("GPU-{}\0", uuid).as_ptr() as *const c_char,
        arg1,
        arg2 as _,
    );
    NVML_SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetNvlinkBwMode(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlSystemGetNvlinkBwMode: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetNvlinkBwMode"));
    log::debug!("[CALL] {}", "nvmlSystemGetNvlinkBwMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCurrentClocksEventReasons");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetUUID");
    nvmlVgpuInstanceGetUUID(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInit_v2() -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlInit_v2");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAutoBoostedClocksEnabled");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetApplicationsClock");
    nvmlDeviceGetApplicationsClock(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPdi(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlPdi_t,
) -> nvmlReturn_t {
    let nvmlDeviceGetPdi: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut nvmlPdi_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetPdi"));
    log::debug!("[CALL] {}", "nvmlDeviceGetPdi");
    nvmlDeviceGetPdi(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceValidateInforom(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceValidateInforom: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceValidateInforom"));
    log::debug!("[CALL] {}", "nvmlDeviceValidateInforom");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMaxClockInfo");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetRuntimeStateSize");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetUtilizationRates");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuSchedulerLog");
    nvmlDeviceGetVgpuSchedulerLog(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMinorNumber(
    _: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetMinorNumber");
    // TODO: this should should really be sequential per GPU
    *arg1 = 0;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceSetPowerMizerMode_v1");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetClockOffsets");
    nvmlDeviceGetClockOffsets(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetExcludedDeviceCount(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlGetExcludedDeviceCount: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGetExcludedDeviceCount"));
    log::debug!("[CALL] {}", "nvmlGetExcludedDeviceCount");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetViolationStatus");
    nvmlDeviceGetViolationStatus(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkState(
    _: nvmlDevice_t,
    _: c_uint,
    arg2: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkState");
    // TODO: check which is best to use for now
    *arg2 = NVML_NVLINK_STATE_INACTIVE;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetEncoderStats");
    nvmlVgpuInstanceGetEncoderStats(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSetVgpuVersion(arg0: *mut nvmlVgpuVersion_t) -> nvmlReturn_t {
    let nvmlSetVgpuVersion: extern "C" fn(arg0: *mut nvmlVgpuVersion_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSetVgpuVersion"));
    log::debug!("[CALL] {}", "nvmlSetVgpuVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRetiredPagesPendingStatus");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetClass");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuCapabilities");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetMemoryLockedClocks");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuMetadata");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleBySerial");
    nvmlDeviceGetHandleBySerial(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInitWithFlags(_arg0: c_uint) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlInitWithFlags");
    NVML_SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeState(
    arg0: *mut nvmlConfComputeSystemState_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeState: extern "C" fn(
        arg0: *mut nvmlConfComputeSystemState_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetConfComputeState"));
    log::debug!("[CALL] {}", "nvmlSystemGetConfComputeState");
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
    log::debug!("[CALL] {}", "nvmlDeviceFreezeNvLinkUtilizationCounter");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetFbUsage");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkVersion");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceGetConfComputeProtectedMemoryUsage");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedEventTypes");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCurrentClockFreqs");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetModuleId");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetFanControlPolicy");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetLicenseStatus");
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
    log::debug!("[CALL] {}", "nvmlGetVgpuDriverCapabilities");
    nvmlGetVgpuDriverCapabilities(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetCreate(_arg0: *mut nvmlEventSet_t) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlEventSetCreate");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetBoardPartNumber");
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
    log::debug!("[CALL] {}", "nvmlDeviceCreateGpuInstance");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceSetEncoderCapacity");
    nvmlVgpuInstanceSetEncoderCapacity(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByIndex_v2(
    index: c_uint,
    dev: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleByIndex_v2");
    // we give each device a handle based on the index that is comes in as.
    //
    // WARNING: add 1 because if we end up using 0 that means a null pointer!
    let ptr = (index as *mut u32).wrapping_add(1);
    *dev = ptr.addr() as _;
    log::debug!(
        "[CALL] {}: gpu index {} given id {:?}",
        "nvmlDeviceGetHandleByIndex_v2",
        index,
        *dev
    );
    NVML_SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeCapabilities(
    arg0: *mut nvmlConfComputeSystemCaps_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeCapabilities: extern "C" fn(
        arg0: *mut nvmlConfComputeSystemCaps_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetConfComputeCapabilities"));
    log::debug!("[CALL] {}", "nvmlSystemGetConfComputeCapabilities");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvlinkBwMode");
    nvmlDeviceGetNvlinkBwMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetConfComputeSettings(
    arg0: *mut nvmlSystemConfComputeSettings_t,
) -> nvmlReturn_t {
    let nvmlSystemGetConfComputeSettings: extern "C" fn(
        arg0: *mut nvmlSystemConfComputeSettings_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemGetConfComputeSettings"));
    log::debug!("[CALL] {}", "nvmlSystemGetConfComputeSettings");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetFbReservation");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetComputeInstanceById");
    nvmlGpuInstanceGetComputeInstanceById(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMemoryInfo(
    arg0: nvmlDevice_t,
    arg1: *mut nvmlMemory_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetMemoryInfo");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRetiredPages_v2");
    nvmlDeviceGetRetiredPages_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemGetNVMLVersion(arg0: *mut c_char, arg1: c_uint) -> nvmlReturn_t {
    let nvmlSystemGetNVMLVersion: extern "C" fn(arg0: *mut c_char, arg1: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemGetNVMLVersion"));
    log::debug!("[CALL] {}", "nvmlSystemGetNVMLVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedMemoryClocks");
    nvmlDeviceGetSupportedMemoryClocks(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceDiscoverGpus(arg0: *mut nvmlPciInfo_t) -> nvmlReturn_t {
    let nvmlDeviceDiscoverGpus: extern "C" fn(arg0: *mut nvmlPciInfo_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceDiscoverGpus"));
    log::debug!("[CALL] {}", "nvmlDeviceDiscoverGpus");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetFBCStats");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSerial");
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
    log::debug!("[CALL] {}", "nvmlUnitGetUnitInfo");
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
    log::debug!("[CALL] {}", "nvmlDevicePowerSmoothingActivatePresetProfile");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAccountingMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCapabilities");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMultiGpuBoard");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuProcessUtilization");
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
    log::debug!("[CALL] {}", "nvmlDeviceModifyDrainState");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetConfComputeGpuCertificate");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceSetVgpuSchedulerState");
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
    log::debug!("[CALL] nvmlDeviceGetFieldValues");

    for i in 0..arg1 as usize {
        let ptr = arg2.add(i);
        let f = &mut *ptr;
        match f.fieldId {
            NVML_FI_DEV_NVLINK_LINK_COUNT => {
                // #define NVML_FI_DEV_NVLINK_LINK_COUNT 91
                //
                // see: https://docs.nvidia.com/deploy/archive/R520/nvml-api/group__nvmlFieldValueEnums.html
                f.nvmlReturn = NVML_SUCCESS;
                f.value = nvmlValue_st { usVal: 18 };
            }
            NVML_FI_DEV_NVLINK_GET_STATE => {
                f.nvmlReturn = NVML_SUCCESS;
                f.value = nvmlValue_st {
                    uiVal: NVML_NVLINK_STATE_ACTIVE,
                };
            }
            NVML_FI_DEV_NVLINK_GET_SPEED => {
                f.nvmlReturn = NVML_SUCCESS;
                f.value = nvmlValue_st {
                    // stored in MB/s instead of GB/s which is what the tooling displays.
                    uiVal: 200_000,
                };
            }
            _ => {
                // log the field idea for determining its purpose/usage.
                log::debug!("fieldId = {}", f.fieldId);

                // restructure the call to pull the top value.
                nvmlDeviceGetFieldValues(arg0, 1, ptr);
            }
        }
    }

    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceSetPersistenceMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceQueryDrainState");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetAccountingStats");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkErrorCounter");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetFBCStats");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCurrPcieLinkGeneration");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuSchedulerState");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetThermalSettings");
    nvmlDeviceGetThermalSettings(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlComputeInstanceDestroy(arg0: nvmlComputeInstance_t) -> nvmlReturn_t {
    let nvmlComputeInstanceDestroy: extern "C" fn(arg0: nvmlComputeInstance_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlComputeInstanceDestroy"));
    log::debug!("[CALL] {}", "nvmlComputeInstanceDestroy");
    nvmlComputeInstanceDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceClearCpuAffinity(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceClearCpuAffinity: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceClearCpuAffinity"));
    log::debug!("[CALL] {}", "nvmlDeviceClearCpuAffinity");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetProcessesUtilizationInfo");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetDeviceID");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCoolerInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceRemoveGpu_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetInforomVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstanceId");
    nvmlDeviceGetGpuInstanceId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByUUIDV(
    _: *const nvmlUUID_t,
    _: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleByUUIDV");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedVgpus");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetNvLinkUtilizationControl");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMPSComputeRunningProcesses_v3");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDynamicPstatesInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuSchedulerCapabilities");
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
    log::debug!("[CALL] {}", "nvmlDeviceOnSameBoard");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstanceById");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTemperatureV");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPcieSpeed");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetMdevUUID");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTopologyNearestGpus");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetFanSpeedRPM");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAddressingMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstancePossiblePlacements_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpcClkVfOffset");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMemClkVfOffset");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetBridgeChipInfo");
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
    log::debug!("[CALL] {}", "nvmlGetExcludedDeviceInfoByIndex");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetLastBBXFlushTime");
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
    log::debug!("[CALL] {}", "nvmlSystemGetDriverBranch");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetEccMode");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceGetComputeInstanceId");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAttributes_v2");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetAccountingPids");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetComputeMode");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetMaxInstancesPerVm");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerMizerMode_v1");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkRemotePciInfo_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuMaxPcieLinkGeneration");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRetiredPages");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetEnforcedPowerLimit");
    nvmlDeviceGetEnforcedPowerLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo_v3(
    _arg0: nvmlDevice_t,
    _arg1: *mut nvmlPciInfo_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetPciInfo_v3");
    *_arg1 = nvmlPciInfo_st {
        bus: 1,
        busId: [1; 32],
        busIdLegacy: [0; 16],
        domain: 4242,
        device: 4242,
        // These are Mellanox Technologies CX8 Family values.
        pciDeviceId: 0x15b3,
        pciSubSystemId: 0x0008,
    };
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetOfaUtilization");
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
    log::debug!(
        "[CALL] {}",
        "nvmlGpuInstanceGetComputeInstanceRemainingCapacity"
    );
    nvmlGpuInstanceGetComputeInstanceRemainingCapacity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuFabricInfo(
    _: nvmlDevice_t,
    arg1: *mut nvmlGpuFabricInfo_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuFabricInfo");
    *arg1 = nvmlGpuFabricInfo_t {
        clusterUuid: [42; 16],
        status: NVML_SUCCESS,
        cliqueId: 4242,
        state: NVML_GPU_FABRIC_STATE_COMPLETED as _,
    };
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetBrand");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPlatformInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCreatableVgpus");
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
    log::debug!("[CALL] {}", "nvmlSystemGetProcessName");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPersistenceMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetArchitecture");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCpuAffinityWithinScope");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMarginTemperature");
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
    log::debug!("[CALL] {}", "nvmlDeviceCreateGpuInstanceWithPlacement");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvlinkSupportedBwModes");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetDefaultFanSpeed_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceClearFieldValues");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRemappedRows");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTotalEnergyConsumption");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetClockInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuInstancesUtilizationInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTopologyCommonAncestor");
    nvmlDeviceGetTopologyCommonAncestor(arg0, arg1, arg2);
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceSetFanSpeed_v2");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetComputeInstances");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetBusType");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuProcessesUtilizationInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMaxPcieLinkWidth");
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
    log::debug!("[CALL] {}", "nvmlUnitGetPsuInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetConfComputeGpuAttestationReport");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetActiveVgpus");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGraphicsRunningProcesses_v3");
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
    log::debug!("[CALL] {}", "nvmlGpmSetStreamingEnabled");
    nvmlGpmSetStreamingEnabled(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetVirtualizationMode(
    _: nvmlDevice_t,
    arg1: *mut nvmlGpuVirtualizationMode_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetVirtualizationMode");
    // not supporting this feature.
    *arg1 = NVML_GPU_VIRTUALIZATION_MODE_NONE;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerSource");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetComputeMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAccountingBufferSize");
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
    log::debug!("[CALL] {}", "nvmlComputeInstanceGetInfo_v2");
    nvmlComputeInstanceGetInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCount_v2(arg0: *mut c_uint) -> nvmlReturn_t {
    *arg0 = 4;
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceRegisterEvents");
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
    log::debug!("[CALL] {}", "nvmlUnitGetTemperature");
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
    log::debug!("[CALL] {}", "nvmlGpmQueryDeviceSupport");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMemoryBusWidth");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkRemoteDeviceType");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetVgpuTypeCreatablePlacements");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerUsage");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedPerformanceStates");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkUtilizationCounter");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetFBCSessions");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetVgpuSchedulerState");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNumaNodeId");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDisplayActive");
    nvmlDeviceGetDisplayActive(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByUUID(
    _: *const c_char,
    _: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleByUUID");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDetailedEccErrors");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetConfComputeMemSizeInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedClocksThrottleReasons");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuTypeSupportedPlacements");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetClkMonStatus");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetEncoderUtilization");
    nvmlDeviceGetEncoderUtilization(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemEventSetFree(
    arg0: *mut nvmlSystemEventSetFreeRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemEventSetFree: extern "C" fn(
        arg0: *mut nvmlSystemEventSetFreeRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemEventSetFree"));
    log::debug!("[CALL] {}", "nvmlSystemEventSetFree");
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
    log::debug!("[CALL] {}", "nvmlEventSetWait_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVbiosVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTemperature");
    nvmlDeviceGetTemperature(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuFabricInfoV(
    _arg0: nvmlDevice_t,
    arg1: *mut nvmlGpuFabricInfoV_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuFabricInfoV");
    *arg1 = nvmlGpuFabricInfo_v3_t {
        version: 0,
        clusterUuid: [42; 16],
        status: NVML_SUCCESS,
        cliqueId: 4242,
        state: NVML_GPU_FABRIC_STATE_COMPLETED as _,
        healthMask: 0,
        healthSummary: NVML_GPU_FABRIC_HEALTH_SUMMARY_HEALTHY as _,
    };
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetC2cModeInfoV");
    nvmlDeviceGetC2cModeInfoV(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemSetNvlinkBwMode(arg0: c_uint) -> nvmlReturn_t {
    let nvmlSystemSetNvlinkBwMode: extern "C" fn(arg0: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemSetNvlinkBwMode"));
    log::debug!("[CALL] {}", "nvmlSystemSetNvlinkBwMode");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetComputeInstanceProfileInfo");
    nvmlGpuInstanceGetComputeInstanceProfileInfo(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmMetricsGet(arg0: *mut nvmlGpmMetricsGet_t) -> nvmlReturn_t {
    let nvmlGpmMetricsGet: extern "C" fn(arg0: *mut nvmlGpmMetricsGet_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmMetricsGet"));
    log::debug!("[CALL] {}", "nvmlGpmMetricsGet");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetMemClkVfOffset");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMigMode");
    nvmlDeviceGetMigMode(arg0, arg1, arg2);
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceResetNvLinkErrorCounters");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetTemperatureThreshold");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetEncoderCapacity");
    nvmlVgpuInstanceGetEncoderCapacity(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemSetConfComputeGpusReadyState(arg0: c_uint) -> nvmlReturn_t {
    let nvmlSystemSetConfComputeGpusReadyState: extern "C" fn(arg0: c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlSystemSetConfComputeGpusReadyState"));
    log::debug!("[CALL] {}", "nvmlSystemSetConfComputeGpusReadyState");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceCreateComputeInstance");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetFanControlPolicy_v2");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetVgpuSchedulerState");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDefaultEccMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerManagementMode");
    nvmlDeviceGetPowerManagementMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetFree(arg0: nvmlEventSet_t) -> nvmlReturn_t {
    let nvmlEventSetFree: extern "C" fn(arg0: nvmlEventSet_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlEventSetFree"));
    log::debug!("[CALL] {}", "nvmlEventSetFree");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetBAR1Info");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetClock");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetFanSpeed");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDeviceHandleFromMigDeviceHandle");
    nvmlDeviceGetDeviceHandleFromMigDeviceHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetCpuAffinity(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceSetCpuAffinity: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceSetCpuAffinity"));
    log::debug!("[CALL] {}", "nvmlDeviceSetCpuAffinity");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetAccountingMode");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceSetClockOffsets");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDisplayMode");
    nvmlDeviceGetDisplayMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlSystemRegisterEvents(
    arg0: *mut nvmlSystemRegisterEventRequest_t,
) -> nvmlReturn_t {
    let nvmlSystemRegisterEvents: extern "C" fn(
        arg0: *mut nvmlSystemRegisterEventRequest_t,
    ) -> nvmlReturn_t = std::mem::transmute(get_sym("nvmlSystemRegisterEvents"));
    log::debug!("[CALL] {}", "nvmlSystemRegisterEvents");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetGspHeapSize");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetAutoBoostedClocksEnabled");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerState");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetBAR1MemoryInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPerformanceModes");
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
    log::debug!("[CALL] {}", "nvmlDeviceWorkloadPowerProfileGetProfilesInfo");
    nvmlDeviceWorkloadPowerProfileGetProfilesInfo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlShutdown() -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlShutdown");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetType");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetGpuInstanceProfileId");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCurrentClocksThrottleReasons");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstanceProfileInfoByIdV");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetP2PStatus");
    // TODO: configure overrides
    nvmlDeviceGetP2PStatus(arg0, arg1, arg2, arg3);
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMemClkMinMaxVfOffset");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetAccountingMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetComputeRunningProcesses_v3");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetCpuAffinity");
    nvmlDeviceGetCpuAffinity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkInfo(
    _: nvmlDevice_t,
    arg1: *mut nvmlNvLinkInfo_t,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkInfo");
    *arg1 = nvmlNvLinkInfo_v2_t {
        version: 2,
        isNvleEnabled: 0, //?
        firmwareInfo: nvmlNvlinkFirmwareInfo_t {
            // TODO:
            numValidEntries: 1,
            // 1 valid entries, but we don't know exactly waht should go here.
            firmwareVersion: [nvmlNvlinkFirmwareVersion_t {
                major: 1,
                ucodeType: 1,
                minor: 1,
                subMinor: 0,
            }; 100],
        },
    };
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetHostVgpuMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceClearEccErrorCounts");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAPIRestriction");
    nvmlDeviceGetAPIRestriction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlErrorString(arg0: nvmlReturn_t) -> *const c_char {
    let nvmlErrorString: extern "C" fn(arg0: nvmlReturn_t) -> *const c_char =
        std::mem::transmute(get_sym("nvmlErrorString"));
    log::debug!("[CALL] {}", "nvmlErrorString");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetVgpuHeterogeneousMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetProcessUtilization");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetNvLinkDeviceLowPowerThreshold");
    nvmlDeviceSetNvLinkDeviceLowPowerThreshold(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNumFans(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetNumFans: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetNumFans"));
    log::debug!("[CALL] {}", "nvmlDeviceGetNumFans");
    nvmlDeviceGetNumFans(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSampleAlloc(_: *mut nvmlGpmSample_t) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlGpmSampleAlloc");
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuTypeCreatablePlacements");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMinMaxClockOfPState");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetNumDisplayHeads");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetFBCSessions");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetEccMode");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceSetDriverModel");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetLicenseInfo_v2");
    nvmlVgpuInstanceGetLicenseInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitSetLedState(
    arg0: nvmlUnit_t,
    arg1: nvmlLedColor_t,
) -> nvmlReturn_t {
    let nvmlUnitSetLedState: extern "C" fn(arg0: nvmlUnit_t, arg1: nvmlLedColor_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlUnitSetLedState"));
    log::debug!("[CALL] {}", "nvmlUnitSetLedState");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetCreatableVgpus");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDecoderUtilization");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkUtilizationControl");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetApplicationsClocks");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetMetadata");
    nvmlVgpuInstanceGetMetadata(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetIndex(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t {
    let nvmlDeviceGetIndex: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetIndex"));
    log::debug!("[CALL] {}", "nvmlDeviceGetIndex");
    nvmlDeviceGetIndex(arg0, arg1);
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpcClkMinMaxVfOffset");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetFanSpeed_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetRepairStatus");
    nvmlDeviceGetRepairStatus(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpmSampleGet(
    arg0: nvmlDevice_t,
    arg1: nvmlGpmSample_t,
) -> nvmlReturn_t {
    let nvmlGpmSampleGet: extern "C" fn(arg0: nvmlDevice_t, arg1: nvmlGpmSample_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpmSampleGet"));
    log::debug!("[CALL] {}", "nvmlGpmSampleGet");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPowerManagementDefaultLimit");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGspFirmwareVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMemoryInfo_v2");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetGpuPciId");
    nvmlVgpuInstanceGetGpuPciId(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGpuInstanceDestroy(arg0: nvmlGpuInstance_t) -> nvmlReturn_t {
    let nvmlGpuInstanceDestroy: extern "C" fn(arg0: nvmlGpuInstance_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlGpuInstanceDestroy"));
    log::debug!("[CALL] {}", "nvmlGpuInstanceDestroy");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetGpuInstanceId");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetName");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetVirtualizationMode");
    nvmlDeviceSetVirtualizationMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlUnitGetCount(arg0: *mut c_uint) -> nvmlReturn_t {
    let nvmlUnitGetCount: extern "C" fn(arg0: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlUnitGetCount"));
    log::debug!("[CALL] {}", "nvmlUnitGetCount");
    nvmlUnitGetCount(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetBoardId(
    arg0: nvmlDevice_t,
    arg1: *mut c_uint,
) -> nvmlReturn_t {
    let nvmlDeviceGetBoardId: extern "C" fn(arg0: nvmlDevice_t, arg1: *mut c_uint) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceGetBoardId"));
    log::debug!("[CALL] {}", "nvmlDeviceGetBoardId");
    nvmlDeviceGetBoardId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceResetMemoryLockedClocks(arg0: nvmlDevice_t) -> nvmlReturn_t {
    let nvmlDeviceResetMemoryLockedClocks: extern "C" fn(arg0: nvmlDevice_t) -> nvmlReturn_t =
        std::mem::transmute(get_sym("nvmlDeviceResetMemoryLockedClocks"));
    log::debug!("[CALL] {}", "nvmlDeviceResetMemoryLockedClocks");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetPowerManagementLimit_v2");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSramEccErrorStatus");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuOperationMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetDramEncryptionMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuHeterogeneousMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetGpuLockedClocks");
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
    log::debug!("[CALL] {}", "nvmlDevicePowerSmoothingSetState");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPgpuMetadataString");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSupportedClocksEventReasons");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetFramebufferSize");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetNumGpuCores");
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
    log::debug!("[CALL] {}", "nvmlUnitGetHandleByIndex");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMaxPcieLinkGeneration");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetEncoderSessions");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetSamples");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMaxCustomerBoostClock");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPcieThroughput");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetPcieLinkMaxSpeed");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstanceProfileInfo");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMigDeviceHandleByIndex");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetActiveVgpus");
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
    log::debug!("[CALL] {}", "nvmlDeviceResetNvLinkUtilizationCounter");
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
    log::debug!("[CALL] {}", "nvmlSystemGetHicVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstances");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetVgpuHeterogeneousMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetAPIRestriction");
    nvmlDeviceSetAPIRestriction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetName(
    _: nvmlDevice_t,
    arg1: *mut c_char,
    arg2: c_uint,
) -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlDeviceGetName");
    std::ptr::copy_nonoverlapping("fake-device\0".as_ptr() as *const c_char, arg1, arg2 as _);
    NVML_SUCCESS
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAccountingStats");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetResolution");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceSetVgpuHeterogeneousMode");
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
    log::debug!("[CALL] {}", "nvmlGpuInstanceGetVgpuSchedulerLog");
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
    log::debug!("[CALL] {}", "nvmlVgpuTypeGetLicense");
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
    log::debug!("[CALL] {}", "nvmlSystemGetTopologyGpuSet");
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
    log::debug!("[CALL] {}", "nvmlDeviceSetDramEncryptionMode");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAdaptiveClockInfoStatus");
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
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetVmID");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetAccountingPids");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlDeviceReadWritePRM_v1");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "nvmlGetVgpuVersion");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstanceRemainingCapacity");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetMemoryAffinity");
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
    log::debug!("[CALL] {}", "nvmlDeviceGetVgpuUtilization");
    nvmlDeviceGetVgpuUtilization(arg0, arg1, arg2, arg3, arg4)
}

// Unknown function stubs (functions not found in header)
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGraphicsRunningProcesses() {
    let nvmlDeviceGetGraphicsRunningProcesses: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGraphicsRunningProcesses"));
    log::debug!("[CALL] {}", "nvmlDeviceGetGraphicsRunningProcesses");
    nvmlDeviceGetGraphicsRunningProcesses()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures_v2() {
    let nvmlDeviceGetGridLicensableFeatures_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures_v2"));
    log::debug!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures_v2");
    nvmlDeviceGetGridLicensableFeatures_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEventSetWait() {
    let nvmlEventSetWait: extern "C" fn() = std::mem::transmute(get_sym("nvmlEventSetWait"));
    log::debug!("[CALL] {}", "nvmlEventSetWait");
    nvmlEventSetWait()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo_v2() {
    let nvmlDeviceGetPciInfo_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetPciInfo_v2"));
    log::debug!("[CALL] {}", "nvmlDeviceGetPciInfo_v2");
    nvmlDeviceGetPciInfo_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetDriverModel() {
    let nvmlDeviceGetDriverModel: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetDriverModel"));
    log::debug!("[CALL] {}", "nvmlDeviceGetDriverModel");
    nvmlDeviceGetDriverModel()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHostname_v1() {
    let nvmlDeviceGetHostname_v1: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetHostname_v1"));
    log::debug!("[CALL] {}", "nvmlDeviceGetHostname_v1");
    nvmlDeviceGetHostname_v1()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeGetProcessInfo() {
    let nvmlEscapeGetProcessInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlEscapeGetProcessInfo"));
    log::debug!("[CALL] {}", "nvmlEscapeGetProcessInfo");
    nvmlEscapeGetProcessInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGraphicsRunningProcesses_v2() {
    let nvmlDeviceGetGraphicsRunningProcesses_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGraphicsRunningProcesses_v2"));
    log::debug!("[CALL] {}", "nvmlDeviceGetGraphicsRunningProcesses_v2");
    nvmlDeviceGetGraphicsRunningProcesses_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetBlacklistDeviceInfoByIndex() {
    let nvmlGetBlacklistDeviceInfoByIndex: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlGetBlacklistDeviceInfoByIndex"));
    log::debug!("[CALL] {}", "nvmlGetBlacklistDeviceInfoByIndex");
    nvmlGetBlacklistDeviceInfoByIndex()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMPSComputeRunningProcesses() {
    let nvmlDeviceGetMPSComputeRunningProcesses: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetMPSComputeRunningProcesses"));
    log::debug!("[CALL] {}", "nvmlDeviceGetMPSComputeRunningProcesses");
    nvmlDeviceGetMPSComputeRunningProcesses()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInternalGetExportTable() -> nvmlReturn_t {
    log::debug!("[CALL] {}", "nvmlInternalGetExportTable");
    NVML_SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetCount() {
    let nvmlDeviceGetCount: extern "C" fn() = std::mem::transmute(get_sym("nvmlDeviceGetCount"));
    log::debug!("[CALL] {}", "nvmlDeviceGetCount");
    nvmlDeviceGetCount()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlVgpuInstanceGetLicenseInfo() {
    let nvmlVgpuInstanceGetLicenseInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlVgpuInstanceGetLicenseInfo"));
    log::debug!("[CALL] {}", "nvmlVgpuInstanceGetLicenseInfo");
    nvmlVgpuInstanceGetLicenseInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceSetHostname_v1() {
    let nvmlDeviceSetHostname_v1: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceSetHostname_v1"));
    log::debug!("[CALL] {}", "nvmlDeviceSetHostname_v1");
    nvmlDeviceSetHostname_v1()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlInit() {
    log::debug!("[CALL] {}", "nvmlInit");
    // NOOP
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeRunningProcesses_v2() {
    let nvmlDeviceGetComputeRunningProcesses_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetComputeRunningProcesses_v2"));
    log::debug!("[CALL] {}", "nvmlDeviceGetComputeRunningProcesses_v2");
    nvmlDeviceGetComputeRunningProcesses_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlThunkInit() {
    let nvmlThunkInit: extern "C" fn() = std::mem::transmute(get_sym("nvmlThunkInit"));
    log::debug!("[CALL] {}", "nvmlThunkInit");
    nvmlThunkInit()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstancePossiblePlacements() {
    let nvmlDeviceGetGpuInstancePossiblePlacements: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGpuInstancePossiblePlacements"));
    log::debug!("[CALL] {}", "nvmlDeviceGetGpuInstancePossiblePlacements");
    nvmlDeviceGetGpuInstancePossiblePlacements()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmFree() {
    let nvmlEscapeRmFree: extern "C" fn() = std::mem::transmute(get_sym("nvmlEscapeRmFree"));
    log::debug!("[CALL] {}", "nvmlEscapeRmFree");
    nvmlEscapeRmFree()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlThunkEscape() {
    let nvmlThunkEscape: extern "C" fn() = std::mem::transmute(get_sym("nvmlThunkEscape"));
    log::debug!("[CALL] {}", "nvmlThunkEscape");
    nvmlThunkEscape()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlRetry_NvRmControl() {
    let nvmlRetry_NvRmControl: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlRetry_NvRmControl"));
    log::debug!("[CALL] {}", "nvmlRetry_NvRmControl");
    nvmlRetry_NvRmControl()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures() {
    let nvmlDeviceGetGridLicensableFeatures: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures"));
    log::debug!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures");
    nvmlDeviceGetGridLicensableFeatures()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlGetBlacklistDeviceCount() {
    let nvmlGetBlacklistDeviceCount: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlGetBlacklistDeviceCount"));
    log::debug!("[CALL] {}", "nvmlGetBlacklistDeviceCount");
    nvmlGetBlacklistDeviceCount()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures_v3() {
    let nvmlDeviceGetGridLicensableFeatures_v3: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetGridLicensableFeatures_v3"));
    log::debug!("[CALL] {}", "nvmlDeviceGetGridLicensableFeatures_v3");
    nvmlDeviceGetGridLicensableFeatures_v3()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmAllocRoot() {
    let nvmlEscapeRmAllocRoot: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlEscapeRmAllocRoot"));
    log::debug!("[CALL] {}", "nvmlEscapeRmAllocRoot");
    nvmlEscapeRmAllocRoot()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeGetMemoryInfo() {
    let nvmlEscapeGetMemoryInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlEscapeGetMemoryInfo"));
    log::debug!("[CALL] {}", "nvmlEscapeGetMemoryInfo");
    nvmlEscapeGetMemoryInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceRemoveGpu() {
    let nvmlDeviceRemoveGpu: extern "C" fn() = std::mem::transmute(get_sym("nvmlDeviceRemoveGpu"));
    log::debug!("[CALL] {}", "nvmlDeviceRemoveGpu");
    nvmlDeviceRemoveGpu()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetComputeRunningProcesses() {
    let nvmlDeviceGetComputeRunningProcesses: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetComputeRunningProcesses"));
    log::debug!("[CALL] {}", "nvmlDeviceGetComputeRunningProcesses");
    nvmlDeviceGetComputeRunningProcesses()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByPciBusId() {
    let nvmlDeviceGetHandleByPciBusId: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetHandleByPciBusId"));
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleByPciBusId");
    nvmlDeviceGetHandleByPciBusId()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlThunkEscapeWithDevice() {
    let nvmlThunkEscapeWithDevice: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlThunkEscapeWithDevice"));
    log::debug!("[CALL] {}", "nvmlThunkEscapeWithDevice");
    nvmlThunkEscapeWithDevice()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetHandleByIndex() {
    let nvmlDeviceGetHandleByIndex: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetHandleByIndex"));
    log::debug!("[CALL] {}", "nvmlDeviceGetHandleByIndex");
    nvmlDeviceGetHandleByIndex()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmControl() {
    let nvmlEscapeRmControl: extern "C" fn() = std::mem::transmute(get_sym("nvmlEscapeRmControl"));
    log::debug!("[CALL] {}", "nvmlEscapeRmControl");
    nvmlEscapeRmControl()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetAttributes() {
    let nvmlDeviceGetAttributes: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetAttributes"));
    log::debug!("[CALL] {}", "nvmlDeviceGetAttributes");
    nvmlDeviceGetAttributes()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlEscapeRmAlloc() {
    let nvmlEscapeRmAlloc: extern "C" fn() = std::mem::transmute(get_sym("nvmlEscapeRmAlloc"));
    log::debug!("[CALL] {}", "nvmlEscapeRmAlloc");
    nvmlEscapeRmAlloc()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkRemotePciInfo() {
    let nvmlDeviceGetNvLinkRemotePciInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetNvLinkRemotePciInfo"));
    log::debug!("[CALL] {}", "nvmlDeviceGetNvLinkRemotePciInfo");
    nvmlDeviceGetNvLinkRemotePciInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceReadPRMCounters_v1() {
    let nvmlDeviceReadPRMCounters_v1: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceReadPRMCounters_v1"));
    log::debug!("[CALL] {}", "nvmlDeviceReadPRMCounters_v1");
    nvmlDeviceReadPRMCounters_v1()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetMPSComputeRunningProcesses_v2() {
    let nvmlDeviceGetMPSComputeRunningProcesses_v2: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetMPSComputeRunningProcesses_v2"));
    log::debug!("[CALL] {}", "nvmlDeviceGetMPSComputeRunningProcesses_v2");
    nvmlDeviceGetMPSComputeRunningProcesses_v2()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlComputeInstanceGetInfo() {
    let nvmlComputeInstanceGetInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlComputeInstanceGetInfo"));
    log::debug!("[CALL] {}", "nvmlComputeInstanceGetInfo");
    nvmlComputeInstanceGetInfo()
}
#[no_mangle]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo() {
    let nvmlDeviceGetPciInfo: extern "C" fn() =
        std::mem::transmute(get_sym("nvmlDeviceGetPciInfo"));
    log::debug!("[CALL] {}", "nvmlDeviceGetPciInfo");
    nvmlDeviceGetPciInfo()
}
