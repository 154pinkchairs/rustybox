use crate::archival::libarchive::bb_archive::archive_handle_t;
pub unsafe fn data_skip(mut archive_handle: *mut archive_handle_t) {
  (*archive_handle).seek.expect("non-null function pointer")(
    (*archive_handle).src_fd,
    (*(*archive_handle).file_header).size,
  );
}
