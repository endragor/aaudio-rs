extern crate libc;
extern crate aaudio_sys;

use std::ffi::c_void;
use std::fmt;
use std::mem::MaybeUninit;

use aaudio_sys as ffi;
use ffi::{AAudioStream as AAudioStreamRaw, AAudioStreamBuilder as AAudioStreamBuilderRaw};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// These values are returned from AAudio functions to indicate failure.
pub enum Error {
    /// AAudio returned error code that is not a part of this enum
    Unknown(i32),

    /// The audio device was disconnected. This could occur, for example, when headphones
    /// are plugged in or unplugged. The stream cannot be used after the device is disconnected.
    /// Applications should stop and close the stream.
    /// If this error is received in an error callback then another thread should be
    /// used to stop and close the stream.
    Disconnected,

    /// An invalid parameter was passed to AAudio.
    IllegalArgument,

    /// The requested operation is not appropriate for the current state of AAudio.
    InvalidState,

    /// The server rejected the handle used to identify the stream.
    InvalidHandle,

    /// The function is not implemented for this stream.
    Unimplemented,

    /// A resource or information is unavailable.
    /// This could occur when an application tries to open too many streams,
    /// or a timestamp is not available.
    Unavailable,

    /// Memory could not be allocated.
    NoFreeHandles,

    /// Memory could not be allocated.
    NoMemory,

    /// A NULL pointer was passed to AAudio.
    /// Or a NULL pointer was detected internally.
    Null,

    /// An operation took longer than expected.
    Timeout,

    WouldBlock,

    /// The requested data format is not supported.
    InvalidFormat,

    /// A requested was out of range.
    OutOfRange,

    /// The audio service was not available.
    NoService,

    /// The requested sample rate was not supported.
    InvalidRate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(code) => write!(f, "Error code {}", code),
            Self::Disconnected => f.write_str("The audio device was disconnected"),
            Self::IllegalArgument => f.write_str("An invalid parameter was passed to AAudio."),
            Self::InvalidState => f.write_str(
                "The requested operation is not appropriate for the current state of AAudio.",
            ),
            Self::InvalidHandle => {
                f.write_str("The server rejected the handle used to identify the stream.")
            }
            Self::Unimplemented => f.write_str("The function is not implemented for this stream."),
            Self::Unavailable => f.write_str("A resource or information is unavailable."),
            Self::NoFreeHandles => f.write_str("Memory could not be allocated."),
            Self::NoMemory => f.write_str("Memory could not be allocated"),
            Self::Null => f.write_str("A NULL pointer was passed to AAudio."),
            Self::Timeout => f.write_str("An operation took longer than expected."),
            Self::WouldBlock => {
                f.write_str("A blocking operation was invoked where no blocking was expected.")
            }
            Self::InvalidFormat => f.write_str("The requested data format is not supported."),
            Self::OutOfRange => f.write_str("A requested was out of range."),
            Self::NoService => f.write_str("The audio service was not available."),
            Self::InvalidRate => f.write_str("The requested sample rate was not supported."),
        }
    }
}

impl Error {
    fn from_code(code: i32) -> Self {
        match code {
            -899 => Self::Disconnected,
            -898 => Self::IllegalArgument,
            -895 => Self::InvalidState,
            -892 => Self::InvalidHandle,
            -890 => Self::Unimplemented,
            -889 => Self::Unavailable,
            -888 => Self::NoFreeHandles,
            -887 => Self::NoMemory,
            -886 => Self::Null,
            -885 => Self::Timeout,
            -884 => Self::WouldBlock,
            -883 => Self::InvalidFormat,
            -882 => Self::OutOfRange,
            -881 => Self::NoService,
            -880 => Self::InvalidRate,
            code => Self::Unknown(code),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    /// Audio data will travel out of the device, for example through a speaker.
    Output,

    /// Audio data will travel into the device, for example from a microphone.
    Input,
}

impl Direction {
    fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::Output,
            1 => Self::Input,
            direction => panic!("Unexpected direction: {}", direction),
        }
    }
}

/// A sample format.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Format {
    Unspecified = 0,

    /// This format uses the i16 data type.
    /// The maximum range of the data is -32768 to 32767.
    I16,

    /// This format uses the float data type.
    /// The nominal range of the data is [-1.0f32, 1.0f32).
    /// Values outside that range may be clipped.
    ///
    /// See also 'floatData' at
    /// https://developer.android.com/reference/android/media/AudioTrack#write(float[],%20int,%20int,%20int)
    F32,
}

impl Format {
    fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::Unspecified,
            1 => Self::I16,
            2 => Self::F32,
            format => panic!("Unexpected format: {}", format),
        }
    }

    fn sample_size(&self) -> i32 {
        match self {
            Self::Unspecified => 0,
            Self::I16 => 2,
            Self::F32 => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SharingMode {
    /// This will be the only stream using a particular source or sink.
    /// This mode will provide the lowest possible latency.
    /// You should close Exclusive streams immediately when you are not using them.
    Exclusive,
    /// Multiple applications will be mixed by the AAudio Server.
    /// This will have higher latency than the Exclusive mode.
    Shared,
}

impl SharingMode {
    fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::Exclusive,
            1 => Self::Shared,
            mode => panic!("Unexpected sharing mode: {}", mode),
        }
    }
}

/// The Usage attribute expresses "why" you are playing a sound, what is this sound used for.
/// This information is used by certain platforms or routing policies
/// to make more refined volume or routing decisions.
///
/// Note that these match the equivalent values in android.media.AudioAttributes
/// in the Android Java API.
///
/// Added in API level 28.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Usage {
    /// Use this for streaming media, music performance, video, podcasts, etcetera.
    Media = 1,

    /// Use this for voice over IP, telephony, etcetera.
    VoiceCommunication = 2,

    /// Use this for sounds associated with telephony such as busy tones, DTMF, etcetera.
    VoiceCommunicationSignalling = 3,

    /// Use this to demand the users attention.
    Alarm = 4,

    /// Use this for notifying the user when a message has arrived or some
    /// other background event has occured.
    Notification = 5,

    /// Use this when the phone rings.
    NotificationRingtone = 6,

    /// Use this to attract the users attention when, for example, the battery is low.
    NotificationEvent = 10,

    /// Use this for screen readers, etcetera.
    AssistanceAccessibility = 11,

    /// Use this for driving or navigation directions.
    AssistanceNavigationGuidance = 12,

    /// Use this for user interface sounds, beeps, etcetera.
    AssistanceSonification = 13,

    /// Use this for game audio and sound effects.
    Game = 14,

    /// Use this for audio responses to user queries, audio instructions or help utterances.
    Assistant = 16,

    /// Use this in case of playing sounds in an emergency.
    /// Privileged MODIFY_AUDIO_ROUTING permission required.
    Emergency = 1000,

    /// Use this for safety sounds and alerts, for example backup camera obstacle detection.
    /// Privileged MODIFY_AUDIO_ROUTING permission required.
    Safety = 1001,

    /// Use this for vehicle status alerts and information, for example the check engine light.
    /// Privileged MODIFY_AUDIO_ROUTING permission required.
    VehicleStatus = 1002,

    /// Use this for traffic announcements, etc.
    /// Privileged MODIFY_AUDIO_ROUTING permission required.
    Announcement = 1003,
}

