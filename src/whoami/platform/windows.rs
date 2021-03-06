/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate winapi;
extern crate advapi32;

#[path = "../../common/wide.rs"] #[macro_use] mod wide;

use std::mem;
use std::io::Write;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use self::wide::FromWide;

pub unsafe fn getusername() -> String {
    let mut buffer: [winapi::WCHAR; winapi::UNLEN as usize + 1] = mem::uninitialized();
    let mut len = buffer.len() as winapi::DWORD;
    if advapi32::GetUserNameW(buffer.as_mut_ptr(), &mut len) == 0 {
        crash!(1, "failed to get username");
    }
    String::from_wide(&buffer[..len as usize - 1])
}
