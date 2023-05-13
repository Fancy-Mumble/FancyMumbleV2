mod tests {
    use crate::utils::file::get_file_as_byte_vec;
    use std::{
        fs::File,
        io::{IoSlice, Write},
    };
    use tempdir::TempDir;

    #[test]
    fn test_file_read() {
        let tmp_dir = TempDir::new("test_dir").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let os_path = file_path.clone();

        let mut tmp_file = File::create(file_path).unwrap();
        let slice = &[0x00, 0x01, 0x02, 0x03];
        tmp_file.write_vectored(&[IoSlice::new(slice)]).unwrap();

        let result = aw!(get_file_as_byte_vec(os_path.as_os_str().to_str().unwrap()));
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.len() == 4);
        assert!(result == slice);

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }
}