/// Defines the audio source.
/// An audio source defines both a default physical source of audio signal, and a recording
/// configuration.
///
/// Note that these match the equivalent values in MediaRecorder.AudioSource in the Android Java API.
///
/// Added in API level 28.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputPreset {
    /// Use this preset when other presets do not apply.
    Generic = 1,

    /// Use this preset when recording video.
    Camcorder = 5,

    /// Use this preset when doing speech recognition.
    VoiceRecognition = 6,

    /// Use this preset when doing telephony or voice messaging.
    VoiceCommunication = 7,

    /// Use this preset to obtain an input with no effects.
    /// Note that this input will not have automatic gain control
    /// so the recorded volume may be very low.
    Unprocessed = 9,

    /// Use this preset for capturing audio meant to be processed in real time
    /// and played back for live performance (e.g karaoke).
    /// The capture path will minimize latency and coupling with playback path.
    /// Available since API level 29.
    VoicePerformance = 10,
}

/// The ContentType attribute describes "what" you are playing.
/// It expresses the general category of the content. This information is optional.
/// But in case it is known (for instance `Movie` for a
/// movie streaming service or `Speech` for
/// an audio book application) this information might be used by the audio framework to
/// enforce audio focus.
///
/// Note that these match the equivalent values in android.media.AudioAttributes
/// in the Android Java API.
///
/// Added in API level 28.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ContentType {
    /// Use this for spoken voice, audio books, etcetera.
    Speech = 1,

    /// Use this for pre-recorded or live music.
    Music = 2,

    /// Use this for a movie or video soundtrack.
    Movie = 3,

    /// Use this for sound is designed to accompany a user action,
    /// such as a click or beep sound made when the user presses a button.
    Sonification = 4,
}

/// Specifying if audio may or may not be captured by other apps or the system.
///
/// Note that these match the equivalent values in android.media.AudioAttributes
/// in the Android Java API.
///
/// Added in API level 29.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AllowedCapturePolicy {
    /// Indicates that the audio may be captured by any app.
    ///
    /// For privacy, the following usages can not be recorded: `VoiceCommunication*`,
    /// `Notification*`, `Assistance*` and `Assistant`.
    ///
    /// On Android Q, this means only `Media` and `Game` may be captured.
    ///
    /// See android.media.AudioAttributes#ALLOW_CAPTURE_BY_ALL.
    AllowCaptureByAll = 1,

    /// Indicates that the audio may only be captured by system apps.
    ///
    /// System apps can capture for many purposes like accessibility, user guidance...
    /// but have strong restriction. See
    /// android.media.AudioAttributes#ALLOW_CAPTURE_BY_SYSTEM for what the system apps
    /// can do with the capture audio.
    AllowCaptureBySystem = 2,

    /// Indicates that the audio may not be recorded by any app, even if it is a system app.
    ///
    /// It is encouraged to use `AllowCaptureBySystem` instead of this value as system apps
    /// provide significant and useful features for the user (eg. accessibility).
    /// See android.media.AudioAttributes#ALLOW_CAPTURE_BY_NONE.
    AllowCaptureByNone = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PerformanceMode {
    /// No particular performance needs. Default.
    None = 10,

    /// Extending battery life is more important than low latency.
    ///
    /// This mode is not supported in input streams.
    /// For input, mode NONE will be used if this is requested.
    PowerSaving = 11,

    /// Reducing latency is more important than battery life.
    LowLatency = 12,
}

impl PerformanceMode {
    fn from_i32(val: i32) -> Self {
        match val {
            11 => Self::PowerSaving,
            12 => Self::LowLatency,
            _ => Self::None,
        }
    }
}

/// Value returned the data callback function.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CallbackResult {
    /// Continue calling the callback.
    Continue,

    /// Stop calling the callback.
    ///
    /// The application will still need to call `AAudioStream_requestPause()`
    /// or `AAudioStream_requestStop()`.
    Stop,
}

fn wrap_result(result: i32) -> Result<(), Error> {
    if result < 0 {
        Err(Error::from_code(result))
    } else {
        Ok(())
    }
}

struct StreamCallbacks {
    _data_callback:
        Box<Box<dyn FnMut(&AAudioStreamInfo, &mut [u8], i32) -> CallbackResult + Send + 'static>>,
    _error_callback: Box<Box<dyn FnMut(&AAudioStreamInfo, Error) + Send + 'static>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StreamState {
    Uninitialized,
    Unknown,
    Open,
    Starting,
    Started,
    Pausing,
    Paused,
    Flushing,
    Flushed,
    Stopping,
    Stopped,
    Closing,
    Closed,
    Disconnected,
}

impl StreamState {
    fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::Uninitialized,
            1 => Self::Unknown,
            2 => Self::Open,
            3 => Self::Starting,
            4 => Self::Started,
            5 => Self::Pausing,
            6 => Self::Paused,
            7 => Self::Flushing,
            8 => Self::Flushed,
            9 => Self::Stopping,
            10 => Self::Stopped,
            11 => Self::Closing,
            12 => Self::Closed,
            13 => Self::Disconnected,
            state => panic!("Unexpected stream state: {}", state),
        }
    }
}

pub struct Timestamp {
    pub frame_position: i64,
    pub time_nanos: i64,
}

pub struct AAudioStream {
    raw: *mut AAudioStreamRaw,
    _callbacks: Option<StreamCallbacks>,
}

