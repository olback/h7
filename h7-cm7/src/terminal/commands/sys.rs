use crate::{display::GPU, time::TimeSource};

use {
    super::{utils::*, HEADER_WIDTH, LABEL_WIDTH},
    crate::{
        consts,
        led::Led,
        logger,
        terminal::{
            menu::{MenuError, MenuItem},
            TerminalWriter, MENU,
        },
        utils::interrupt_free,
    },
    chrono::{Datelike, NaiveDate, Timelike},
    core::{fmt::Write, str::FromStr},
    stm32h7xx_hal as hal,
};

pub const HELP: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "help",
    help: "help <program> - Show help about a program",
    description: "Show help about a program",
    action: |m, args| {
        check_args_len(1, args.len())?;
        let program = args[0];
        let mut command_found = false;
        iter_menu(m, args, MENU, &mut |menu, _, item, _, _| {
            match item {
                MenuItem::Command { name, help, .. } => {
                    if *name == program {
                        writeln!(menu.writer(), "{help}")?;
                        command_found = true;
                        return Ok(false);
                    }
                }
                MenuItem::Alias { alias, command } => {
                    if *alias == program {
                        writeln!(menu.writer(), "'{alias}' aliased to '{command}'")?;
                        menu.run("help", &[*command])?;
                        command_found = true;
                        return Ok(false);
                    }
                }
                MenuItem::Group { .. } => {
                    // nop
                }
            };
            Ok(true)
        })?;

        if command_found {
            Ok(())
        } else {
            Err(MenuError::CommandNotFound)
        }
    },
};

pub const MAN: MenuItem<'static, TerminalWriter> = MenuItem::Alias { alias: "man", command: "help" };

pub const PROGRAMS: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "programs",
    help: "programs - Show available builtin programs",
    description: "Show available builtin programs",
    action: |m, args| {
        check_args_len(0, args.len())?;

        iter_menu(m, args, MENU, &mut |menu, _, item, _, level| {
            match item {
                MenuItem::Command { name, description, .. } => {
                    writeln!(
                        menu.writer(),
                        "{padding}{name:LABEL_WIDTH$} {description}",
                        padding = PaddedStr::<b' '>("", level),
                    )?;
                }
                MenuItem::Alias { alias, command } => {
                    writeln!(
                        menu.writer(),
                        "{padding}{alias:LABEL_WIDTH$} aliased to {command}",
                        padding = PaddedStr::<b' '>("", level),
                    )?;
                }
                MenuItem::Group { title, .. } => {
                    writeln!(
                        menu.writer(),
                        "{p}{t:-^-w$}",
                        p = PaddedStr::<b' '>("", level),
                        t = PaddedStr::<b' '>(title, 1),
                        w = HEADER_WIDTH - level - level
                    )?;
                }
            }
            Ok(true)
        })
    },
};

pub const COMMANDS: MenuItem<'static, TerminalWriter> = MenuItem::Alias {
    alias: "commands",
    command: "programs",
};

pub const SYS: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "sys",
    help: "sys <function> - Test system functionality",
    description: "Test system functionality",
    action: |m, args| match args {
        ["panic"] => {
            writeln!(m.writer(), "Panicing!")?;
            panic!("User caused a panic")
        }
        ["bkpt"] => {
            writeln!(m.writer(), "Breakpoint!")?;
            cortex_m::asm::bkpt();
            Ok(())
        }
        ["udf"] => {
            writeln!(m.writer(), "Undefined instruction!")?;
            cortex_m::asm::udf();
        }
        ["reset"] => {
            writeln!(m.writer(), "Resetting!")?;
            cortex_m::peripheral::SCB::sys_reset()
        }
        ["loglevel"] => {
            writeln!(m.writer(), "Current log level: {}", logger::get_log_level())?;
            Ok(())
        }
        ["loglevel", level] => {
            match log::LevelFilter::from_str(level) {
                Ok(new_level) => {
                    logger::set_log_level(new_level);
                    writeln!(m.writer(), "New log level: {new_level}")?;
                }
                Err(e) => {
                    writeln!(m.writer(), "Failed to set new log level: {e}")?;
                }
            }
            Ok(())
        }
        _ => check_args_len(1, args.len()),
    },
};

