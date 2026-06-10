pub mod access;
pub mod alloc;
pub mod print;
pub mod set_replace;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PageReplacement {
    FIFO,
    LRU,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub page_id: usize,
    pub process_id: usize,
    pub loaded_at: usize,
    pub last_accessed: usize,
    pub is_mapped: bool,
}

#[derive(Debug, Clone)]
pub struct MemoryManager {
    pub frame_count: usize,
    pub page_size: usize,
    pub frames: Vec<Option<Page>>,
    pub algorithm: PageReplacement,
    pub page_faults: usize,
    pub access_counter: usize,
    pub process_pages: HashMap<usize, Vec<usize>>,
    next_page_id: usize,
}

impl MemoryManager {
    pub fn new(frame_count: usize, page_size: usize) -> Self {
        MemoryManager {
            frame_count,
            page_size,
            frames: vec![None; frame_count],
            algorithm: PageReplacement::FIFO,
            page_faults: 0,
            access_counter: 0,
            process_pages: HashMap::new(),
            next_page_id: 1,
        }
    }

    pub fn allocate_pages(&mut self, process_id: usize, count: usize) -> Vec<usize> {
        let mut allocated = Vec::new();

        for _ in 0..count {
            let page_id = self.next_page_id;
            self.next_page_id += 1;
            self.process_pages.entry(process_id).or_default().push(page_id);

            if self.free_frame() {
                self.load_page(page_id, process_id);
                println!(
                    "Page #{} allocated to Process #{} and loaded into memory",
                    page_id, process_id
                );
            } else {
                println!(
                    "Page #{} allocated to Process #{}, but memory full — will fault on access",
                    page_id, process_id
                );
            }
            allocated.push(page_id);
        }

        let total_pages = self.process_pages.get(&process_id).map_or(0, |v| v.len());
        println!(
            "Process #{} now has {} pages (total memory: {} / {} frames used)",
            process_id,
            total_pages,
            self.used_frames(),
            self.frame_count
        );
        allocated
    }

    pub fn access_page(&mut self, process_id: usize, page_number: usize) {
        self.access_counter += 1;

        let pages = match self.process_pages.get(&process_id) {
            Some(p) => p,
            None => {
                println!("Error: Process #{} has no allocated pages", process_id);
                return;
            }
        };

        if page_number >= pages.len() {
            println!("Error: Process #{} does not have page #{}", process_id, page_number);
            return;
        }

        let page_id = pages[page_number];

        if self.is_page_in_memory(page_id) {
            self.update_access(page_id);
            println!(
                "Process #{} accessed page #{} (virtual page {}) — HIT",
                process_id, page_id, page_number
            );
        } else {
            self.page_faults += 1;
            println!(
                "Process #{} accessed page #{} (virtual page {}) — PAGE FAULT (#{})",
                process_id, page_id, page_number, self.page_faults
            );
            self.handle_page_fault(page_id, process_id);
        }
    }

    fn is_page_in_memory(&self, page_id: usize) -> bool {
        self.frames.iter().any(|f| f.as_ref().is_some_and(|p| p.page_id == page_id && p.is_mapped))
    }

    fn update_access(&mut self, page_id: usize) {
        for frame in &mut self.frames {
            if let Some(page) = frame {
                if page.page_id == page_id {
                    page.last_accessed = self.access_counter;
                    return;
                }
            }
        }
    }

    fn free_frame(&self) -> bool {
        for frame in &self.frames {
            if frame.is_none() || !frame.as_ref().unwrap().is_mapped {
                return true;
            }
        }
        false
    }

    fn handle_page_fault(&mut self, page_id: usize, process_id: usize) {
        if self.free_frame() {
            self.load_page(page_id, process_id);
        } else {
            let victim_index = self.select_victim();
            let victim = self.frames[victim_index].take().unwrap();
            println!(
                "Replacing page #{} (Process #{}) with page #{} (Process #{}) using {:?}",
                victim.page_id, victim.process_id, page_id, process_id, self.algorithm
            );
            self.load_page_at(victim_index, page_id, process_id);
        }
    }

    fn select_victim(&self) -> usize {
        match self.algorithm {
            PageReplacement::FIFO => {
                let mut oldest_idx = 0;
                let mut oldest_time = usize::MAX;
                for (i, frame) in self.frames.iter().enumerate() {
                    if let Some(page) = frame {
                        if page.is_mapped && page.loaded_at < oldest_time {
                            oldest_time = page.loaded_at;
                            oldest_idx = i;
                        }
                    }
                }
                oldest_idx
            }
            PageReplacement::LRU => {
                let mut lru_idx = 0;
                let mut lru_time = usize::MAX;
                for (i, frame) in self.frames.iter().enumerate() {
                    if let Some(page) = frame {
                        if page.is_mapped && page.last_accessed < lru_time {
                            lru_time = page.last_accessed;
                            lru_idx = i;
                        }
                    }
                }
                lru_idx
            }
        }
    }

    fn load_page(&mut self, page_id: usize, process_id: usize) {
        for i in 0..self.frame_count {
            if self.frames[i].is_none() || !self.frames[i].as_ref().unwrap().is_mapped {
                self.load_page_at(i, page_id, process_id);
                return;
            }
        }
    }

    fn load_page_at(&mut self, index: usize, page_id: usize, process_id: usize) {
        self.access_counter += 1;
        self.frames[index] = Some(Page {
            page_id,
            process_id,
            loaded_at: self.access_counter,
            last_accessed: self.access_counter,
            is_mapped: true,
        });
    }

    fn used_frames(&self) -> usize {
        self.frames.iter().filter(|f| f.as_ref().is_some_and(|p| p.is_mapped)).count()
    }

    pub fn print_state(&self) {
        println!("\n=== Memory Manager State ===");
        println!("Algorithm: {:?}", self.algorithm);
        println!("Frame Count: {}", self.frame_count);
        println!("Page Size: {} bytes", self.page_size);
        println!("Used Frames: {} / {}", self.used_frames(), self.frame_count);
        println!("Total Page Faults: {}", self.page_faults);

        println!("\nFrame Table:");
        for (i, frame) in self.frames.iter().enumerate() {
            match frame {
                Some(page) if page.is_mapped => {
                    println!(
                        "  Frame {}: Page #{} (Process #{}, loaded: {}, last access: {})",
                        i, page.page_id, page.process_id, page.loaded_at, page.last_accessed
                    );
                }
                _ => {
                    println!("  Frame {}: FREE", i);
                }
            }
        }

        println!("\nProcess Page Tables:");
        for (pid, pages) in &self.process_pages {
            println!("  Process #{}:", pid);
            for (i, page_id) in pages.iter().enumerate() {
                let in_mem = if self.is_page_in_memory(*page_id) { "in memory" } else { "swapped out" };
                println!("    Virtual page {} -> Page #{} ({})", i, page_id, in_mem);
            }
        }
    }

    pub fn set_algorithm(&mut self, algo: PageReplacement) {
        let name = format!("{:?}", algo);
        self.algorithm = algo;
        println!("Page replacement algorithm set to {}", name);
    }
}
