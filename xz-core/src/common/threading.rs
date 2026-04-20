use crate::types::{c_char, c_int, c_long, c_uint, c_void};

#[cfg(windows)]
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
#[cfg(windows)]
use windows_sys::Win32::System::SystemInformation::GetTickCount;
#[cfg(windows)]
use windows_sys::Win32::System::Threading::{
    CONDITION_VARIABLE, CRITICAL_SECTION, DeleteCriticalSection, EnterCriticalSection, INFINITE,
    InitializeConditionVariable, InitializeCriticalSection, LeaveCriticalSection,
    SleepConditionVariableCS, WaitForSingleObject, WakeConditionVariable,
};

#[cfg(windows)]
unsafe extern "C" {
    fn _beginthreadex(
        security: *mut c_void,
        stack_size: c_uint,
        start_address: Option<unsafe extern "system" fn(*mut c_void) -> u32>,
        arglist: *mut c_void,
        initflag: c_uint,
        thrdaddr: *mut c_uint,
    ) -> usize;
}

pub const MYTHREAD_RET_VALUE: *mut c_void = core::ptr::null_mut();
pub const SIG_SETMASK: c_int = 3;

pub type __uint32_t = u32;
pub type __darwin_time_t = c_long;
pub type __darwin_sigset_t = __uint32_t;

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    pub __arg: *mut c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 56],
}

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 40],
}

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 8],
}

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 56],
}

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 8],
}

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [c_char; 8176],
}

#[cfg(not(windows))]
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
#[cfg(not(windows))]
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
#[cfg(not(windows))]
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
#[cfg(not(windows))]
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
#[cfg(not(windows))]
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
#[cfg(not(windows))]
pub type __darwin_pthread_t = *mut _opaque_pthread_t;

#[cfg(not(windows))]
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type sigset_t = __darwin_sigset_t;
pub type time_t = __darwin_time_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: c_long,
}

pub type clockid_t = c_uint;
pub const _CLOCK_THREAD_CPUTIME_ID: clockid_t = 16;
pub const _CLOCK_PROCESS_CPUTIME_ID: clockid_t = 12;
pub const _CLOCK_UPTIME_RAW_APPROX: clockid_t = 9;
pub const _CLOCK_UPTIME_RAW: clockid_t = 8;
pub const _CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;
pub const _CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const _CLOCK_MONOTONIC: clockid_t = 6;
pub const _CLOCK_REALTIME: clockid_t = 0;

#[cfg(not(windows))]
pub type pthread_cond_t = __darwin_pthread_cond_t;
#[cfg(not(windows))]
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
#[cfg(not(windows))]
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
#[cfg(not(windows))]
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
#[cfg(not(windows))]
pub type pthread_t = __darwin_pthread_t;
#[cfg(windows)]
pub type pthread_attr_t = HANDLE;
#[cfg(windows)]
pub type pthread_cond_t = CONDITION_VARIABLE;
#[cfg(windows)]
pub type pthread_condattr_t = HANDLE;
#[cfg(windows)]
pub type pthread_mutex_t = CRITICAL_SECTION;
#[cfg(windows)]
pub type pthread_mutexattr_t = HANDLE;
#[cfg(windows)]
pub type pthread_t = HANDLE;

#[cfg(not(windows))]
pub type mythread = pthread_t;
#[cfg(windows)]
pub type mythread = HANDLE;
#[cfg(not(windows))]
pub type mythread_mutex = pthread_mutex_t;
#[cfg(windows)]
pub type mythread_mutex = CRITICAL_SECTION;

#[cfg(not(windows))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mythread_cond {
    pub cond: pthread_cond_t,
    pub clk_id: clockid_t,
}

#[cfg(windows)]
pub type mythread_cond = CONDITION_VARIABLE;
#[cfg(not(windows))]
pub type mythread_condtime = timespec;

#[cfg(windows)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mythread_condtime {
    pub start: u32,
    pub timeout: u32,
}

