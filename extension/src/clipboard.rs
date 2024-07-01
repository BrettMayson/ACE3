use arma_rs::Group;

#[cfg(feature = "clipboard")]
use clipboard::{ClipboardContext, ClipboardProvider};

#[cfg(feature = "clipboard")]
pub fn group() -> Group {
    Group::new()
        .command("clear", clear)
        .command("append", append)
        .command("complete", complete)
        .command("loadout", loadout)
}

#[cfg(not(feature = "clipboard"))]
pub fn group() -> Group {
    Group::new()
}

static mut BUFFER: String = String::new();

#[cfg(feature = "clipboard")]
pub fn clear() {
    // Safety: this is all single threaded, so no need to lock
    unsafe {
        BUFFER.clear();
    }
}

#[cfg(feature = "clipboard")]
pub fn append(text: String) {
    // Safety: this is all single threaded, so no need to lock
    unsafe {
        BUFFER.push_str(&text);
    }
}

#[cfg(feature = "clipboard")]
pub fn complete() -> Result<(), String> {
    // Safety: this is all single threaded, so no need to lock
    let mut ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    ctx.set_contents(unsafe { BUFFER.clone() })
        .map_err(|e| e.to_string())?;
    unsafe {
        BUFFER = String::new();
    }
    Ok(())
}

#[cfg(feature = "clipboard")]
pub fn loadout() -> Option<String> {
    let mut ctx = ClipboardContext::new().ok()?;
    let content = ctx.get_contents().ok()?;
    if !content.starts_with("[[") || !content.ends_with("]]") {
        return None;
    }
    if !content.contains("],[") || !content.contains("\",\"") {
        return None;
    }
    Some(content)
}