unsafe impl Send for AAudioStream {}

fn get_timestamp_monotonic(raw: *mut AAudioStreamRaw) -> Result<Timestamp, Error> {
    let mut frame_position = MaybeUninit::uninit();
    let mut time_nanos = MaybeUninit::uninit();
    let result = unsafe {
        ffi::AAudioStream_getTimestamp(
            raw,
            libc::CLOCK_MONOTONIC,
            frame_position.as_mut_ptr(),
            time_nanos.as_mut_ptr(),
        )
    };
    wrap_result(result)?;
    Ok(unsafe {
        Timestamp {
            frame_position: frame_position.assume_init(),
            time_nanos: time_nanos.assume_init(),
        }
    })
}

impl AAudioStream {
    /// Returns the actual sample rate.
    ///
    /// Available since API level 26.
    pub fn get_sample_rate(&self) -> i32 {
        unsafe { ffi::AAudioStream_getSampleRate(self.raw) }
    }

    /// A stream has one or more channels of data.
    /// A frame will contain one sample for each channel.
    ///
    /// Available since API level 26.
    pub fn get_channel_count(&self) -> i32 {
        unsafe { ffi::AAudioStream_getChannelCount(self.raw) }
    }

    /// Query the maximum number of frames that can be filled without blocking.
    ///
    /// Available since API level 26.
    pub fn get_buffer_size_in_frames(&self) -> i32 {
        unsafe { ffi::AAudioStream_getBufferSizeInFrames(self.raw) }
    }

    /// Query the number of frames that the application should read or write at
    /// one time for optimal performance. It is OK if an application writes
    /// a different number of frames. But the buffer size may need to be larger
    /// in order to avoid underruns or overruns.
    ///
    /// Note that this may or may not match the actual device burst size.
    /// For some endpoints, the burst size can vary dynamically.
    /// But these tend to be devices with high latency.
    ///
    /// Available since API level 26.
    pub fn get_frames_per_burst(&self) -> i32 {
        unsafe { ffi::AAudioStream_getFramesPerBurst(self.raw) }
    }

    /// Query maximum buffer capacity in frames.
    ///
    /// Available since API level 26.
    pub fn get_buffer_capacity_in_frames(&self) -> i32 {
        unsafe { ffi::AAudioStream_getBufferCapacityInFrames(self.raw) }
    }

    /// Query the size of the buffer that will be passed to the dataProc callback
    /// in the numFrames parameter.
    ///
    /// This call can be used if the application needs to know the value of numFrames before
    /// the stream is started. This is not normally necessary.
    ///
    /// If a specific size was requested by calling
    /// `AAudioStreamBuilder::set_frames_per_data_callback()` then this will be the same size.
    ///
    /// If `AAudioStreamBuilder_set_frames_per_data_callback()` was not called then this will
    /// return the size chosen by AAudio, or 0.
    ///
    /// 0 indicates that the callback buffer size for this stream
    /// may vary from one dataProc callback to the next.
    ///
    /// Available since API level 26.
    pub fn get_frames_per_data_callback(&self) -> i32 {
        unsafe { ffi::AAudioStream_getFramesPerDataCallback(self.raw) }
    }

    /// An XRun is an Underrun or an Overrun.
    /// During playing, an underrun will occur if the stream is not written in time
    /// and the system runs out of valid data.
    /// During recording, an overrun will occur if the stream is not read in time
    /// and there is no place to put the incoming data so it is discarded.
    ///
    /// An underrun or overrun can cause an audible "pop" or "glitch".
    ///
    /// Note that some INPUT devices may not support this function.
    /// In that case a 0 will always be returned.
    ///
    /// Available since API level 26.
    pub fn get_x_run_count(&self) -> i32 {
        unsafe { ffi::AAudioStream_getXRunCount(self.raw) }
    }

    /// Returns the actual device ID.
    ///
    /// Available since API level 26.
    pub fn get_device_id(&self) -> i32 {
        unsafe { ffi::AAudioStream_getDeviceId(self.raw) }
    }

    /// Returns the actual data format.
    ///
    /// Available since API level 26.
    pub fn get_format(&self) -> Format {
        let val = unsafe { ffi::AAudioStream_getFormat(self.raw) };
        Format::from_i32(val)
    }

    /// Provide actual sharing mode.
    ///
    /// Available since API level 26.
    pub fn get_sharing_mode(&self) -> SharingMode {
        let val = unsafe { ffi::AAudioStream_getSharingMode(self.raw) };
        SharingMode::from_i32(val)
    }

    /// Get the performance mode used by the stream.
    ///
    /// Available since API level 26.
    pub fn get_performance_mode(&self) -> PerformanceMode {
        let val = unsafe { ffi::AAudioStream_getPerformanceMode(self.raw) };
        PerformanceMode::from_i32(val)
    }

    /// Available since API level 26.
    pub fn get_direction(&self) -> Direction {
        let val = unsafe { ffi::AAudioStream_getDirection(self.raw) };
        Direction::from_i32(val)
    }

    /// Returns the number of frames that have been written since the stream was created.
    /// For an output stream, this will be advanced by the application calling write()
    /// or by a data callback.
    /// For an input stream, this will be advanced by the endpoint.
    ///
    /// The frame position is monotonically increasing.
    ///
    /// Available since API level 26.
    pub fn get_frames_written(&self) -> i64 {
        unsafe { ffi::AAudioStream_getFramesWritten(self.raw) }
    }

    /// Returns the number of frames that have been read since the stream was created.
    /// For an output stream, this will be advanced by the endpoint.
    /// For an input stream, this will be advanced by the application calling read()
    /// or by a data callback.
    ///
    /// The frame position is monotonically increasing.
    ///
    /// Available since API level 26.
    pub fn get_frames_read(&self) -> i64 {
        unsafe { ffi::AAudioStream_getFramesRead(self.raw) }
    }

    /// Passes back the session ID associated with this stream.
    ///
    /// The session ID can be used to associate a stream with effects processors.
    /// The effects are controlled using the Android AudioEffect Java API.
    ///
    /// If `AAudioStreamBuilder::set_session_id()` was called with 0
    /// then a new session ID should be allocated once when the stream is opened.
    ///
    /// If `AAudioStreamBuilder::set_session_id()` was called with a previously allocated
    /// session ID then that value should be returned.
    ///
    /// If `AAudioStreamBuilder::set_session_id()` was not called then this function should
    /// return -1.
    ///
    /// The sessionID for a stream should not change once the stream has been opened.
    ///
    /// Available since API level 28.
    pub fn get_session_id(&self) -> i32 {
        unsafe { ffi::AAudioStream_getSessionId(self.raw) }
    }

