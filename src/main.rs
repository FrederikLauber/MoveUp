/* MoveUp, a tool to move all files in selected folders up and delete the folder>
/Copyright (C) 2025  Dr. Frederik Lauber

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>."""
*/

#![windows_subsystem = "windows"]

use std::{env, path};
use std::fs;
use std::path::{Path};

use windows::core::{Error, PCWSTR};
use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS};
use windows::Win32::Storage::FileSystem::MoveFileExW;
use windows::Win32::Storage::FileSystem::MOVE_FILE_FLAGS;

use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};

fn main() {
    unsafe {
        FreeConsole();
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    println!(r###"MoveUp {} Copyright (C) 2025 Dr. Frederik Lauber
    This program comes with ABSOLUTELY NO WARRANTY;
    This is free software licensed under the GPL3, and you are welcome to redistribute it
    under certain conditions."###, env!("GIT_TAG"));

    let args: Vec<String> = env::args().collect();

    if args.len()!=2{
        eprintln!("Wrong arguments, only allowed argument is the path to a folder you want to move up one folder");
        std::process::exit(2);
    }

    let folder_str = args.get(1).unwrap();
    let folder_non_canonical = Path::new(folder_str);
    let folder = match folder_non_canonical.canonicalize(){
        Ok(folder) => folder,
        Err(_) => {
            eprintln!("Arguments need to be folders");
            std::process::exit(3);
        }
    };

    let target = match folder.parent(){
        Some(target) => target,
        None => {
            eprintln!("No parent folder found");
            std::process::exit(4);
        }
    };

    if ! target.is_dir() || ! folder.is_dir(){
        eprintln!("Arguments need to be folders");
        std::process::exit(5);
    }

    let read_dir = match fs::read_dir(&folder){
        Ok(t) => t,
        Err(_) => {
            eprintln!("Could not read content of {}, skipping", folder.to_string_lossy());
            std::process::exit(6);
        }
    };

    'directory_loop: for entry in read_dir {
        match entry {
            Ok(dir_entry) => {
                let dir_path = dir_entry.path();
                let file_stem = dir_path.file_stem().unwrap_or_default();
                let extension = dir_path.extension().unwrap_or_default();

                let Ok(old_path) = fs::canonicalize(&dir_entry.path()) else { continue };

                'inner: for suffix in 0..1000000{
                    let suffix_str = if suffix == 0 {""} else { &*format!("_{}", suffix) };

                    let extension = if extension.is_empty() {""} else { &*format!(".{}", extension.to_string_lossy()) };

                    let tmp = target.join(format!("{}{}{}", file_stem.to_string_lossy(), suffix_str, extension));
                    let Ok(new_path) = path::absolute(tmp) else { continue };

                    let source_wide: Vec<u16> = old_path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
                    let destination_wide: Vec<u16> = new_path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();

                    let result = unsafe {
                        MoveFileExW(
                            PCWSTR(source_wide.as_ptr()),
                            PCWSTR(destination_wide.as_ptr()),
                            MOVE_FILE_FLAGS(0), // No flags so we get an error on existing files
                        )
                    };

                    if let Err(_e) = result {
                        if Error::from_win32() == Error::from(ERROR_ALREADY_EXISTS) {
                            eprintln!("Exists: {:?}", new_path);
                            continue 'inner
                        } else {
                            eprintln!("Error moving entry: {:?}", dir_entry.path());
                            continue 'directory_loop
                        }
                    } else {
                        continue 'directory_loop
                    }
                }
                eprintln!("Error moving entry after 1000 rename tries: {:?}", dir_entry.path());
                continue 'directory_loop
            },
            Err(e) => {
                eprintln!("Error reading an entry: {}", e);
                continue
            }
        }
    }

    if let Err(e) = fs::remove_dir(&folder) {
        eprintln!("Could not remove {} due to  {}", folder.to_string_lossy(), e);
    }
}
