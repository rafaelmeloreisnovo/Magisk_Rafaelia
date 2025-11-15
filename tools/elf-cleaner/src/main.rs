use object::build::elf::{Builder, Dynamic, SectionData};
use object::elf;
use std::{env, fs};

// Implementation adapted from https://github.com/termux/termux-elf-cleaner
//
// This tool removes unsupported dynamic section entries from ELF binaries to ensure
// compatibility with older Android versions. Some dynamic entries like DT_RPATH,
// DT_RUNPATH, and certain AArch64-specific entries can cause runtime linker issues
// on older Android releases.

// Missing ELF constants for AArch64 architecture
// These constants are not defined in the standard object crate but are needed
// for AArch64-specific binary manipulation
const DT_AARCH64_BTI_PLT: u32 = elf::DT_LOPROC + 1;  // Branch Target Identification
const DT_AARCH64_PAC_PLT: u32 = elf::DT_LOPROC + 3;  // Pointer Authentication Code
const DT_AARCH64_VARIANT_PCS: u32 = elf::DT_LOPROC + 5;  // Variant Procedure Call Standard

// Only keep DT_FLAGS_1 flags that are widely supported across Android versions
// DF_1_NOW: Immediate symbol resolution (safe)
// DF_1_GLOBAL: Available to dlopen calls (safe)
const SUPPORTED_DT_FLAGS: u32 = elf::DF_1_NOW | elf::DF_1_GLOBAL;

/// Print a message when removing a dynamic section entry
/// 
/// # Arguments
/// * `name` - The name of the dynamic section entry being removed
/// * `path` - The path to the ELF file being processed
fn print_remove_dynamic(name: &str, path: &str) {
    println!("Removing dynamic section entry {} in '{}'", name, path);
}

/// Process an ELF binary file to remove unsupported dynamic section entries
/// 
/// This function:
/// 1. Reads the ELF file into memory
/// 2. Parses the ELF structure
/// 3. Removes unsupported dynamic section entries (DT_RPATH, DT_RUNPATH, etc.)
/// 4. Masks unsupported DT_FLAGS_1 flags
/// 5. Writes the cleaned ELF back to disk
/// 
/// # Arguments
/// * `path` - Path to the ELF file to process
/// 
/// # Returns
/// * `Ok(())` if processing succeeded
/// * `Err` if there was an I/O error or the file is not a valid ELF
fn process_elf(path: &str) -> anyhow::Result<()> {
    // Read the ELF file from disk
    let bytes = fs::read(path)?;
    let mut elf = Builder::read(bytes.as_slice())?;
    
    // Check if this is an AArch64 (ARM64) binary
    // We need to know this to remove AArch64-specific dynamic entries
    let is_aarch64 = elf.header.e_machine == elf::EM_AARCH64;

    // Iterate through all sections in the ELF file
    elf.sections.iter_mut().for_each(|section| {
        if let SectionData::Dynamic(entries) = &mut section.data {
            // Remove unsupported dynamic section entries
            // DT_RPATH and DT_RUNPATH can cause issues on older Android versions
            entries.retain(|e| {
                let tag = e.tag();
                match tag {
                    // DT_RPATH: Runtime library search path (deprecated, use DT_RUNPATH)
                    elf::DT_RPATH => {
                        print_remove_dynamic("DT_RPATH", path);
                        return false;
                    }
                    // DT_RUNPATH: Runtime library search path (not supported on old Android)
                    elf::DT_RUNPATH => {
                        print_remove_dynamic("DT_RUNPATH", path);
                        return false;
                    }
                    _ => {}
                }
                // For AArch64 binaries, also remove architecture-specific entries
                if is_aarch64 {
                    match tag {
                        // Branch Target Identification for PLT entries
                        DT_AARCH64_BTI_PLT => {
                            print_remove_dynamic("DT_AARCH64_BTI_PLT", path);
                            return false;
                        }
                        // Pointer Authentication Code for PLT entries
                        DT_AARCH64_PAC_PLT => {
                            print_remove_dynamic("DT_AARCH64_PAC_PLT", path);
                            return false;
                        }
                        // Variant Procedure Call Standard marker
                        DT_AARCH64_VARIANT_PCS => {
                            print_remove_dynamic("DT_AARCH64_VARIANT_PCS", path);
                            return false;
                        }
                        _ => {}
                    }
                }
                true
            });
            
            // Remove unsupported flags from DT_FLAGS_1
            // Some flags cause issues on older Android versions, so we mask them out
            for entry in entries.iter_mut() {
                if let Dynamic::Integer { tag, val } = entry {
                    if *tag == elf::DT_FLAGS_1 {
                        // Keep only the flags that are widely supported
                        let new_flags = *val & SUPPORTED_DT_FLAGS as u64;
                        if new_flags != *val {
                            println!(
                                "Replacing unsupported DT_FLAGS_1 {:#x} with {:#x} in '{}'",
                                *val, new_flags, path
                            );
                            *val = new_flags;
                        }
                        break;
                    }
                }
            }
        }
    });

    // Write the modified ELF back to disk
    let mut out_bytes = Vec::new();
    elf.write(&mut out_bytes)?;
    fs::write(path, &out_bytes)?;
    Ok(())
}

/// Main entry point
/// 
/// Processes all ELF files passed as command-line arguments.
/// Each argument should be a path to an ELF binary file.
/// 
/// # Example
/// ```
/// elf-cleaner /path/to/libfoo.so /path/to/libbar.so
/// ```
fn main() -> anyhow::Result<()> {
    // Skip the program name (first argument) and process each file
    env::args().skip(1).try_for_each(|s| process_elf(&s))
}