pub const INFO: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "info",
    help: "info [target] - Query information from the system",
    description: "Query information from the system",
    action: |m, args| match args {
        ["mcu"] => {
            writeln!(m.writer(), "{:LABEL_WIDTH$} STM32H747", "MCU")?;
            // Uid is 12 byts, hex string will be 24.
            let (id, id_len) = to_hex::<24>(hal::signature::Uid::read(), false);
            // SAFETY: to_hex always returns valid hex
            let id_str = unsafe { core::str::from_utf8_unchecked(&id[0..id_len]) };
            writeln!(m.writer(), "{:LABEL_WIDTH$} {}", "Unique ID", id_str)?;
            Ok(())
        }
        ["cpu"] => {
            writeln!(m.writer(), "{:LABEL_WIDTH$} Cortex-M7F", "Core")?;
            match interrupt_free(crate::system::cpu_freq) {
                Some(freq) => writeln!(m.writer(), "{:LABEL_WIDTH$} {}MHz", "Core frequency", freq.to_MHz())?,
                None => writeln!(m.writer(), "{:LABEL_WIDTH$} unavailable", "Core frequency")?,
            }
            match interrupt_free(crate::system::cpu_temp) {
                Some(temp) => writeln!(m.writer(), "{:LABEL_WIDTH$} {:.01}°C", "Core temperature", temp)?,
                None => writeln!(m.writer(), "{:LABEL_WIDTH$} unavailable", "Core temperature")?,
            }
            writeln!(m.writer(), "{:LABEL_WIDTH$} {}", "Cycle count", cortex_m::peripheral::DWT::cycle_count())?;
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {}",
                "Instruction cache",
                bool_to_enabled_disabled_str(cortex_m::peripheral::SCB::icache_enabled())
            )?;
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {}",
                "Data cache",
                bool_to_enabled_disabled_str(cortex_m::peripheral::SCB::dcache_enabled())
            )?;
            Ok(())
        }
        ["ram"] => {
            writeln!(m.writer(), "{:LABEL_WIDTH$} {}KiB", "Internal RAM", crate::system::ram_size() / 1024)?;
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {}MiB",
                "External SDRAM",
                crate::mem::sdram::SDRAM_SIZE / (1024 * 1024)
            )?;
            Ok(())
        }
        ["flash"] => {
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {}MiB",
                "Internal FLASH",
                crate::system::flash_size() / (1024 * 1024)
            )?;
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {}MiB",
                "External FLASH",
                crate::fs::qspi_store::QSPI_FLASH_SIZE / (1024 * 1024)
            )?;
            Ok(())
        }
        ["os"] => {
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {used} / {total} bytes ({fraction:.02}%)",
                "Memory usage",
                used = crate::mem::ALLOCATOR.used(),
                total = crate::mem::HEAP_SIZE,
                fraction = (crate::mem::ALLOCATOR.used() as f64 / crate::mem::HEAP_SIZE as f64) * 100.0
            )?;
            writeln!(m.writer(), "{:LABEL_WIDTH$} {} bytes", "GPU reserved", crate::display::FRAME_BUFFER_ALLOC_SIZE)?;
            writeln!(m.writer(), "{:LABEL_WIDTH$} {}", "Rust version", consts::RUSTC_VERSION)?;
            writeln!(m.writer(), "{:LABEL_WIDTH$} {}", "Version", consts::GIT_DESCRIBE)?;
            writeln!(m.writer(), "{:LABEL_WIDTH$} {}", "Debug", cfg!(debug_assertions))?;
            let dt = NaiveDate::from_ymd_opt(consts::COMPILE_TIME_YEAR, consts::COMPILE_TIME_MONTH, consts::COMPILE_TIME_DAY)
                .and_then(|dt| dt.and_hms_opt(consts::COMPILE_TIME_HOUR, consts::COMPILE_TIME_MINUTE, consts::COMPILE_TIME_SECOND))
                .unwrap();
            writeln!(
                m.writer(),
                "{:LABEL_WIDTH$} {weekday} {month} {day} {hh:02}:{mm:02}:{ss:02} {year}",
                "Compiled",
                weekday = dt.weekday(),
                month = month_to_str(dt.month()),
                day = dt.day(),
                hh = dt.hour(),
                mm = dt.minute(),
                ss = dt.second(),
                year = dt.year()
            )?;

            match interrupt_free(|cs| *crate::time::BOOT_TIME.borrow(cs).borrow()) {
                Some(dt) => {
                    writeln!(
                        m.writer(),
                        "{:LABEL_WIDTH$} {weekday} {month} {day} {hh:02}:{mm:02}:{ss:02} {year}",
                        "Boot time",
                        weekday = dt.weekday(),
                        month = month_to_str(dt.month()),
                        day = dt.day(),
                        hh = dt.hour(),
                        mm = dt.minute(),
                        ss = dt.second(),
                        year = dt.year()
                    )?;
                }
                None => {
                    writeln!(m.writer(), "{:LABEL_WIDTH$} unavailable", "Boot time")?;
                }
            }

            Ok(())
        }
        [] => {
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " MCU ")?;
            m.run("info", &["mcu"])?;
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " CPU ")?;
            m.run("info", &["cpu"])?;
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " RAM ")?;
            m.run("info", &["ram"])?;
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " FLASH ")?;
            m.run("info", &["flash"])?;
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " SD Card ")?;
            m.run("sdcard", &["info"])?;
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " OS ")?;
            m.run("info", &["os"])?;
            writeln!(m.writer(), "{:-^-HEADER_WIDTH$}", " Date/Time ")?;
            m.run("date", args)?;
            Ok(())
        }
        _ => {
            writeln!(m.writer(), "Unknown query")?;
            Ok(())
        }
    },
};