    /// Returns the time at which a particular frame was presented.
    /// This can be used to synchronize audio with video or MIDI.
    /// It can also be used to align a recorded stream with a playback stream.
    ///
    /// Timestamps are only valid when the stream is in `Started` state.
    /// `InvalidState` will be returned if the stream is not started.
    /// Note that because request_start() is asynchronous, timestamps will not be valid until
    /// a short time after calling request_start().
    /// So `InvalidState` should not be considered a fatal error.
    /// Just try calling again later.
    ///
    /// If an error occurs, then the position and time will not be modified.
    ///
    /// The position and time passed back are monotonically increasing.
    ///
    /// Available since API level 26.
    pub fn get_timestamp_monotonic(&self) -> Result<Timestamp, Error> {
        get_timestamp_monotonic(self.raw)
    }

    /// Query the current state of the client, eg. `Pausing`.
    ///
    /// This function will immediately return the state without updating the state.
    /// If you want to update the client state based on the server state then
    /// call `AAudioStream::wait_for_state_change()` with currentState
    /// set to `Unknown` and a zero timeout.
    ///
    /// Available since API level 26.
    pub fn get_state(&self) -> StreamState {
        let val = unsafe { ffi::AAudioStream_getState(self.raw) };
        StreamState::from_i32(val)
    }

    /// Free the audio resources associated with the stream.
    ///
    /// After this call, the stream will be in `Closing` state.
    ///
    /// This function is useful if you want to release the audio resources immediately,
    /// but still allow queries to the stream to occur from other threads. This often
    /// happens if you are monitoring stream progress from a UI thread.
    ///
    /// Available since API level 30.
    pub fn release(&mut self) -> Result<(), Error> {
        let val = unsafe { ffi::AAudioStream_release(self.raw) };
        wrap_result(val)
    }

    /// Asynchronously request to start playing the stream. For output streams, one should
    /// write to the stream to fill the buffer before starting.
    /// Otherwise it will underflow.
    /// After this call the state will be in `Starting` or `Started`.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// Available since API level 26.
    pub fn request_start(&mut self) -> Result<(), Error> {
        let val = unsafe { ffi::AAudioStream_requestStart(self.raw) };
        wrap_result(val)
    }

    /// Asynchronous request for the stream to pause.
    /// Pausing a stream will freeze the data flow but not flush any buffers.
    /// Use `AAudioStream::request_start()` to resume playback after a pause.
    /// After this call the state will be in `Pausing` or
    /// `Paused`.
    ///
    /// This will return `Unimplemented` for input streams.
    /// For input streams use `AAudioStream::request_stop()`.
    ///
    /// Available since API level 26.
    pub fn request_pause(&mut self) -> Result<(), Error> {
        let val = unsafe { ffi::AAudioStream_requestPause(self.raw) };
        wrap_result(val)
    }

    /// Asynchronous request for the stream to flush.
    /// Flushing will discard any pending data.
    /// This call only works if the stream is pausing or paused.
    /// Frame counters are not reset by a flush. They may be advanced.
    /// After this call the state will be in `Flushing` or `Flushed`.
    ///
    /// This will return `Unimplemented` for input streams.
    ///
    /// Available since API level 26.
    pub fn request_flush(&mut self) -> Result<(), Error> {
        let val = unsafe { ffi::AAudioStream_requestFlush(self.raw) };
        wrap_result(val)
    }

    /// Asynchronous request for the stream to stop.
    /// The stream will stop after all of the data currently buffered has been played.
    /// After this call the state will be in `Stopping` or `Stopped`.
    ///
    /// Available since API level 26.
    pub fn request_stop(&mut self) -> Result<(), Error> {
        let val = unsafe { ffi::AAudioStream_requestStop(self.raw) };
        wrap_result(val)
    }

    /// Wait until the current state no longer matches the input state.
    ///
    /// This will update the current client state.
    ///
    /// Returns the new state.
    ///
    /// Available since API level 26.
    pub fn wait_for_state_change(
        &mut self,
        input_state: StreamState,
        timeout_nanos: i64,
    ) -> Result<StreamState, Error> {
        let mut new_state = MaybeUninit::uninit();
        let result = unsafe {
            ffi::AAudioStream_waitForStateChange(
                self.raw,
                input_state as i32,
                new_state.as_mut_ptr(),
                timeout_nanos,
            )
        };
        wrap_result(result)?;
        Ok(unsafe { StreamState::from_i32(new_state.assume_init()) })
    }

    /// Read data from the stream.
    /// Returns the number of frames actually read or a negative error.
    ///
    /// The call will wait until the read is complete or until it runs out of time.
    /// If timeoutNanos is zero then this call will not wait.
    ///
    /// Note that timeoutNanoseconds is a relative duration in wall clock time.
    /// Time will not stop if the thread is asleep.
    /// So it will be implemented using CLOCK_BOOTTIME.
    ///
    /// This call is "strong non-blocking" unless it has to wait for data.
    ///
    /// If the call times out then zero or a partial frame count will be returned.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The slice with the samples.
    /// * `num_frames` - Number of frames to read. Only complete frames will be written.
    /// * `timeout_nanoseconds` - Maximum number of nanoseconds to wait for completion.
    pub fn read(
        &mut self,
        buffer: &mut [u8],
        num_frames: i32,
        timeout_nanoseconds: i64,
    ) -> Result<u32, Error> {
        let result = unsafe {
            ffi::AAudioStream_read(
                self.raw,
                buffer.as_mut_ptr() as *mut c_void,
                num_frames,
                timeout_nanoseconds,
            )
        };
        wrap_result(result)?;
        Ok(result as u32)
    }

