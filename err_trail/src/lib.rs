#![cfg_attr(not(test), no_std)]
#![allow(unused_variables)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use core::fmt::Display;

mod sealed {
    /// A sealed trait to prevent external implementations.
    pub trait Sealed {}
}

/// For logging a [`Result`] when [`Result::Err`] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "defmt",)))
)]
pub trait ErrContext<T, E>: sealed::Sealed {
    /// If [`Result::Err`], logging the display of the [`Result::Err`] as an "error".
    fn log_error(self) -> Result<T, E>
    where
        E: Display;
    /// If [`Result::Err`], logging the display of the [`Result::Err`] as an "warn".
    fn log_warn(self) -> Result<T, E>
    where
        E: Display;
    /// If [`Result::Err`], logging the display of the [`Result::Err`] as an "info".
    fn log_info(self) -> Result<T, E>
    where
        E: Display;
    /// If [`Result::Err`], logging the display of the [`Result::Err`] as an "debug".
    fn log_debug(self) -> Result<T, E>
    where
        E: Display;
    /// If [`Result::Err`], logging the display of the [`Result::Err`] as an "trace".
    fn log_trace(self) -> Result<T, E>
    where
        E: Display;

    /// If [`Result::Err`], logging context as an "error".
    fn log_error_msg(self, msg: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as a "warn".
    fn log_warn_msg(self, msg: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as an "info".
    fn log_info_msg(self, msg: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as a "debug".
    fn log_debug_msg(self, msg: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as a "trace".
    fn log_trace_msg(self, msg: impl Display) -> Result<T, E>;

    /// If [`Result::Err`], lazily logging the result of [f] as an "error".
    fn log_error_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as a "warn".
    fn log_warn_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as an "info".
    fn log_info_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as a "debug".
    fn log_debug_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as a "trace".
    fn log_trace_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
}

/// For logging a [Option] when [None] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "defmt",)))
)]
pub trait NoneContext<T>: sealed::Sealed {
    /// If [None], logging context as an "error".
    fn log_error_msg(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "warn".
    fn log_warn_msg(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as an "info".
    fn log_info_msg(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "debug".
    fn log_debug_msg(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "trace".
    fn log_trace_msg(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn log_error_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "warn".
    fn log_warn_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "info".
    fn log_info_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "debug".
    fn log_debug_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "trace".
    fn log_trace_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

impl<T, E> sealed::Sealed for Result<T, E> {}

impl<T, E> ErrContext<T, E> for Result<T, E> {
    #[inline]
    fn log_error(self) -> Result<T, E>
    where
        E: Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", err);
                #[cfg(feature = "log")]
                log::error!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::error!("{}", defmt::Display2Format(&err));
                Err(err)
            }
        }
    }

    #[inline]
    fn log_warn(self) -> Result<T, E>
    where
        E: Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", err);
                #[cfg(feature = "log")]
                log::warn!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::warn!("{}", defmt::Display2Format(&err));
                Err(err)
            }
        }
    }

    #[inline]
    fn log_info(self) -> Result<T, E>
    where
        E: Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{}", err);
                #[cfg(feature = "log")]
                log::info!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::info!("{}", defmt::Display2Format(&err));
                Err(err)
            }
        }
    }

    #[inline]
    fn log_debug(self) -> Result<T,E>
    where
        E: Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("{}", err);
                #[cfg(feature = "log")]
                log::debug!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::debug!("{}", defmt::Display2Format(&err));
                Err(err)
            }
        }
    }

    #[inline]
    fn log_trace(self) -> Result<T, E>
    where
        E: Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("{}", err);
                #[cfg(feature = "log")]
                log::trace!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::trace!("{}", defmt::Display2Format(&err));
                Err(err)
            }
        }
    }
    #[inline]
    fn log_error_msg(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_warn_msg(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_info_msg(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_debug_msg(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_trace_msg(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_error_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_warn_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_info_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_debug_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_trace_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }
}

impl<T> sealed::Sealed for Option<T> {}

impl<T> NoneContext<T> for Option<T> {
    #[inline]
    fn log_error_msg(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_warn_msg(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_info_msg(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_debug_msg(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_trace_msg(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_error_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_warn_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_info_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_debug_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn log_trace_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }
}
