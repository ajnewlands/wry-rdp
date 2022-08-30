mod rdp;
use std::net::ToSocketAddrs;

use anyhow::{anyhow, Result};
use messages::*;
use rdp::*;

use log::*;

use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, OnceCell};

pub static SCREEN_TX: OnceCell<mpsc::UnboundedSender<Vec<u8>>> = OnceCell::const_new();

#[repr(C)]
struct RDPContext {
    common: rdp::rdpClientContext,
}

const TRUE: i32 = 1;
const FALSE: i32 = 0;

const MAXIMUM_RDP_WAIT_OBJECTS: usize = 64;

extern "C" fn rdp_global_init() -> i32 {
    // Use a provided signal handler to print out stack traces
    unsafe {
        if rdp::freerdp_handle_signals() != 0 {
            return FALSE;
        }
    }

    TRUE
}

extern "C" fn rdp_global_uninit() {}

extern "C" fn rdp_pre_connect(instance: *mut rdp::freerdp) -> i32 {
    unsafe {
        // Lint was refusing to dereference these automatically.
        (*(*(*instance).context).settings).OsMajorType = rdp::OSMAJORTYPE_WINDOWS;
        (*(*(*instance).context).settings).OsMinorType = rdp::OSMINORTYPE_WINDOWS_NT;
        (*(*(*instance).context).settings).ConnectionType = CONNECTION_TYPE_AUTODETECT;
        (*(*(*instance).context).settings).IgnoreCertificate = 1;
    }

    TRUE
}

extern "C" fn rdp_begin_paint(_context: *mut rdp_context) -> i32 {
    TRUE
}
extern "C" fn rdp_end_paint(context: *mut rdp_context) -> i32 {
    unsafe {
        let screen_tx = SCREEN_TX
            .get()
            .expect("rdp::SCREEN_TX is not initialized in rdp_end_paint()");
        let invalid = (*(*(*(*(*context).gdi).primary).hdc).hwnd).ninvalid;

        /*
        for i in 0..invalid {
            let region = (*(*(*(*(*context).gdi).primary).hdc).hwnd)
                .cinvalid
                .offset(i as isize);
            trace!(
                "Invalid region {} x: {}, y: {}, h: {}, w: {}",
                i,
                (*region).x,
                (*region).y,
                (*region).h,
                (*region).w
            );
        }
        */

        // Dump the content of the framebuffer down the websocket
        if invalid > 0 {
            let screen_width = (*(*context).gdi).width as u32;
            let screen_height = (*(*context).gdi).height as u32;

            let data = std::slice::from_raw_parts(
                (*(*context).gdi).primary_buffer,
                (screen_width * screen_height * 4) as usize,
            );
            screen_tx.send(data.to_vec()).unwrap();
            std::mem::forget(data);
        }

        (*(*(*(*(*(*context).gdi).primary).hdc).hwnd).invalid).null = TRUE;
        (*(*(*(*(*context).gdi).primary).hdc).hwnd).ninvalid = 0;
    }

    TRUE
}

extern "C" fn rdp_desktop_resize(_context: *mut rdp_context) -> i32 {
    info!("TODO Resize desktop");

    TRUE
}

fn get_rdp_pixel_format(bpp: u32, tp: u32, a: u32, r: u32, g: u32, b: u32) -> u32 {
    (bpp << 24) + (tp << 16) | (a << 12) | (r << 8) | (g << 4) | b
}

extern "C" fn rdp_post_connect(instance: *mut rdp::freerdp) -> i32 {
    let pixel_format_rgbx32 =
        get_rdp_pixel_format(32, rdp::FREERDP_PIXEL_FORMAT_TYPE_RGBA, 0, 8, 8, 8);

    unsafe {
        if rdp::gdi_init(instance, pixel_format_rgbx32) == FALSE {
            rdp_lasterror("RDP GDI initialization failed", instance);
            return FALSE;
        }
        (*(*(*instance).context).update).BeginPaint = Some(rdp_begin_paint);
        (*(*(*instance).context).update).EndPaint = Some(rdp_end_paint);
        (*(*(*instance).context).update).DesktopResize = Some(rdp_desktop_resize);
    }

    TRUE
}

extern "C" fn rdp_post_disconnect(_instance: *mut rdp::freerdp) {
    info!("TODO rdp_post_disconnect()");
}