    /// Write data to the stream.
    /// Returns the number of frames actually written or a negative error.
    ///
    /// The call will wait until the write is complete or until it runs out of time.
    /// If timeoutNanos is zero then this call will not wait.
    ///
    /// Note that timeoutNanoseconds is a relative duration in wall clock time.
    /// Time will not stop if the thread is asleep.
    /// So it will be implemented using CLOCK_BOOTTIME.
    ///
    /// This call is "strong non-blocking" unless it has to wait for room in the buffer.
    ///
    /// If the call times out then zero or a partial frame count will be returned.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The address of the first sample.
    /// * `num_frames` - Number of frames to write. Only complete frames will be written.
    /// * `timeout_nanoseconds` - Maximum number of nanoseconds to wait for completion.
    pub fn write(
        &mut self,
        buffer: &[u8],
        num_frames: i32,
        timeout_nanoseconds: i64,
    ) -> Result<u32, Error> {
        let result = unsafe {
            ffi::AAudioStream_write(
                self.raw,
                buffer.as_ptr() as *const c_void,
                num_frames,
                timeout_nanoseconds,
            )
        };
        wrap_result(result)?;
        Ok(result as u32)
    }

    /// This can be used to adjust the latency of the buffer by changing
    /// the threshold where blocking will occur.
    /// By combining this with `AAudioStream::get_x_run_count()`, the latency can be tuned
    /// at run-time for each device.
    /// Returns actual buffer size in frames or a negative error.
    ///
    /// This cannot be set higher than `AAudioStream::get_buffer_capacity_in_frames()`.
    ///
    /// Note that you will probably not get the exact size you request.
    /// You can check the return value or call `AAudioStream::get_buffer_size_in_frames()`
    /// to see what the actual final size is.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `num_frames` - requested number of frames that can be filled without blocking
    pub fn set_buffer_size_in_frames(&mut self, num_frames: i32) -> Result<(), Error> {
        let result = unsafe { ffi::AAudioStream_setBufferSizeInFrames(self.raw, num_frames) };
        wrap_result(result)
    }
}

impl Drop for AAudioStream {
    fn drop(&mut self) {
        unsafe {
            ffi::AAudioStream_close(self.raw);
        }
    }
}

/// Passed as a callback parameter, providing operations that are safe
/// to perform from callback invocation.
pub struct AAudioStreamInfo {
    raw: *mut AAudioStreamRaw,
}

unsafe impl Send for AAudioStreamInfo {}

impl AAudioStreamInfo {
    /// Returns the actual sample rate.
    ///
    /// Available since API level 26.
    pub fn get_sample_rate(&self) -> i32 {
        unsafe { ffi::AAudioStream_getSampleRate(self.raw) }
    }

    /// A stream has one or more channels of data.
    /// A frame will contain one sample for each channel.
    ///
    /// Available since API level 26.
    pub fn get_channel_count(&self) -> i32 {
        unsafe { ffi::AAudioStream_getChannelCount(self.raw) }
    }

    /// Query the maximum number of frames that can be filled without blocking.
    ///
    /// Available since API level 26.
    pub fn get_buffer_size_in_frames(&self) -> i32 {
        unsafe { ffi::AAudioStream_getBufferSizeInFrames(self.raw) }
    }

    /// Query the number of frames that the application should read or write at
    /// one time for optimal performance. It is OK if an application writes
    /// a different number of frames. But the buffer size may need to be larger
    /// in order to avoid underruns or overruns.
    ///
    /// Note that this may or may not match the actual device burst size.
    /// For some endpoints, the burst size can vary dynamically.
    /// But these tend to be devices with high latency.
    ///
    /// Available since API level 26.
    pub fn get_frames_per_burst(&self) -> i32 {
        unsafe { ffi::AAudioStream_getFramesPerBurst(self.raw) }
    }

    /// Query maximum buffer capacity in frames.
    ///
    /// Available since API level 26.
    pub fn get_buffer_capacity_in_frames(&self) -> i32 {
        unsafe { ffi::AAudioStream_getBufferCapacityInFrames(self.raw) }
    }

    /// Query the size of the buffer that will be passed to the dataProc callback
    /// in the numFrames parameter.
    ///
    /// This call can be used if the application needs to know the value of numFrames before
    /// the stream is started. This is not normally necessary.
    ///
    /// If a specific size was requested by calling
    /// `AAudioStreamBuilder::set_frames_per_data_callback()` then this will be the same size.
    ///
    /// If `AAudioStreamBuilder::set_frames_per_data_callback()` was not called then this will
    /// return the size chosen by AAudio, or 0.
    ///
    /// 0 indicates that the callback buffer size for this stream
    /// may vary from one dataProc callback to the next.
    ///
    /// Available since API level 26.
    pub fn get_frames_per_data_callback(&self) -> i32 {
        unsafe { ffi::AAudioStream_getFramesPerDataCallback(self.raw) }
    }

    /// An XRun is an Underrun or an Overrun.
    /// During playing, an underrun will occur if the stream is not written in time
    /// and the system runs out of valid data.
    /// During recording, an overrun will occur if the stream is not read in time
    /// and there is no place to put the incoming data so it is discarded.
    ///
    /// An underrun or overrun can cause an audible "pop" or "glitch".
    ///
    /// Note that some INPUT devices may not support this function.
    /// In that case a 0 will always be returned.
    ///
    /// Available since API level 26.
    pub fn get_x_run_count(&self) -> i32 {
        unsafe { ffi::AAudioStream_getXRunCount(self.raw) }
    }

    /// Returns the actual device ID.
    ///
    /// Available since API level 26.
    pub fn get_device_id(&self) -> i32 {
        unsafe { ffi::AAudioStream_getDeviceId(self.raw) }
    }

    /// Returns the actual data format.
    ///
    /// Available since API level 26.
    pub fn get_format(&self) -> Format {
        let val = unsafe { ffi::AAudioStream_getFormat(self.raw) };
        Format::from_i32(val)
    }

    /// Provide actual sharing mode.
    ///
    /// Available since API level 26.
    pub fn get_sharing_mode(&self) -> SharingMode {
        let val = unsafe { ffi::AAudioStream_getSharingMode(self.raw) };
        SharingMode::from_i32(val)
    }

    /// Get the performance mode used by the stream.
    ///
    /// Available since API level 26.
    pub fn get_performance_mode(&self) -> PerformanceMode {
        let val = unsafe { ffi::AAudioStream_getPerformanceMode(self.raw) };
        PerformanceMode::from_i32(val)
    }

    /// Available since API level 26.
    pub fn get_direction(&self) -> Direction {
        let val = unsafe { ffi::AAudioStream_getDirection(self.raw) };
        Direction::from_i32(val)
    }

