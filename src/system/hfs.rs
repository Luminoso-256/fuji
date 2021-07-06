use std::io::Cursor;
use byteorder::{ReadBytesExt, BigEndian};
use std::convert::TryFrom;

//Master Directory Block. See files-102
//names have been redone to be more descriptive than Apple's originals
//note that i16 is used as the classic Macs were 16-bit, and integers took up two bytes
//i32 is used in place of a longint
//to see why no ExtDataRecs are used, consult https://developer.apple.com/library/archive/documentation/mac/Files/Files-106.html#MARKER-2-608
#[derive(Debug)]
pub struct Mdb {
    file_magic:i16,
    creation_date:i32,
    last_modified:i32,
    volume_attributes:i16,
    //the number of files in the *root* directory of the volume
    num_root_files:i16,
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
    overflow_extent_record:[ExtDescriptor;2],
    //size of catalog file (unit not provided?)
    ctfl_size:i32,
    //extent record for catalog file
    catalog_extent_record: [ExtDescriptor;2]
}

impl Default for Mdb{
    fn default() -> Self {
        Mdb{
            file_magic: 0,
            creation_date: 0,
            last_modified: 0,
            volume_attributes: 0,
            num_root_files: 0,
            vbmst: 0,
            alloc_pointer: 0,
            num_blocks: 0,
            block_size: 0,
            clump_size: 0,
            first_block: 0,
            next_unused_catalog_id: 0,
            free_blocks: 0,
            volume_name: ["_".parse().unwrap();27],
            last_backup: 0,
            backup_sequence_number: 0,
            write_count: 0,
            XTClpSize: 0,
            CTClpSize: 0,
            num_root_dirs: 0,
            file_count: 0,
            dir_count: 0,
            finder_info: [0,0,0,0,0,0,0,0],
            vc_size: 0,
            vbmc_size: 0,
            ctlc_size: 0,
            xtfl_size: 0,
            overflow_extent_record: [ExtDescriptor{..Default::default()},ExtDescriptor{..Default::default()}],
            ctfl_size: 0,
            catalog_extent_record: [ExtDescriptor{..Default::default()},ExtDescriptor{..Default::default()}]
        }
    }
}

//extent descriptor. starting block and number of blocks, since extents are contiguous
#[derive(Debug)]
pub struct ExtDescriptor {
    first_block:u16,
    num_blocks:u16
}

impl Default for ExtDescriptor{
    fn default() -> Self {
        ExtDescriptor{
            first_block: 0,
            num_blocks: 0
        }
    }
}

pub fn read_disk(filename:&str){
   let disk_file = std::fs::read(filename).unwrap();
    let mut mdb = Mdb{..Default::default()};
    let mut offset = 0;
    //first, get our offset & header.
    for i in 0..disk_file.len(){

        if disk_file[i] == 66 as u8{
            if disk_file[i+1] == 68 as u8{
                //we found the magic! and the offset is i+2 (since we don't want to bother with any of the magic).
                mdb.file_magic = 6668;
                offset = i+2;
                println!("Found filemagic for HFS at byte {}",&i);
                break;
            }
        }
    }
    let mut cursor = Cursor::new(&disk_file);
    cursor.set_position(offset as u64);
    mdb.creation_date = cursor.read_i32::<BigEndian>().unwrap();
    mdb.last_modified = cursor.read_i32::<BigEndian>().unwrap();
    mdb.volume_attributes = cursor.read_i16::<BigEndian>().unwrap();
    mdb.num_root_files = cursor.read_i16::<BigEndian>().unwrap();
    mdb.vbmst = cursor.read_i16::<BigEndian>().unwrap();
    mdb.alloc_pointer = cursor.read_i16::<BigEndian>().unwrap();
    mdb.num_blocks = cursor.read_u16::<BigEndian>().unwrap();
    mdb.block_size = cursor.read_i32::<BigEndian>().unwrap();
    mdb.clump_size = cursor.read_i32::<BigEndian>().unwrap();
    mdb.first_block = cursor.read_i16::<BigEndian>().unwrap();
    mdb.next_unused_catalog_id = cursor.read_i32::<BigEndian>().unwrap();
    mdb.free_blocks = cursor.read_u16::<BigEndian>().unwrap();
    for i in 0..27{
        mdb.volume_name[i] = char::try_from(cursor.read_u8().unwrap() as u32).unwrap();
    }
    mdb.last_backup =cursor.read_i32::<BigEndian>().unwrap();
    mdb.backup_sequence_number = cursor.read_i16::<BigEndian>().unwrap();
    mdb.write_count = cursor.read_i32::<BigEndian>().unwrap();
    mdb.XTClpSize = cursor.read_i32::<BigEndian>().unwrap();
    mdb.CTClpSize = cursor.read_i32::<BigEndian>().unwrap();
    mdb.num_root_dirs = cursor.read_i16::<BigEndian>().unwrap();
    mdb.file_count = cursor.read_i32::<BigEndian>().unwrap();
    for i in 0..8{
        mdb.finder_info[i] = cursor.read_i32::<BigEndian>().unwrap();
    }
    mdb.vc_size = cursor.read_i16::<BigEndian>().unwrap();
    mdb.vbmc_size = cursor.read_i16::<BigEndian>().unwrap();
    mdb.ctlc_size = cursor.read_i16::<BigEndian>().unwrap();
    mdb.xtfl_size = cursor.read_i32::<BigEndian>().unwrap();
    for i in 0..2{
        mdb.overflow_extent_record[i]  = ExtDescriptor{
            first_block: cursor.read_u16::<BigEndian>().unwrap(),
            num_blocks: cursor.read_u16::<BigEndian>().unwrap()
        }
    }
    mdb.ctfl_size = cursor.read_i32::<BigEndian>().unwrap();
    for i in 0..2{
        mdb.catalog_extent_record[i]  = ExtDescriptor{
            first_block: cursor.read_u16::<BigEndian>().unwrap(),
            num_blocks: cursor.read_u16::<BigEndian>().unwrap()
        }
    }
    println!("mdb: {:?}",&mdb);
}

//=== B*-Trees (https://developer.apple.com/library/archive/documentation/mac/Files/Files-104.html)
//i8 has been used here because the official Pascal representation makes mention of a SignedByte
//a byte is the size of an 8 bit number, and signed only really makes sense in the context of a number
struct NodeDescriptor{
    //link to next node of type
    forward_link:i32,
    //link to previous node of type
    backward_link:i32,
    //the type of the node. Options are:
    // 0x00 | Index Node
    // 0x01 | Header Node
    // 0x02 | Map Node
    // 0xFF | Leaf Node
    node_type:i8,
    //depth of node in hierarchy of b*-tree
    node_level:i8,
    num_records:i16,
    //what, exactly, is this for? It'll always be 0...
    reserved:i16
}