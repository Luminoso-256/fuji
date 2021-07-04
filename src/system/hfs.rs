use structview::{View};

//Master Directory Block. See files-102
//names have been redone to be more descriptive than Apple's originals
//note that i16 is used as the classic Macs were 16-bit, and integers took up two bytes
//i32 is used in place of a longint
//to see why no ExtDataRecs are used, consult https://developer.apple.com/library/archive/documentation/mac/Files/Files-106.html#MARKER-2-608
#[derive(Clone, Copy, View)]
#[repr(C)]
struct Mdb {
    file_magic:i16,
    creation_date:i32,
    last_modified:i32,
    volume_attributes:i16,
    //the number of files in the *root* directory of the volume
    num_files:i16,
    //couldn't think of a better name for this one. It is the first block of the volume bitmap. Should *Always* be 3
    vbmst:i16,
    //this is where the next allocation search will begin. TODO: find out what an allocation search is
    alloc_pointer:i16,
    //the total number of blocks on the disk.
    num_blocks:u16,
    //the size in bytes of each blocl. must be a multiple of 512
    block_size:i32,
    // no extra info given
    clump_size:i32,
    //location of the first block of the volume
    first_block:i16,
    //the next unused catalog ID. (Dir/file ID)
    next_unused_catalog_id:i32,
    //number of unused allocation blocks
    free_blocks:u16,
    //the name of our disk
    volume_name:[char; 27],
    //backup related things
    last_backup:i32,
    backup_sequence_number:i16,
    //how many times has this been written to?
    write_count:i32,
    //the clump size for the "extents overflow file". original name used for brevity.
    XTClpSize:i32,
    //the clump size for the catalog file. original name used for brevity
    CTClpSize:i32,
    //the number of directories on the *root* of the volume
    num_root_dirs:i16,
    //the total number of files on the volume
    file_count:i32,
    //the total number of directories on the volume
    dir_count:i32,
    //internal info for the mac os finder
    //https://developer.apple.com/library/archive/documentation/mac/Toolbox/Toolbox-2.html this should contain info on the topic
    finder_info:[i32;8],
    //size in blocks of volume cache
    vc_size:i16,
    //size in blocks of volume bitmap cache
    vbmc_size:i16,
    //size in blocks of common volume cache
    ctlc_size:i16,
    //size of extents overflow file. (unit not proviede?)
    xtfl_size:i32,
    //extent record for overflow file,
    overflow_extent_record:[ExtDescriptor;3],
    //size of catalog file (unit not provided?)
    ctfl_size:i32,
    //extent record for catalog file
    catalog_extent_record: [ExtDescriptor;3]
}

//extent descriptor. starting block and number of blocks, since extents are contiguous
#[derive(Clone, Copy, View)]
#[repr(C)]
struct ExtDescriptor {
    first_block:u16,
    num_blocks:u16
}

fn read_disk(filename:&str){
    //TODO:implemnt
    unimplemented!();
}