extern "C" fn rdp_logon_error_info(_instance: *mut rdp::freerdp, data: u32, ty: u32) -> i32 {
    unsafe {
        let pdata = rdp::freerdp_get_logon_error_info_data(data);
        let ptype = rdp::freerdp_get_logon_error_info_type(ty);

        let str_data = std::ffi::CStr::from_ptr(pdata);
        let str_type = std::ffi::CStr::from_ptr(ptype);
        error!(
            "Logon failed: {}: {}",
            str_data.to_string_lossy(),
            str_type.to_string_lossy()
        );
    }

    TRUE
}

extern "C" fn rdp_client_new(instance: *mut rdp::freerdp, context: *mut rdp_context) -> i32 {
    let ctx = context as *mut RDPContext;

    if instance.is_null() {
        error!("instance is null in rdp_client_new()");
        return FALSE;
    }
    if ctx.is_null() {
        error!("ctx is null in rdp_client_new()");
        return FALSE;
    }

    unsafe {
        (*instance).PreConnect = Some(rdp_pre_connect);
        (*instance).PostConnect = Some(rdp_post_connect);
        (*instance).PostDisconnect = Some(rdp_post_disconnect);
        //(*instance).Authenticate = Some(rdp_client_authenticate);
        (*instance).LogonErrorInfo = Some(rdp_logon_error_info);
    }

    TRUE
}

extern "C" fn rdp_client_free(_instance: *mut rdp::freerdp, _context: *mut rdp_context) {
    info!("TODO rdp_client_free()");
}

extern "C" fn rdp_client_start(_context: *mut rdp_context) -> i32 {
    info!("TODO rdp_client_start()");

    0
}

extern "C" fn rdp_client_stop(_context: *mut rdp_context) -> i32 {
    info!("TODO rdp_client_stop()");

    0
}

fn rdp_client_entry() -> rdp::RDP_CLIENT_ENTRY_POINTS {
    let mut ep: rdp::RDP_CLIENT_ENTRY_POINTS = unsafe { std::mem::zeroed() };

    ep.Version = rdp::RDP_CLIENT_INTERFACE_VERSION;
    ep.Size = std::mem::size_of::<rdp::RDP_CLIENT_ENTRY_POINTS_V1>() as u32;
    ep.GlobalInit = Some(rdp_global_init);
    ep.GlobalUninit = Some(rdp_global_uninit);
    ep.ContextSize = std::mem::size_of::<RDPContext>() as u32;
    ep.ClientNew = Some(rdp_client_new);
    ep.ClientFree = Some(rdp_client_free);
    ep.ClientStart = Some(rdp_client_start);
    ep.ClientStop = Some(rdp_client_stop);

    ep
}

fn rdp_lasterror(tag: &str, instance: *mut rdp::freerdp) -> u32 {
    unsafe {
        let rc = rdp::freerdp_get_last_error((*instance).context);
        let perror = rdp::freerdp_get_last_error_string(rc);
        let str_error = std::ffi::CStr::from_ptr(perror);
        rdp::freerdp_abort_connect(instance);
        error!(
            "{}, code: {}, reason {}",
            tag,
            rc,
            str_error.to_string_lossy()
        );

        rc
    }
}

fn handle_mouse_input(input: *mut rdp_input, mouse_event: MouseEvent) {
    unsafe {
        let mut flags = 0;
        flags |= match mouse_event.action.as_str() {
            "down" => rdp::PTR_FLAGS_DOWN,
            "up" => 0,
            _ => rdp::PTR_FLAGS_MOVE,
        };

        flags |= match mouse_event.button.as_str() {
            "left" => rdp::PTR_FLAGS_BUTTON1,
            "middle" => rdp::PTR_FLAGS_BUTTON3,
            "right" => rdp::PTR_FLAGS_BUTTON2,
            _ => 0,
        };

        let _rc = freerdp_input_send_mouse_event(
            input,
            flags as u16,
            mouse_event.x.try_into().unwrap(),
            mouse_event.y.try_into().unwrap(),
        );
    }
}

fn handle_key_input(input: *mut rdp_input, key_event: KeyboardEvent) {
    let flags = match key_event.action.as_str() {
        "up" => rdp::KBD_FLAGS_RELEASE,
        _ => 0,
    } as u16;

    let code;
    if key_event.key.len() == 1 {
        code = key_event.key.encode_utf16().next().unwrap();
        unsafe {
            rdp::freerdp_input_send_unicode_keyboard_event(input, flags, code);
        }
    } else {
        let code = match key_event.key.as_str() {
            "Backspace" => 0x0e,
            other => {
                log::warn!("Disregarding unhandled special key {}", other);
                return;
            }
        };
        unsafe {
            rdp::freerdp_input_send_keyboard_event(input, flags, code);
        }
    };
}