    /// Returns the number of frames that have been written since the stream was created.
    /// For an output stream, this will be advanced by the application calling `write()`
    /// or by a data callback.
    /// For an input stream, this will be advanced by the endpoint.
    ///
    /// The frame position is monotonically increasing.
    ///
    /// Available since API level 26.
    pub fn get_frames_written(&self) -> i64 {
        unsafe { ffi::AAudioStream_getFramesWritten(self.raw) }
    }

    /// Returns the number of frames that have been read since the stream was created.
    /// For an output stream, this will be advanced by the endpoint.
    /// For an input stream, this will be advanced by the application calling `read()`
    /// or by a data callback.
    ///
    /// The frame position is monotonically increasing.
    ///
    /// Available since API level 26.
    pub fn get_frames_read(&self) -> i64 {
        unsafe { ffi::AAudioStream_getFramesRead(self.raw) }
    }

    /// Passes back the session ID associated with this stream.
    ///
    /// The session ID can be used to associate a stream with effects processors.
    /// The effects are controlled using the Android AudioEffect Java API.
    ///
    /// If `AAudioStreamBuilder::set_session_id()` was called with 0
    /// then a new session ID should be allocated once when the stream is opened.
    ///
    /// If `AAudioStreamBuilder::set_session_id()` was called with a previously allocated
    /// session ID then that value should be returned.
    ///
    /// If `AAudioStreamBuilder::set_session_id()` was not called then this function should
    /// return -1.
    ///
    /// The sessionID for a stream should not change once the stream has been opened.
    ///
    /// Available since API level 28.
    pub fn get_session_id(&self) -> i32 {
        unsafe { ffi::AAudioStream_getSessionId(self.raw) }
    }

    /// Returns the time at which a particular frame was presented.
    /// This can be used to synchronize audio with video or MIDI.
    /// It can also be used to align a recorded stream with a playback stream.
    ///
    /// Timestamps are only valid when the stream is in `Started` state.
    /// `InvalidState` will be returned if the stream is not started.
    /// Note that because request_start() is asynchronous, timestamps will not be valid until
    /// a short time after calling request_start().
    /// So `InvalidState` should not be considered a fatal error.
    /// Just try calling again later.
    ///
    /// If an error occurs, then the position and time will not be modified.
    ///
    /// The position and time passed back are monotonically increasing.
    ///
    /// Available since API level 26.
    pub fn get_timestamp_monotonic(&self) -> Result<Timestamp, Error> {
        get_timestamp_monotonic(self.raw)
    }

    /// Query the current state of the client, eg. `Pausing`.
    ///
    /// This function will immediately return the state without updating the state.
    /// If you want to update the client state based on the server state then
    /// call AAudioStream::wait_for_state_change() with currentState
    /// set to `Unknown` and a zero timeout.
    ///
    /// Available since API level 26.
    pub fn get_state(&self) -> StreamState {
        let val = unsafe { ffi::AAudioStream_getState(self.raw) };
        StreamState::from_i32(val)
    }
}

pub struct AAudioStreamBuilder {
    raw: *mut AAudioStreamBuilderRaw,
    callbacks: Option<StreamCallbacks>,
}

unsafe extern "C" fn raw_data_callback(
    stream: *mut AAudioStreamRaw,
    user_data: *mut c_void,
    audio_data: *mut c_void,
    num_frames: i32,
) -> i32 {
    match std::panic::catch_unwind(|| {
        let stream = AAudioStreamInfo { raw: stream };
        let data: &mut [u8] = std::slice::from_raw_parts_mut(
            audio_data as *mut u8,
            (num_frames * stream.get_channel_count() * stream.get_format().sample_size()) as usize,
        );
        let callback =
            user_data as *mut Box<dyn FnMut(&AAudioStreamInfo, &mut [u8], i32) -> CallbackResult>;
        let callback = &mut *callback;
        callback(&stream, data, num_frames) as i32
    }) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::abort();
        }
    }
}

unsafe extern "C" fn raw_error_callback(
    stream: *mut AAudioStreamRaw,
    user_data: *mut c_void,
    error: i32,
) {
    if let Err(e) = std::panic::catch_unwind(|| {
        let stream = AAudioStreamInfo { raw: stream };
        let callback = user_data as *mut Box<dyn FnMut(&AAudioStreamInfo, Error)>;
        let callback = &mut *callback;
        callback(&stream, Error::from_code(error));
    }) {
        eprintln!("{:?}", e);
        std::process::abort();
    }
}

impl AAudioStreamBuilder {
    pub fn new() -> Result<Self, Error> {
        let mut raw = MaybeUninit::<*mut AAudioStreamBuilderRaw>::uninit();
        let result = unsafe { ffi::AAudio_createStreamBuilder(raw.as_mut_ptr()) };
        wrap_result(result)?;
        Ok(Self {
            raw: unsafe { raw.assume_init() },
            callbacks: None,
        })
    }

