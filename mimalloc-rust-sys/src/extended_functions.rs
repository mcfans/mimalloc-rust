use cty::{c_char, c_int, c_ulonglong, c_void};

use crate::types::mi_heap_t;

// Doc: https://microsoft.github.io/mimalloc/group__malloc.html
pub const MI_SMALL_SIZE_MAX: usize = 128 * core::mem::size_of::<*mut c_void>();
pub type mi_deferred_free_fun =
    Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;
pub type mi_error_fun = Option<unsafe extern "C" fn(code: c_int, arg: *mut c_void)>;

// Arena types
pub type mi_arena_id_t = c_int;
extern "C" {
    pub fn mi_collect(force: bool);
    pub fn mi_good_size(size: usize) -> usize;
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;
    pub fn mi_malloc_small(size: usize) -> *mut c_void;
    pub fn mi_process_info(
        elapsed_msecs: *mut usize,
        user_msecs: *mut usize,
        system_msecs: *mut usize,
        current_rss: *mut usize,
        peak_rss: *mut usize,
        current_commit: *mut usize,
        peak_commit: *mut usize,
        page_faults: *mut usize,
    );
    pub fn mi_register_deferred_free(out: mi_deferred_free_fun, arg: *mut c_void);
    pub fn mi_register_error(out: mi_error_fun, arg: *mut c_void);
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_reserve_huge_os_pages_at(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
    ) -> c_int;
    pub fn mi_reserve_huge_os_pages_interleave(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
    ) -> c_int;
    pub fn mi_stats_print(_: *mut c_void);
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_stats_reset();
    pub fn mi_stats_merge();
    pub fn mi_thread_init();
    pub fn mi_thread_done();
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_usable_size(p: *const c_void) -> usize;
    pub fn mi_zalloc_small(size: usize) -> *mut c_void;

    // Arena functions
    pub fn mi_reserve_os_memory(size: usize, commit: bool, allow_large: bool) -> c_int;
    pub fn mi_manage_os_memory(
        start: *mut c_void,
        size: usize,
        is_committed: bool,
        is_large: bool,
        is_zero: bool,
        numa_node: c_int,
    ) -> bool;
    pub fn mi_debug_show_arenas();

    // Experimental: heaps associated with specific memory arenas
    pub fn mi_arena_area(arena_id: mi_arena_id_t, size: *mut usize) -> *mut c_void;
    pub fn mi_reserve_huge_os_pages_at_ex(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> c_int;
    pub fn mi_reserve_os_memory_ex(
        size: usize,
        commit: bool,
        allow_large: bool,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> c_int;
    pub fn mi_manage_os_memory_ex(
        start: *mut c_void,
        size: usize,
        is_committed: bool,
        is_large: bool,
        is_zero: bool,
        numa_node: c_int,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> bool;
    pub fn mi_heap_new_in_arena(arena_id: mi_arena_id_t) -> *mut mi_heap_t;
}