fn handle_input(input: *mut rdp_input, event: FromBrowserMessages) {
    match event {
        FromBrowserMessages::MouseEvent(me) => {
            handle_mouse_input(input, me);
        }
        FromBrowserMessages::KeyboardEvent(ke) => {
            handle_key_input(input, ke);
        }
        unhandled => log::warn!("Discarding unhandled input event: {:?}", unhandled),
    }
}

fn rdp_client_thread_proc(
    instance: *mut rdp::freerdp,
    mut command_rx: mpsc::UnboundedReceiver<FromBrowserMessages>,
) -> bool {
    info!("starting rdp_client_thread_proc()");

    unsafe {
        // TODO consider case of multiple addresses returned - try each in sequence until one connects.
        if rdp::freerdp_connect(instance) == FALSE {
            let rc = rdp::freerdp_get_last_error((*instance).context);
            let perror = rdp::freerdp_get_last_error_string(rc);
            let str_error = std::ffi::CStr::from_ptr(perror);
            rdp::freerdp_abort_connect(instance);
            error!(
                "RDP connection failed, code: {}, reason {}",
                rc,
                str_error.to_string_lossy()
            );

            return false;
        }

        let mut events: Vec<*mut std::ffi::c_void> =
            Vec::<*mut std::ffi::c_void>::with_capacity(MAXIMUM_RDP_WAIT_OBJECTS);
        loop {
            if rdp::freerdp_shall_disconnect(instance) != 0 {
                info!("RDP shall disconnect");
                break;
            }

            let count = rdp::freerdp_get_event_handles(
                (*instance).context,
                events.as_mut_ptr(),
                MAXIMUM_RDP_WAIT_OBJECTS as u32,
            );
            trace!("Event count is {}", count);

            // Apparently most people cannot hit a key twice consecutively with a gap much smaller than 100ms
            let status = rdp::WaitForMultipleObjects(count, events.as_mut_ptr(), FALSE, 25);
            trace!("Wait status: {}", status);
            loop {
                match command_rx.try_recv() {
                    Ok(ev) => handle_input((*(*instance).context).input, ev),
                    Err(TryRecvError::Empty) => break,
                    Err(e) => {
                        log::error!("Failed receiving more input events: {:?}", e);
                        break;
                    }
                }
            }

            if rdp::freerdp_check_event_handles((*instance).context) == FALSE {
                rdp_lasterror("Failed checking RDP event handles", instance);
                break;
            }
        }
    }

    true
}

pub struct RDP {
    context: *mut rdp::rdp_context,
}
unsafe impl Send for RDP {}
unsafe impl Sync for RDP {}

impl RDP {
    pub fn new(cfg: RDPConfiguration) -> Result<Self> {
        info!(
            "Attempting to establish RDP client for {}:{}",
            cfg.host, cfg.port
        );
        if let Err(_) = format!("{}:0", cfg.host).to_socket_addrs() {
            return Err(anyhow!("Unable to resolve host."));
        }

        let mut cep = rdp_client_entry();
        unsafe {
            let root = rdp::WLog_GetRoot();
            rdp::WLog_SetLogLevel(root, 1);
            let context = rdp::freerdp_client_context_new(&mut cep);
            let server_hostname = std::ffi::CString::new(cfg.host).unwrap();
            (*(*context).settings).ServerHostname = server_hostname.into_raw();
            (*(*context).settings).ServerPort = cfg.port as u32;
            let username = std::ffi::CString::new(cfg.username).unwrap();
            (*(*context).settings).Username = username.into_raw();
            let password = std::ffi::CString::new(cfg.password).unwrap();
            (*(*context).settings).Password = password.into_raw();

            if context.is_null() {
                info!("Context is null");
                std::process::exit(1);
            }

            if rdp::freerdp_client_start(context) != 0 {
                error!("freerdp_client_start() failed");
                std::process::exit(1);
            }

            return Ok(RDP { context });
        }
    }

    pub fn start(&self, command_rx: mpsc::UnboundedReceiver<FromBrowserMessages>) -> Result<()> {
        unsafe {
            if !rdp_client_thread_proc((*self.context).instance, command_rx) {
                error!("rdp_client_thread_proc() returned false..");
                return Err(anyhow!("Failed to start RDP process"));
            }
        }
        Ok(())
    }
}