    /// Request that AAudio call the `data_callback` when the stream is running and the
    /// `error_callback` if any error occurs or the stream is disconnected.
    ///
    /// Note that when using data callback, the audio data will be passed in or out
    /// of the function as an argument.
    /// So you cannot call `AAudioStream::write()` or `AAudioStream::read()`
    /// on the same stream that has an active data callback.
    ///
    /// The data callback function will start being called after `AAudioStream::request_start()`
    /// is called.
    /// It will stop being called after `AAudioStream::request_pause()` or
    /// `AAudioStream::request_stop()` is called.
    ///
    /// The `data_callback` function will be called on a real-time thread owned by AAudio.
    /// Note that numFrames can vary unless `AAudioStreamBuilder::set_frames_per_data_callback()`
    /// is called.
    ///
    /// Also note that this callback function should be considered a "real-time" function.
    /// It must not do anything that could cause an unbounded delay because that can cause the
    /// audio to glitch or pop.
    ///
    /// These are things the function should NOT do:
    /// * allocate memory using, for example, malloc() or new
    /// * any file operations such as opening, closing, reading or writing
    /// * any network operations such as streaming
    /// * use any mutexes or other synchronization primitives
    /// * sleep
    /// * stop or close the stream
    /// * `AAudioStream::read()`
    /// * `AAudioStream::write()`
    ///
    /// If you need to move data, eg. MIDI commands, in or out of the callback function then
    /// we recommend the use of non-blocking techniques such as an atomic FIFO.
    ///
    /// The `error_callback` will be called, for example, if a headset or a USB device is unplugged causing the stream's
    /// device to be unavailable or "disconnected".
    /// Another possible cause of error would be a timeout or an unanticipated internal error.
    ///
    /// In response, this function should signal or create another thread to stop
    /// and close this stream. The other thread could then reopen a stream on another device.
    /// Do not stop or close the stream, or reopen the new stream, directly from this callback.
    ///
    /// The `error_callback` will not be called because of actions by the application, such as stopping
    /// or closing a stream.
    ///
    /// Note that the AAudio callbacks will never be called simultaneously from multiple threads.
    ///
    /// Available since API level 26.
    pub fn set_callbacks<D, E>(mut self, data_callback: D, error_callback: E) -> Self
    where
        D: FnMut(&AAudioStreamInfo, &mut [u8], i32) -> CallbackResult + Send + 'static,
        E: FnMut(&AAudioStreamInfo, Error) + Send + 'static,
    {
        let data_callback: Box<
            Box<dyn FnMut(&AAudioStreamInfo, &mut [u8], i32) -> CallbackResult + Send + 'static>,
        > = Box::new(Box::new(data_callback));
        let error_callback: Box<Box<dyn FnMut(&AAudioStreamInfo, Error) + Send + 'static>> =
            Box::new(Box::new(error_callback));
        let data_callback_raw = Box::into_raw(data_callback);
        let error_callback_raw = Box::into_raw(error_callback);
        let callbacks = StreamCallbacks {
            _data_callback: unsafe { Box::from_raw(data_callback_raw) },
            _error_callback: unsafe { Box::from_raw(error_callback_raw) },
        };
        unsafe {
            ffi::AAudioStreamBuilder_setDataCallback(
                self.raw,
                Some(raw_data_callback),
                data_callback_raw as *mut c_void,
            );
            ffi::AAudioStreamBuilder_setErrorCallback(
                self.raw,
                Some(raw_error_callback),
                error_callback_raw as *mut c_void,
            );
        }
        self.callbacks = Some(callbacks);
        self
    }

    /// Request an audio device identified device using an ID.
    /// On Android, for example, the ID could be obtained from the Java AudioManager.
    ///
    /// The default, if you do not call this function, is 0,
    /// in which case the primary device will be used.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `device_id` - device identifier or 0 for unspecified
    pub fn set_device_id(self, device_id: i32) -> Self {
        unsafe {
            ffi::AAudioStreamBuilder_setDeviceId(self.raw, device_id);
        }
        self
    }

    /// Request a sample rate in Hertz.
    ///
    /// The default, if you do not call this function, is 0 (unspecified).
    /// An optimal value will then be chosen when the stream is opened.
    /// After opening a stream with an unspecified value, the application must
    /// query for the actual value, which may vary by device.
    ///
    /// If an exact value is specified then an opened stream will use that value.
    /// If a stream cannot be opened with the specified value then the open will fail.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - frames per second. Common rates include 44100 and 48000 Hz.
    pub fn set_sample_rate(self, sample_rate: i32) -> Self {
        unsafe {
            ffi::AAudioStreamBuilder_setSampleRate(self.raw, sample_rate);
        }
        self
    }

    /// Request a number of channels for the stream.
    ///
    /// The default, if you do not call this function, is unspecified.
    /// An optimal value will then be chosen when the stream is opened.
    /// After opening a stream with an unspecified value, the application must
    /// query for the actual value, which may vary by device.
    ///
    /// If an exact value is specified then an opened stream will use that value.
    /// If a stream cannot be opened with the specified value then the open will fail.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `channel_count` - Number of channels desired.
    pub fn set_channel_count(self, channel_count: i32) -> Self {
        unsafe {
            ffi::AAudioStreamBuilder_setChannelCount(self.raw, channel_count);
        }
        self
    }