pub const WIFICTL: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "wifictl",
    help: "wifictl - Control WIFI networks and connections",
    description: "(TODO) Control WIFI networks and connections",
    action: |m, _| {
        writeln!(m.writer(), "todo")?;
        Ok(())
    },
};

pub const BTCTL: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "btctl",
    help: "btctl - Control Bluetooth connections",
    description: "(TODO) Control Bluetooth connections",
    action: |m, _| {
        writeln!(m.writer(), "todo")?;
        Ok(())
    },
};

pub const ETHCTL: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "ethctl",
    help: "ethctl - Control ethernet connections",
    description: "(TODO) Control ethernet connections",
    action: |m, _| {
        writeln!(m.writer(), "todo")?;
        Ok(())
    },
};

pub const UPTIME: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "uptime",
    help: "uptime - Query the system uptime",
    description: "Query the system uptime",
    action: |m, args| {
        check_args_len(0, args.len())?;
        match (TimeSource::get_date_time(), interrupt_free(|cs| *crate::time::BOOT_TIME.borrow(cs).borrow())) {
            (Some(now), Some(boot_time)) => {
                let dur = now - boot_time;
                write!(m.writer(), "Uptime: ")?;
                crate::utils::write_pretty_duration(m.writer(), dur)?;
                writeln!(m.writer())?;
            }
            (Some(_), _) => {
                writeln!(m.writer(), "Uptime: unavailable <boot time unavailable>")?;
            }
            (_, Some(_)) => {
                writeln!(m.writer(), "Uptime: unavailable <current time unavailable>")?;
            }
            _ => {
                writeln!(m.writer(), "Uptime: unavailable <boot time and current time unavailable>")?;
            }
        }
        Ok(())
    },
};

pub const LEDCTL: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "ledctl",
    help: "ledctl <(r|red)|(g|green)|(b|blue)> <0|1> - Control RGB LED",
    description: "Control RGB LED",
    action: |m, args| {
        check_args_len(2, args.len())?;
        match args {
            ["r" | "red", "0"] => unsafe { Led::Red.off() },
            ["r" | "red", "1"] => unsafe { Led::Red.on() },
            ["g" | "green", "0"] => unsafe { Led::Green.off() },
            ["g" | "green", "1"] => unsafe { Led::Green.off() },
            ["b" | "blue", "0"] => unsafe { Led::Blue.off() },
            ["b" | "blue", "1"] => unsafe { Led::Blue.on() },
            _ => {
                writeln!(m.writer(), "Invalid color or state")?;
            }
        }
        Ok(())
    },
};

pub const CORECTL: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "corectl",
    help: "corectl - Start/stop the Cortex-M4 core",
    description: "(TODO) Start/stop the Cortex-M4 core",
    action: |m, args| {
        check_args_len(1, args.len())?;
        match args {
            ["start"] => writeln!(m.writer(), "todo")?,
            ["stop"] => writeln!(m.writer(), "todo")?,
            ["status"] => writeln!(m.writer(), "todo")?,
            _ => writeln!(m.writer(), "Unknown command")?,
        }

        Ok(())
    },
};

pub const DISPCTL: MenuItem<'static, TerminalWriter> = MenuItem::Command {
    name: "dispctl",
    help: "dispctl - Control the display",
    description: "Control the display",
    action: |m, args| {
        check_args_len(1, args.len())?;
        match args {
            ["swap"] => {
                interrupt_free(|cs| {
                    GPU.borrow(cs).borrow_mut().as_mut().unwrap().swap();
                });
            }
            _ => writeln!(m.writer(), "Unknown command")?,
        }

        Ok(())
    },
};
