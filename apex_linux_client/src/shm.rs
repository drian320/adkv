use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use crate::types::{SharedData, SHARED_MEMORY_NAME, SHARED_MEMORY_SIZE};

pub struct SharedMemoryClient {
    mmap: Arc<Mutex<MmapMut>>,
}

impl SharedMemoryClient {
    /// 共有メモリに接続 (POSIX shm_open経由)
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        // POSIXの共有メモリは /dev/shm/ 配下にあるファイルとしてアクセス可能
        let shm_path = "/dev/shm/apex_dma_shared";

        // 共有メモリファイルをオープン
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&shm_path)?;

        // メモリマップ
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(Self {
            mmap: Arc::new(Mutex::new(mmap)),
        })
    }

    /// 共有メモリからデータを読み取り
    pub fn read_data(&self) -> Result<SharedData, Box<dyn std::error::Error>> {
        let mmap = self.mmap.lock().unwrap();

        // 生のバイト列としてアクセス
        let data_bytes = &mmap[..std::mem::size_of::<SharedData>()];

        // C構造体として直接キャスト（repr(C)で保証）
        let data = unsafe {
            std::ptr::read_unaligned(data_bytes.as_ptr() as *const SharedData)
        };

        Ok(data)
    }

    /// 設定を共有メモリに書き込み (部分更新)
    pub fn write_settings(&self, settings: &crate::types::Settings) -> Result<(), Box<dyn std::error::Error>> {
        let mut mmap = self.mmap.lock().unwrap();

        // 既存データを読み取り
        let data_bytes = &mmap[..std::mem::size_of::<SharedData>()];
        let mut data = unsafe {
            std::ptr::read_unaligned(data_bytes.as_ptr() as *const SharedData)
        };

        // 設定部分のみ更新
        data.settings = *settings;

        // 書き戻し
        let data_ptr = &data as *const SharedData as *const u8;
        let data_bytes = unsafe {
            std::slice::from_raw_parts(data_ptr, std::mem::size_of::<SharedData>())
        };

        mmap[..std::mem::size_of::<SharedData>()].copy_from_slice(data_bytes);

        Ok(())
    }
}