    /// Request a sample data format, for example `Format::I16`.
    ///
    /// The default, if you do not call this function, is `Unspecified`.
    /// An optimal value will then be chosen when the stream is opened.
    /// After opening a stream with an unspecified value, the application must
    /// query for the actual value, which may vary by device.
    ///
    /// If an exact value is specified then an opened stream will use that value.
    /// If a stream cannot be opened with the specified value then the open will fail.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `format` - the sample data format.
    pub fn set_format(self, format: Format) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setFormat(self.raw, format as i32) }
        self
    }

    /// Request a mode for sharing the device.
    ///
    /// The default, if you do not call this function, is `SharingMode::Shared`.
    ///
    /// The requested sharing mode may not be available.
    /// The application can query for the actual mode after the stream is opened.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `sharing_mode` - `SharingMode::Shared` or `SharingMode::Exclusive`
    pub fn set_sharing_mode(self, sharing_mode: SharingMode) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setSharingMode(self.raw, sharing_mode as i32) }
        self
    }

    /// Request the direction for a stream.
    ///
    /// The default, if you do not call this function, is `Direction::Output`.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `direction` - `Direction::Output` or `Direction::Input`
    pub fn set_direction(self, direction: Direction) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setDirection(self.raw, direction as i32) }
        self
    }

    /// Set the requested buffer capacity in frames.
    /// The final AAudioStream capacity may differ, but will probably be at least this big.
    ///
    /// The default, if you do not call this function, is unspecified.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `num_frames` - the desired buffer capacity in frames or 0 for unspecified
    pub fn set_buffer_capacity_in_frames(self, num_frames: i32) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setBufferCapacityInFrames(self.raw, num_frames as i32) }
        self
    }

    /// Set the requested performance mode.
    ///
    /// Supported modes are None, PowerSaving and LowLatency.
    ///
    /// The default, if you do not call this function, is None.
    ///
    /// You may not get the mode you requested.
    /// You can call `AAudioStream::get_performance_mode()`
    /// to find out the final mode for the stream.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `mode` - the desired performance mode, eg. LowLatency
    pub fn set_performance_mode(self, mode: PerformanceMode) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setPerformanceMode(self.raw, mode as i32) }
        self
    }

    /// Set the intended use case for the stream.
    ///
    /// The AAudio system will use this information to optimize the
    /// behavior of the stream.
    /// This could, for example, affect how volume and focus is handled for the stream.
    ///
    /// The default, if you do not call this function, is `Usage::Media`.
    ///
    /// Available since API level 28.
    ///
    /// * `usage` - the desired usage, eg. `Usage::Game`
    pub fn set_usage(self, usage: Usage) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setUsage(self.raw, usage as i32) }
        self
    }

    /// Set the type of audio data that the stream will carry.
    ///
    /// The AAudio system will use this information to optimize the
    /// behavior of the stream.
    /// This could, for example, affect whether a stream is paused when a notification occurs.
    ///
    /// The default, if you do not call this function, is `ContentType::Music`.
    ///
    /// Available since API level 28.
    ///
    /// # Arguments
    ///
    /// * `content_type` - the type of audio data, eg. `ContentType::Speech`
    pub fn set_content_type(self, content_type: ContentType) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setContentType(self.raw, content_type as i32) }
        self
    }

    /// Set the input (capture) preset for the stream.
    ///
    /// The AAudio system will use this information to optimize the
    /// behavior of the stream.
    /// This could, for example, affect which microphones are used and how the
    /// recorded data is processed.
    ///
    /// The default, if you do not call this function, is `InputPreset::VoiceRecognition`.
    /// That is because `InputPreset::VoiceRecognition` is the preset with the lowest latency
    /// on many platforms.
    ///
    /// Available since API level 28.
    ///
    /// # Arguments
    ///
    /// * `input_preset` - the desired configuration for recording
    pub fn set_input_preset(self, input_preset: InputPreset) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setInputPreset(self.raw, input_preset as i32) }
        self
    }

    /// Specify whether this stream audio may or may not be captured by other apps or the system.
    ///
    /// The default is `AllowedCapturePolicy::AllowCaptureByAll`.
    ///
    /// Note that an application can also set its global policy, in which case the most restrictive
    /// policy is always applied. See android.media.AudioAttributes#setAllowedCapturePolicy(int)
    ///
    /// Available since API level 29.
    ///
    /// # Arguments
    ///
    /// * `policy` - the desired level of opt-out from being captured.
    pub fn set_allowed_capture_policy(self, policy: AllowedCapturePolicy) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setAllowedCapturePolicy(self.raw, policy as i32) }
        self
    }

    /// Equivalent to invoking `AAudioStreamBuilder::set_session_id` with 0 argument.
    pub fn allocate_session_id(self) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setSessionId(self.raw, 0) }
        self
    }

    /// Equivalent to invoking `AAudioStreamBuilder::set_session_id` with -1 argument.
    pub fn remove_session_id(self) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setSessionId(self.raw, -1) }
        self
    }

    /// The session ID can be used to associate a stream with effects processors.
    /// The effects are controlled using the Android AudioEffect Java API.
    ///
    /// The default, if you do not call this function, is -1 (none).
    ///
    /// If set to 0 then a session ID will be allocated
    /// when the stream is opened.
    ///
    /// The allocated session ID can be obtained by calling `AAudioStream::get_session_id()`
    /// and then used with this function when opening another stream.
    /// This allows effects to be shared between streams.
    ///
    /// Session IDs from AAudio can be used with the Android Java APIs and vice versa.
    /// So a session ID from an AAudio stream can be passed to Java
    /// and effects applied using the Java AudioEffect API.
    ///
    /// Note that allocating or setting a session ID may result in a stream with higher latency.
    ///
    /// Allocated session IDs will always be positive and nonzero.
    ///
    /// Available since API level 28.
    ///
    /// # Arguments
    ///
    /// * `session_id` - an allocated sessionID or 0 to allocate a new sessionID
    pub fn set_session_id(self, session_id: i32) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setSessionId(self.raw, session_id as i32) }
        self
    }

    /// Indicates whether this input stream must be marked as privacy sensitive or not.
    ///
    /// When true, this input stream is privacy sensitive and any concurrent capture
    /// is not permitted.
    ///
    /// This is off (false) by default except when the input preset is `InputPreset::VoiceCommunication`
    /// or `InputPreset::Camcorder`.
    ///
    /// Always takes precedence over default from input preset when set explicitly.
    ///
    /// Only relevant if the stream direction is `Direction::Input`.
    ///
    /// Added in API level 30.
    ///
    /// # Arguments
    ///
    /// * `privacy_sensitive` - `true` if capture from this stream must be marked as privacy sensitive, `false` otherwise.
    pub fn set_privacy_sensitive(self, privacy_sensitive: bool) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setPrivacySensitive(self.raw, privacy_sensitive) }
        self
    }

    /// Set the requested data callback buffer size in frames.
    /// See [`set_callbacks`].
    ///
    /// The default, if you do not call this function, is unspecified.
    ///
    /// For the lowest possible latency, do not call this function. AAudio will then
    /// call the dataProc callback function with whatever size is optimal.
    /// That size may vary from one callback to another.
    ///
    /// Only use this function if the application requires a specific number of frames for processing.
    /// The application might, for example, be using an FFT that requires
    /// a specific power-of-two sized buffer.
    ///
    /// AAudio may need to add additional buffering in order to adapt between the internal
    /// buffer size and the requested buffer size.
    ///
    /// If you do call this function then the requested size should be less than
    /// half the buffer capacity, to allow double buffering.
    ///
    /// Available since API level 26.
    ///
    /// * `num_frames` - the desired buffer size in frames or 0 for unspecified
    ///
    /// [`set_callbacks`]: AAudioStreamBuilder::set_callbacks
    pub fn set_frames_per_data_callback(self, num_frames: i32) -> Self {
        unsafe { ffi::AAudioStreamBuilder_setFramesPerDataCallback(self.raw, num_frames) }
        self
    }

    /// Open a stream based on the options in the AAudioStreamBuilder.
    pub fn open_stream(mut self) -> Result<AAudioStream, Error> {
        let mut raw = MaybeUninit::<*mut AAudioStreamRaw>::uninit();
        let result = unsafe { ffi::AAudioStreamBuilder_openStream(self.raw, raw.as_mut_ptr()) };
        wrap_result(result)?;
        let stream = AAudioStream {
            raw: unsafe { raw.assume_init() },
            _callbacks: self.callbacks.take(),
        };
        Ok(stream)
    }
}

impl Drop for AAudioStreamBuilder {
    fn drop(&mut self) {
        unsafe {
            ffi::AAudioStreamBuilder_delete(self.raw);
        }
    }
}