#[cfg(not(windows))]
unsafe extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> c_int;
    pub fn pthread_cond_destroy(_: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_init(_: *mut pthread_cond_t, _: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_signal(_: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_timedwait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
        _: *const timespec,
    ) -> c_int;
    pub fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        _: *mut c_void,
    ) -> c_int;
    pub fn pthread_join(_: pthread_t, _: *mut *mut c_void) -> c_int;
    pub fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_init(_: *mut pthread_mutex_t, _: *const pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_sigmask(_: c_int, _: *const sigset_t, _: *mut sigset_t) -> c_int;
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_sigmask(how: c_int, set: *const sigset_t, oset: *mut sigset_t) {
    let _ret: c_int =
        unsafe { pthread_sigmask(how, set as *const sigset_t, oset as *mut sigset_t) };
}

#[cfg(windows)]
#[inline]
pub fn mythread_sigmask(_how: c_int, _set: *const sigset_t, _oset: *mut sigset_t) {}

#[cfg(windows)]
struct mythread_start_info {
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
}

#[cfg(windows)]
unsafe extern "system" fn mythread_start(param: *mut c_void) -> u32 {
    let info = Box::from_raw(param.cast::<mythread_start_info>());
    let _ = (info.func)(info.arg);
    0
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_create(
    thread: *mut mythread,
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> c_int {
    let mut old: sigset_t = 0;
    let mut all: sigset_t = 0;
    all = !(0 as sigset_t);
    mythread_sigmask(
        SIG_SETMASK,
        ::core::ptr::addr_of_mut!(all),
        ::core::ptr::addr_of_mut!(old),
    );
    let ret: c_int = unsafe {
        pthread_create(
            thread as *mut pthread_t,
            core::ptr::null(),
            Some(func),
            arg as *mut c_void,
        )
    };
    mythread_sigmask(
        SIG_SETMASK,
        ::core::ptr::addr_of_mut!(old),
        core::ptr::null_mut(),
    );
    ret
}

#[cfg(windows)]
#[inline]
pub fn mythread_create(
    thread: *mut mythread,
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> c_int {
    let info = Box::into_raw(Box::new(mythread_start_info { func, arg }));
    let ret = unsafe {
        _beginthreadex(
            core::ptr::null_mut(),
            0,
            Some(mythread_start),
            info.cast::<c_void>(),
            0,
            core::ptr::null_mut(),
        )
    };
    if ret == 0 {
        unsafe {
            let _ = Box::from_raw(info);
        }
        -1
    } else {
        unsafe {
            *thread = ret as HANDLE;
        }
        0
    }
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_join(thread: mythread) -> c_int {
    unsafe { pthread_join(thread as pthread_t, core::ptr::null_mut()) }
}

#[cfg(windows)]
#[inline]
pub fn mythread_join(thread: mythread) -> c_int {
    let mut ret = 0;
    unsafe {
        if WaitForSingleObject(thread, INFINITE) != WAIT_OBJECT_0 {
            ret = -1;
        }
        if CloseHandle(thread) == 0 {
            ret = -1;
        }
    }
    ret
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_mutex_init(mutex: *mut mythread_mutex) -> c_int {
    unsafe { pthread_mutex_init(mutex as *mut pthread_mutex_t, core::ptr::null()) }
}

#[cfg(windows)]
#[inline]
pub fn mythread_mutex_init(mutex: *mut mythread_mutex) -> c_int {
    unsafe {
        InitializeCriticalSection(mutex);
    }
    0
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_mutex_destroy(mutex: *mut mythread_mutex) {
    let _ret: c_int = unsafe { pthread_mutex_destroy(mutex as *mut pthread_mutex_t) };
}

#[cfg(windows)]
#[inline]
pub fn mythread_mutex_destroy(mutex: *mut mythread_mutex) {
    unsafe {
        DeleteCriticalSection(mutex);
    }
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_mutex_lock(mutex: *mut mythread_mutex) {
    let _ret: c_int = unsafe { pthread_mutex_lock(mutex as *mut pthread_mutex_t) };
}

#[cfg(windows)]
#[inline]
pub fn mythread_mutex_lock(mutex: *mut mythread_mutex) {
    unsafe {
        EnterCriticalSection(mutex);
    }
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_mutex_unlock(mutex: *mut mythread_mutex) {
    let _ret: c_int = unsafe { pthread_mutex_unlock(mutex as *mut pthread_mutex_t) };
}

#[cfg(windows)]
#[inline]
pub fn mythread_mutex_unlock(mutex: *mut mythread_mutex) {
    unsafe {
        LeaveCriticalSection(mutex);
    }
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_cond_init(mycond: *mut mythread_cond) -> c_int {
    unsafe {
        (*mycond).clk_id = _CLOCK_REALTIME;
        pthread_cond_init(::core::ptr::addr_of_mut!((*mycond).cond), core::ptr::null())
    }
}

#[cfg(windows)]
#[inline]
pub fn mythread_cond_init(cond: *mut mythread_cond) -> c_int {
    unsafe {
        InitializeConditionVariable(cond);
    }
    0
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_cond_destroy(cond: *mut mythread_cond) {
    let _ret: c_int = unsafe { pthread_cond_destroy(::core::ptr::addr_of_mut!((*cond).cond)) };
}

#[cfg(windows)]
#[inline]
pub fn mythread_cond_destroy(_cond: *mut mythread_cond) {}

#[cfg(not(windows))]
#[inline]
pub fn mythread_cond_signal(cond: *mut mythread_cond) {
    let _ret: c_int = unsafe { pthread_cond_signal(::core::ptr::addr_of_mut!((*cond).cond)) };
}

#[cfg(windows)]
#[inline]
pub fn mythread_cond_signal(cond: *mut mythread_cond) {
    unsafe {
        WakeConditionVariable(cond);
    }
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_cond_wait(cond: *mut mythread_cond, mutex: *mut mythread_mutex) {
    let _ret: c_int = unsafe {
        pthread_cond_wait(
            ::core::ptr::addr_of_mut!((*cond).cond),
            mutex as *mut pthread_mutex_t,
        )
    };
}

#[cfg(windows)]
#[inline]
pub fn mythread_cond_wait(cond: *mut mythread_cond, mutex: *mut mythread_mutex) {
    unsafe {
        let _ = SleepConditionVariableCS(cond, mutex, INFINITE);
    }
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_cond_timedwait(
    cond: *mut mythread_cond,
    mutex: *mut mythread_mutex,
    condtime: *const mythread_condtime,
) -> c_int {
    let ret: c_int = unsafe {
        pthread_cond_timedwait(
            ::core::ptr::addr_of_mut!((*cond).cond),
            mutex as *mut pthread_mutex_t,
            condtime as *const timespec,
        )
    };
    ret
}

#[cfg(windows)]
#[inline]
pub fn mythread_cond_timedwait(
    cond: *mut mythread_cond,
    mutex: *mut mythread_mutex,
    condtime: *const mythread_condtime,
) -> c_int {
    let (start, timeout_ms) = unsafe { ((*condtime).start, (*condtime).timeout) };
    let elapsed = unsafe { GetTickCount().wrapping_sub(start) };
    let timeout = if elapsed >= timeout_ms {
        0
    } else {
        timeout_ms - elapsed
    };
    let ret = unsafe { SleepConditionVariableCS(cond, mutex, timeout) };
    i32::from(ret == 0)
}

#[cfg(not(windows))]
#[inline]
pub fn mythread_condtime_set(
    condtime: *mut mythread_condtime,
    cond: *const mythread_cond,
    timeout_ms: u32,
) {
    unsafe {
        (*condtime).tv_sec = timeout_ms.wrapping_div(1000) as time_t as __darwin_time_t;
        (*condtime).tv_nsec = timeout_ms.wrapping_rem(1000).wrapping_mul(1_000_000) as c_long;
        let mut now: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let _ret: c_int = clock_gettime((*cond).clk_id, ::core::ptr::addr_of_mut!(now));
        (*condtime).tv_sec += now.tv_sec;
        (*condtime).tv_nsec += now.tv_nsec;
        if (*condtime).tv_nsec >= 1_000_000_000 {
            (*condtime).tv_nsec -= 1_000_000_000;
            (*condtime).tv_sec += 1;
        }
    }
}

#[cfg(windows)]
#[inline]
pub fn mythread_condtime_set(
    condtime: *mut mythread_condtime,
    _cond: *const mythread_cond,
    timeout_ms: u32,
) {
    unsafe {
        (*condtime).start = GetTickCount();
        (*condtime).timeout = timeout_ms;
    }
}
