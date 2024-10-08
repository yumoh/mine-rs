/* automatically generated by rust-bindgen 0.69.4 */

pub const LIBMINE_VERSION: &[u8; 6] = b"1.2.6\0";
pub const EST_MIC_APPROX: u32 = 0;
pub const EST_MIC_E: u32 = 1;
pub const FALSE: u32 = 0;
pub const TRUE: u32 = 1;
extern "C" {
    pub static mut libmine_version: *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mine_problem {
    pub n: ::std::os::raw::c_int,
    pub x: *mut f64,
    pub y: *mut f64,
}
#[test]
fn bindgen_test_layout_mine_problem() {
    const UNINIT: ::std::mem::MaybeUninit<mine_problem> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mine_problem>(),
        24usize,
        concat!("Size of: ", stringify!(mine_problem))
    );
    assert_eq!(
        ::std::mem::align_of::<mine_problem>(),
        8usize,
        concat!("Alignment of ", stringify!(mine_problem))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_problem),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_problem),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_problem),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mine_parameter {
    pub alpha: f64,
    pub c: f64,
    pub est: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mine_parameter() {
    const UNINIT: ::std::mem::MaybeUninit<mine_parameter> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mine_parameter>(),
        24usize,
        concat!("Size of: ", stringify!(mine_parameter))
    );
    assert_eq!(
        ::std::mem::align_of::<mine_parameter>(),
        8usize,
        concat!("Alignment of ", stringify!(mine_parameter))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alpha) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_parameter),
            "::",
            stringify!(alpha)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_parameter),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).est) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_parameter),
            "::",
            stringify!(est)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mine_score {
    pub n: ::std::os::raw::c_int,
    pub m: *mut ::std::os::raw::c_int,
    pub M: *mut *mut f64,
}
#[test]
fn bindgen_test_layout_mine_score() {
    const UNINIT: ::std::mem::MaybeUninit<mine_score> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mine_score>(),
        24usize,
        concat!("Size of: ", stringify!(mine_score))
    );
    assert_eq!(
        ::std::mem::align_of::<mine_score>(),
        8usize,
        concat!("Alignment of ", stringify!(mine_score))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_score),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_score),
            "::",
            stringify!(m)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).M) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_score),
            "::",
            stringify!(M)
        )
    );
}
extern "C" {
    pub fn mine_compute_score(
        prob: *mut mine_problem,
        param: *mut mine_parameter,
    ) -> *mut mine_score;
}
extern "C" {
    pub fn mine_check_parameter(param: *mut mine_parameter) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mine_mic(score: *mut mine_score) -> f64;
}
extern "C" {
    pub fn mine_mas(score: *mut mine_score) -> f64;
}
extern "C" {
    pub fn mine_mev(score: *mut mine_score) -> f64;
}
extern "C" {
    pub fn mine_mcn(score: *mut mine_score, eps: f64) -> f64;
}
extern "C" {
    pub fn mine_mcn_general(score: *mut mine_score) -> f64;
}
extern "C" {
    pub fn mine_tic(score: *mut mine_score, norm: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn mine_gmic(score: *mut mine_score, p: f64) -> f64;
}
extern "C" {
    pub fn mine_free_score(score: *mut *mut mine_score);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mine_matrix {
    pub data: *mut f64,
    pub n: ::std::os::raw::c_int,
    pub m: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mine_matrix() {
    const UNINIT: ::std::mem::MaybeUninit<mine_matrix> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mine_matrix>(),
        16usize,
        concat!("Size of: ", stringify!(mine_matrix))
    );
    assert_eq!(
        ::std::mem::align_of::<mine_matrix>(),
        8usize,
        concat!("Alignment of ", stringify!(mine_matrix))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_matrix),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_matrix),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_matrix),
            "::",
            stringify!(m)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mine_pstats {
    pub mic: *mut f64,
    pub tic: *mut f64,
    pub n: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mine_pstats() {
    const UNINIT: ::std::mem::MaybeUninit<mine_pstats> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mine_pstats>(),
        24usize,
        concat!("Size of: ", stringify!(mine_pstats))
    );
    assert_eq!(
        ::std::mem::align_of::<mine_pstats>(),
        8usize,
        concat!("Alignment of ", stringify!(mine_pstats))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_pstats),
            "::",
            stringify!(mic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tic) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_pstats),
            "::",
            stringify!(tic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_pstats),
            "::",
            stringify!(n)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mine_cstats {
    pub mic: *mut f64,
    pub tic: *mut f64,
    pub n: ::std::os::raw::c_int,
    pub m: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mine_cstats() {
    const UNINIT: ::std::mem::MaybeUninit<mine_cstats> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mine_cstats>(),
        24usize,
        concat!("Size of: ", stringify!(mine_cstats))
    );
    assert_eq!(
        ::std::mem::align_of::<mine_cstats>(),
        8usize,
        concat!("Alignment of ", stringify!(mine_cstats))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_cstats),
            "::",
            stringify!(mic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tic) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_cstats),
            "::",
            stringify!(tic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_cstats),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(mine_cstats),
            "::",
            stringify!(m)
        )
    );
}
extern "C" {
    pub fn mine_compute_pstats(X: *mut mine_matrix, param: *mut mine_parameter)
        -> *mut mine_pstats;
}
extern "C" {
    pub fn mine_compute_cstats(
        X: *mut mine_matrix,
        Y: *mut mine_matrix,
        param: *mut mine_parameter,
    ) -> *mut mine_cstats;
}
