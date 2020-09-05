extern crate libc;

use std::ffi::c_void;

/// Low-level wrappers for AAudio API.

pub enum AAudioStream {}
pub enum AAudioStreamBuilder {}

/// This is used to represent a value that has not been specified.
/// For example, an application could use `UNSPECIFIED` to indicate
/// that is did not not care what the specific value of a parameter was
/// and would accept whatever it was given.
pub const UNSPECIFIED: i32 = 0;

pub const SESSION_ID_NONE: i32 = -1;
pub const SESSION_ID_ALLOCATE: i32 = 0;

pub const DIRECTION_OUTPUT: i32 = 0;
pub const DIRECTION_INPUT: i32 = 0;

pub const FORMAT_INVALID: i32 = -1;
pub const FORMAT_PCM_I16: i32 = 1;
pub const FORMAT_PCM_FLOAT: i32 = 2;

pub const SHARING_EXCLUSIVE: i32 = 0;
pub const SHARING_SHARED: i32 = 1;

pub const USAGE_MEDIA: i32 = 1;
pub const USAGE_VOICE_COMMUNICATION: i32 = 2;
pub const USAGE_VOICE_COMMUNICATION_SIGNALLING: i32 = 3;
pub const USAGE_ALARM: i32 = 4;
pub const USAGE_NOTIFICATION: i32 = 5;
pub const USAGE_NOTIFICATION_RINGTONE: i32 = 6;
pub const USAGE_NOTIFICATION_EVENT: i32 = 10;
pub const USAGE_ASSISTANCE_ACCESSIBILITY: i32 = 11;
pub const USAGE_ASSISTANCE_NAVIGATION_GUIDANCE: i32 = 12;
pub const USAGE_ASSISTANCE_SONIFICATION: i32 = 13;
pub const USAGE_GAME: i32 = 14;
pub const USAGE_ASSISTANT: i32 = 16;
pub const USAGE_EMERGENCY: i32 = 1000;
pub const USAGE_SAFETY: i32 = 1001;
pub const USAGE_VEHICLE_STATUS: i32 = 1002;
pub const USAGE_ANNOUNCEMENT: i32 = 1003;

pub const INPUT_PRESET_GENERIC: i32 = 1;
pub const INPUT_PRESET_CAMCORDER: i32 = 5;
pub const INPUT_PRESET_VOICE_RECOGNITION: i32 = 6;
pub const INPUT_PRESET_VOICE_COMMUNICATION: i32 = 7;
pub const INPUT_PRESET_UNPROCESSED: i32 = 9;
pub const INPUT_PRESET_VOICE_PERFORMANCE: i32 = 10;

pub const CONTENT_TYPE_SPEECH: i32 = 1;
pub const CONTENT_TYPE_MUSIC: i32 = 2;
pub const CONTENT_TYPE_MOVIE: i32 = 3;
pub const CONTENT_TYPE_SONIFICATION: i32 = 4;

pub const ALLOW_CAPTURE_BY_ALL: i32 = 1;
pub const ALLOW_CAPTURE_BY_SYSTEM: i32 = 2;
pub const ALLOW_CAPTURE_BY_NONE: i32 = 3;

pub const CALLBACK_CONTINUE: i32 = 0;
pub const CALLBACK_STOP: i32 = 1;

pub const PERFORMANCE_MODE_NONE: i32 = 10;
pub const PERFORMANCE_MODE_POWER_SAVING: i32 = 11;
pub const PERFORMANCE_MODE_LOW_LATENCY: i32 = 12;

pub const STREAM_STATE_UNINITIALIZED: i32 = 0;
pub const STREAM_STATE_UNKNOWN: i32 = 1;
pub const STREAM_STATE_OPEN: i32 = 2;
pub const STREAM_STATE_STARTING: i32 = 3;
pub const STREAM_STATE_STARTED: i32 = 4;
pub const STREAM_STATE_PAUSING: i32 = 5;
pub const STREAM_STATE_PAUSED: i32 = 6;
pub const STREAM_STATE_FLUSHING: i32 = 7;
pub const STREAM_STATE_FLUSHED: i32 = 8;
pub const STREAM_STATE_STOPPING: i32 = 9;
pub const STREAM_STATE_STOPPED: i32 = 10;
pub const STREAM_STATE_CLOSING: i32 = 11;
pub const STREAM_STATE_CLOSED: i32 = 12;
pub const STREAM_STATE_DISCONNECTED: i32 = 13;

pub const OK: i32 = 0;

const ERROR_BASE: i32 = -900;

/// The audio device was disconnected. This could occur, for example, when headphones
/// are plugged in or unplugged. The stream cannot be used after the device is disconnected.
/// Applications should stop and close the stream.
/// If this error is received in an error callback then another thread should be
/// used to stop and close the stream.
pub const ERROR_DISCONNECTED: i32 = ERROR_BASE + 1;

/// An invalid parameter was passed to AAudio.
pub const ERROR_ILLEGAL_ARGUMENT: i32 = ERROR_BASE + 2;

pub const ERROR_INTERNAL: i32 = ERROR_ILLEGAL_ARGUMENT + 2;

/// The requested operation is not appropriate for the current state of AAudio.
pub const ERROR_INVALID_STATE: i32 = ERROR_INTERNAL + 1;

/// The server rejected the handle used to identify the stream.
pub const ERROR_INVALID_HANDLE: i32 = ERROR_INVALID_STATE + 3;

/// The function is not implemented for this stream.
pub const ERROR_UNIMPLEMENTED: i32 = ERROR_INVALID_HANDLE + 2;

/// A resource or information is unavailable.
/// This could occur when an application tries to open too many streams,
/// or a timestamp is not available.
pub const ERROR_UNAVAILABLE: i32 = ERROR_UNIMPLEMENTED + 1;
pub const ERROR_NO_FREE_HANDLES: i32 = ERROR_UNIMPLEMENTED + 2;

/// Memory could not be allocated.
pub const ERROR_NO_MEMORY: i32 = ERROR_UNIMPLEMENTED + 3;

/// A NULL pointer was passed to AAudio.
/// Or a NULL pointer was detected internally.
pub const ERROR_NULL: i32 = ERROR_UNIMPLEMENTED + 4;

/// An operation took longer than expected.
pub const ERROR_TIMEOUT: i32 = ERROR_UNIMPLEMENTED + 5;
pub const ERROR_WOULD_BLOCK: i32 = ERROR_UNIMPLEMENTED + 6;

/// The requested data format is not supported.
pub const ERROR_INVALID_FORMAT: i32 = ERROR_UNIMPLEMENTED + 7;

/// A requested was out of range.
pub const ERROR_OUT_OF_RANGE: i32 = ERROR_UNIMPLEMENTED + 8;

/// The audio service was not available.
pub const ERROR_NO_SERVICE: i32 = ERROR_UNIMPLEMENTED + 9;

/// The requested sample rate was not supported.
pub const ERROR_INVALID_RATE: i32 = ERROR_UNIMPLEMENTED + 10;

/// Prototype for the data function that is passed to AAudioStreamBuilder_setDataCallback().
///
/// For an output stream, this function should render and write numFrames of data
/// in the streams current data format to the audioData buffer.
///
/// For an input stream, this function should read and process numFrames of data
/// from the audioData buffer.
///
/// The audio data is passed through the buffer. So do NOT call AAudioStream_read() or
/// AAudioStream_write() on the stream that is making the callback.
///
/// Note that numFrames can vary unless AAudioStreamBuilder_setFramesPerDataCallback()
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
/// * AAudioStream_read()
/// * AAudioStream_write()
///
/// The following are OK to call from the data callback:
/// * `AAudioStream_get*()`
/// * AAudio_convertResultToText()
///
/// If you need to move data, eg. MIDI commands, in or out of the callback function then
/// we recommend the use of non-blocking techniques such as an atomic FIFO.
///
/// * `stream` - reference provided by AAudioStreamBuilder_openStream()
/// * `user_data` - the same address that was passed to AAudioStreamBuilder_setCallback()
/// * `audio_data` - a pointer to the audio data
/// * `num_frames` - the number of frames to be processed, which can vary
///
/// Returns CallbackResult (Continue = 0, Stop = 1)
pub type DataCallback = Option<
    unsafe extern "C" fn(
        stream: *mut AAudioStream,
        user_data: *mut c_void,
        audio_data: *mut c_void,
        num_frames: i32,
    ) -> i32,
>;

/// Prototype for the callback function that is passed to
/// AAudioStreamBuilder_setErrorCallback().
///
/// The following may NOT be called from the error callback:
/// * AAudioStream_requestStop()
/// * AAudioStream_requestPause()
/// * AAudioStream_close()
/// * AAudioStream_waitForStateChange()
/// * AAudioStream_read()
/// * AAudioStream_write()
///
/// The following are OK to call from the error callback:
/// * AAudioStream_get*()
/// * AAudio_convertResultToText()
///
/// * `stream` - reference provided by AAudioStreamBuilder_openStream()
/// * `user_data` - the same address that was passed to AAudioStreamBuilder_setErrorCallback()
/// * `error` - an AAUDIO_ERROR_* value.
pub type ErrorCallback =
    Option<unsafe extern "C" fn(stream: *mut AAudioStream, user_data: *mut c_void, error: i32)>;

#[link(name = "aaudio")]
extern "C" {
    /// Create a StreamBuilder that can be used to open a Stream.
    ///
    /// The deviceId is initially unspecified, meaning that the current default device will be used.
    ///
    /// The default direction is `DIRECTION_OUTPUT`.
    /// The default sharing mode is `SHARING_MODE_SHARED`.
    /// The data format, samplesPerFrames and sampleRate are unspecified and will be
    /// chosen by the device when it is opened.
    ///
    /// AAudioStreamBuilder_delete() must be called when you are done using the builder.
    ///
    /// Available since API level 26.
    pub fn AAudio_createStreamBuilder(builder: *mut *mut AAudioStreamBuilder) -> i32;

    /// Request an audio device identified device using an ID.
    /// On Android, for example, the ID could be obtained from the Java AudioManager.
    ///
    /// The default, if you do not call this function, is `UNSPECIFIED`,
    /// in which case the primary device will be used.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `device_id` - device identifier or `UNSPECIFIED`
    pub fn AAudioStreamBuilder_setDeviceId(builder: *mut AAudioStreamBuilder, device_id: i32);

    /// Request a sample rate in Hertz.
    ///
    /// The default, if you do not call this function, is `UNSPECIFIED`.
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
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `sample_rate` - frames per second. Common rates include 44100 and 48000 Hz.
    pub fn AAudioStreamBuilder_setSampleRate(builder: *mut AAudioStreamBuilder, sample_rate: i32);

    /// Request a number of channels for the stream.
    ///
    /// The default, if you do not call this function, is AAUDIO_UNSPECIFIED.
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
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `channel_count` - Number of channels desired.
    pub fn AAudioStreamBuilder_setChannelCount(
        builder: *mut AAudioStreamBuilder,
        channel_count: i32,
    );

    /// Request a sample data format, for example `FORMAT_PCM_I16`.
    ///
    /// The default, if you do not call this function, is `UNSPECIFIED`.
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
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `format` - the sample data format.
    pub fn AAudioStreamBuilder_setFormat(builder: *mut AAudioStreamBuilder, format: i32);

    /// Request a mode for sharing the device.
    ///
    /// The default, if you do not call this function, is `SHARING_MODE_SHARED`.
    ///
    /// The requested sharing mode may not be available.
    /// The application can query for the actual mode after the stream is opened.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `sharing_mode` - `SHARING_MODE_SHARED` or `SHARING_MODE_EXCLUSIVE`
    pub fn AAudioStreamBuilder_setSharingMode(builder: *mut AAudioStreamBuilder, sharing_mode: i32);

    /// Request the direction for a stream.
    ///
    /// The default, if you do not call this function, is Output.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `direction` - `DIRECTION_OUTPUT` or `DIRECTION_INPUT`
    pub fn AAudioStreamBuilder_setDirection(builder: *mut AAudioStreamBuilder, direction: i32);

    /// Set the requested buffer capacity in frames.
    /// The final AAudioStream capacity may differ, but will probably be at least this big.
    ///
    /// The default, if you do not call this function, is `UNSPECIFIED`.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `numFrames` - the desired buffer capacity in frames or `UNSPECIFIED`
    pub fn AAudioStreamBuilder_setBufferCapacityInFrames(
        builder: *mut AAudioStreamBuilder,
        num_frames: i32,
    );

    /// Set the requested performance mode.
    ///
    /// Supported modes are None, PowerSaving and LowLatency.
    ///
    /// The default, if you do not call this function, is None.
    ///
    /// You may not get the mode you requested.
    /// You can call AAudioStream_getPerformanceMode()
    /// to find out the final mode for the stream.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `mode` - the desired performance mode, eg. LowLatency
    pub fn AAudioStreamBuilder_setPerformanceMode(builder: *mut AAudioStreamBuilder, mode: i32);

    /// Set the intended use case for the stream.
    ///
    /// The AAudio system will use this information to optimize the
    /// behavior of the stream.
    /// This could, for example, affect how volume and focus is handled for the stream.
    ///
    /// The default, if you do not call this function, is Media.
    ///
    /// Available since API level 28.
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `usage` - the desired usage, eg. Game
    pub fn AAudioStreamBuilder_setUsage(builder: *mut AAudioStreamBuilder, usage: i32);

    /// Set the type of audio data that the stream will carry.
    ///
    /// The AAudio system will use this information to optimize the
    /// behavior of the stream.
    /// This could, for example, affect whether a stream is paused when a notification occurs.
    ///
    /// The default, if you do not call this function, is Music.
    ///
    /// Available since API level 28.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `content_type` - the type of audio data, eg. Speech
    pub fn AAudioStreamBuilder_setContentType(builder: *mut AAudioStreamBuilder, content_type: i32);

    /// Set the input (capture) preset for the stream.
    ///
    /// The AAudio system will use this information to optimize the
    /// behavior of the stream.
    /// This could, for example, affect which microphones are used and how the
    /// recorded data is processed.
    ///
    /// The default, if you do not call this function, is VoiceRecognition.
    /// That is because VoiceRecognition is the preset with the lowest latency
    /// on many platforms.
    ///
    /// Available since API level 28.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `input_preset` - the desired configuration for recording
    pub fn AAudioStreamBuilder_setInputPreset(builder: *mut AAudioStreamBuilder, input_preset: i32);

    /// Specify whether this stream audio may or may not be captured by other apps or the system.
    ///
    /// The default is AllowCaptureByAll.
    ///
    /// Note that an application can also set its global policy, in which case the most restrictive
    /// policy is always applied. See android.media.AudioAttributes#setAllowedCapturePolicy(int)
    ///
    /// Available since API level 29.
    ///
    /// `builder` - reference provided by AAudio_createStreamBuilder()
    /// `capturePolicy` - the desired level of opt-out from being captured.
    pub fn AAudioStreamBuilder_setAllowedCapturePolicy(
        builder: *mut AAudioStreamBuilder,
        capture_policy: i32,
    );

    /// The session ID can be used to associate a stream with effects processors.
    /// The effects are controlled using the Android AudioEffect Java API.
    ///
    /// The default, if you do not call this function, is `SESSION_ID_NONE`.
    ///
    /// If set to `SESSION_ID_ALLOCATE` then a session ID will be allocated
    /// when the stream is opened.
    ///
    /// The allocated session ID can be obtained by calling AAudioStream_getSessionId()
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
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `session_id` - an allocated sessionID or `SESSION_ID_ALLOCATE`
    pub fn AAudioStreamBuilder_setSessionId(builder: *mut AAudioStreamBuilder, session_id: i32);

    /// Indicates whether this input stream must be marked as privacy sensitive or not.
    ///
    /// When true, this input stream is privacy sensitive and any concurrent capture
    /// is not permitted.
    ///
    /// This is off (false) by default except when the input preset is `VoiceCommunication`
    /// or `Camcorder`.
    ///
    /// Always takes precedence over default from input preset when set explicitly.
    ///
    /// Only relevant if the stream direction is `Input`.
    ///
    /// Added in API level 30.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `privacy_sensitive` - true if capture from this stream must be marked as privacy sensitive, false otherwise.
    pub fn AAudioStreamBuilder_setPrivacySensitive(
        builder: *mut AAudioStreamBuilder,
        privacy_sensitive: bool,
    );

    /// Request that AAudio call this functions when the stream is running.
    ///
    /// Note that when using this callback, the audio data will be passed in or out
    /// of the function as an argument.
    /// So you cannot call AAudioStream_write() or AAudioStream_read()
    /// on the same stream that has an active data callback.
    ///
    /// The callback function will start being called after AAudioStream_requestStart()
    /// is called.
    /// It will stop being called after AAudioStream_requestPause() or
    /// AAudioStream_requestStop() is called.
    ///
    /// This callback function will be called on a real-time thread owned by AAudio. See
    /// `DataCallback` for more information.
    ///
    /// Note that the AAudio callbacks will never be called simultaneously from multiple threads.
    ///
    /// Available since API level 26.
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `callback` - pointer to a function that will process audio data.
    /// * `user_data` - pointer to an application data structure that will be passed
    ///          to the callback functions.
    pub fn AAudioStreamBuilder_setDataCallback(
        builder: *mut AAudioStreamBuilder,
        callback: DataCallback,
        user_data: *mut c_void,
    );

    /// Set the requested data callback buffer size in frames.
    /// See `DataCallback`.
    ///
    /// The default, if you do not call this function, is `Unspecified`.
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
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `num_frames` - the desired buffer size in frames or `Unspecified`
    pub fn AAudioStreamBuilder_setFramesPerDataCallback(
        builder: *mut AAudioStreamBuilder,
        num_frames: i32,
    );

    /// Request that AAudio call this function if any error occurs or the stream is disconnected.
    ///
    /// It will be called, for example, if a headset or a USB device is unplugged causing the stream's
    /// device to be unavailable or "disconnected".
    /// Another possible cause of error would be a timeout or an unanticipated internal error.
    ///
    /// In response, this function should signal or create another thread to stop
    /// and close this stream. The other thread could then reopen a stream on another device.
    /// Do not stop or close the stream, or reopen the new stream, directly from this callback.
    ///
    /// This callback will not be called because of actions by the application, such as stopping
    /// or closing a stream.
    ///
    /// Note that the AAudio callbacks will never be called simultaneously from multiple threads.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `callback` - pointer to a function that will be called if an error occurs.
    /// * `user_data` - pointer to an application data structure that will be passed
    ///          to the callback functions.
    pub fn AAudioStreamBuilder_setErrorCallback(
        builder: *mut AAudioStreamBuilder,
        callback: ErrorCallback,
        user_data: *mut c_void,
    );

    /// Open a stream based on the options in the StreamBuilder.
    /// Returns 0 for OK or a negative error.
    ///
    /// AAudioStream_close() must be called when finished with the stream to recover
    /// the memory and to free the associated resources.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// * `stream` - pointer to a variable to receive the new stream reference
    pub fn AAudioStreamBuilder_openStream(
        builder: *mut AAudioStreamBuilder,
        stream: *mut *mut AAudioStream,
    ) -> i32;

    /// Delete the resources associated with the StreamBuilder.
    /// Returns 0 for OK or a negative error.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `builder` - reference provided by AAudio_createStreamBuilder()
    /// @return 0 for OK or a negative error.
    pub fn AAudioStreamBuilder_delete(builder: *mut AAudioStreamBuilder) -> i32;

    /// Free the audio resources associated with a stream created by
    /// AAudioStreamBuilder_openStream().
    /// AAudioStream_close() should be called at some point after calling
    /// this function.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// After this call, the stream will be in Closing state
    ///
    /// This function is useful if you want to release the audio resources immediately,
    /// but still allow queries to the stream to occur from other threads. This often
    /// happens if you are monitoring stream progress from a UI thread.
    ///
    /// Available since API level 30.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_release(stream: *mut AAudioStream) -> i32;

    /// Delete the internal data structures associated with the stream created
    /// by AAudioStreamBuilder_openStream().
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// If AAudioStream_release() has not been called then it will be called automatically.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_close(stream: *mut AAudioStream) -> i32;

    /// Asynchronously request to start playing the stream. For output streams, one should
    /// write to the stream to fill the buffer before starting.
    /// Otherwise it will underflow.
    /// After this call the state will be in `Starting` or `Started`.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_requestStart(stream: *mut AAudioStream) -> i32;

    /// Asynchronous request for the stream to pause.
    /// Pausing a stream will freeze the data flow but not flush any buffers.
    /// Use AAudioStream_requestStart() to resume playback after a pause.
    /// After this call the state will be in `Pausing` or
    /// `Paused`.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// This will return `Unimplemented` for input streams.
    /// For input streams use AAudioStream_requestStop().
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_requestPause(stream: *mut AAudioStream) -> i32;

    /// Asynchronous request for the stream to flush.
    /// Flushing will discard any pending data.
    /// This call only works if the stream is pausing or paused.
    /// Frame counters are not reset by a flush. They may be advanced.
    /// After this call the state will be in `Flushing` or `Flushed`.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// This will return `Unimplemented` for input streams.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_requestFlush(stream: *mut AAudioStream) -> i32;

    /// Asynchronous request for the stream to stop.
    /// The stream will stop after all of the data currently buffered has been played.
    /// After this call the state will be in `Stopping` or `Stopped`.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// Available since API level 26.
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_requestStop(stream: *mut AAudioStream) -> i32;

    /// Query the current state of the client, eg. `Pausing`.
    ///
    /// This function will immediately return the state without updating the state.
    /// If you want to update the client state based on the server state then
    /// call AAudioStream_waitForStateChange() with currentState
    /// set to `Unknown` and a zero timeout.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getState(stream: *mut AAudioStream) -> i32;

    /// Wait until the current state no longer matches the input state.
    ///
    /// Returns 0 for OK or a negative error.
    ///
    /// This will update the current client state.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - A reference provided by AAudioStreamBuilder_openStream()
    /// * `input_state` - The state we want to avoid.
    /// * `next_state` - Pointer to a variable that will be set to the new state.
    /// * `timeout_nanoseconds` - Maximum number of nanoseconds to wait for completion.
    pub fn AAudioStream_waitForStateChange(
        stream: *mut AAudioStream,
        input_state: i32,
        next_state: *mut i32,
        timeout_nanoseconds: i64,
    ) -> i32;

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
    /// * `stream` - A stream created using AAudioStreamBuilder_openStream().
    /// * `buffer` - The address of the first sample.
    /// * `num_frames` - Number of frames to read. Only complete frames will be written.
    /// * `timeout_nanoseconds` - Maximum number of nanoseconds to wait for completion.
    pub fn AAudioStream_read(
        stream: *mut AAudioStream,
        buffer: *mut c_void,
        num_frames: i32,
        timeout_nanoseconds: i64,
    ) -> i32;

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
    /// * `stream` - A stream created using AAudioStreamBuilder_openStream().
    /// * `buffer` - The address of the first sample.
    /// * `num_frames` - Number of frames to write. Only complete frames will be written.
    /// * `timeout_nanoseconds` - Maximum number of nanoseconds to wait for completion.
    pub fn AAudioStream_write(
        stream: *mut AAudioStream,
        buffer: *const c_void,
        num_frames: i32,
        timeout_nanoseconds: i64,
    ) -> i32;

    /// This can be used to adjust the latency of the buffer by changing
    /// the threshold where blocking will occur.
    /// By combining this with AAudioStream_getXRunCount(), the latency can be tuned
    /// at run-time for each device.
    /// Returns actual buffer size in frames or a negative error.
    ///
    /// This cannot be set higher than AAudioStream_getBufferCapacityInFrames().
    ///
    /// Note that you will probably not get the exact size you request.
    /// You can check the return value or call AAudioStream_getBufferSizeInFrames()
    /// to see what the actual final size is.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    /// * `num_frames` - requested number of frames that can be filled without blocking
    pub fn AAudioStream_setBufferSizeInFrames(stream: *mut AAudioStream, num_frames: i32) -> i32;

    /// Query the maximum number of frames that can be filled without blocking.
    ///
    /// Available since API level 26.
    ///
    /// Stream reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getBufferSizeInFrames(stream: *mut AAudioStream) -> i32;

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
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getFramesPerBurst(stream: *mut AAudioStream) -> i32;

    /// Query maximum buffer capacity in frames.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// @param stream reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getBufferCapacityInFrames(stream: *mut AAudioStream) -> i32;

    /// Query the size of the buffer that will be passed to the dataProc callback
    /// in the numFrames parameter.
    ///
    /// This call can be used if the application needs to know the value of numFrames before
    /// the stream is started. This is not normally necessary.
    ///
    /// If a specific size was requested by calling
    /// AAudioStreamBuilder_setFramesPerDataCallback() then this will be the same size.
    ///
    /// If AAudioStreamBuilder_setFramesPerDataCallback() was not called then this will
    /// return the size chosen by AAudio, or `UNSPECIFIED`.
    ///
    /// `UNSPECIFIED` indicates that the callback buffer size for this stream
    /// may vary from one dataProc callback to the next.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getFramesPerDataCallback(stream: *mut AAudioStream) -> i32;

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
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getXRunCount(stream: *mut AAudioStream) -> i32;

    /// Available since API level 26.
    /// Returns the actual sample rate.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getSampleRate(stream: *mut AAudioStream) -> i32;

    /// A stream has one or more channels of data.
    /// A frame will contain one sample for each channel.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getChannelCount(stream: *mut AAudioStream) -> i32;

    /// Available since API level 26.
    /// Returns the actual device ID.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getDeviceId(stream: *mut AAudioStream) -> i32;

    /// Available since API level 26.
    /// Returns the actual data format.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getFormat(stream: *mut AAudioStream) -> i32;

    /// Provide actual sharing mode.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getSharingMode(stream: *mut AAudioStream) -> i32;

    /// Get the performance mode used by the stream.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getPerformanceMode(stream: *mut AAudioStream) -> i32;

    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getDirection(stream: *mut AAudioStream) -> i32;

    /// Passes back the number of frames that have been written since the stream was created.
    /// For an output stream, this will be advanced by the application calling write()
    /// or by a data callback.
    /// For an input stream, this will be advanced by the endpoint.
    ///
    /// The frame position is monotonically increasing.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getFramesWritten(stream: *mut AAudioStream) -> i64;

    /// Passes back the number of frames that have been read since the stream was created.
    /// For an output stream, this will be advanced by the endpoint.
    /// For an input stream, this will be advanced by the application calling read()
    /// or by a data callback.
    ///
    /// The frame position is monotonically increasing.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getFramesRead(stream: *mut AAudioStream) -> i64;

    /// Passes back the session ID associated with this stream.
    ///
    /// The session ID can be used to associate a stream with effects processors.
    /// The effects are controlled using the Android AudioEffect Java API.
    ///
    /// If AAudioStreamBuilder_setSessionId() was called with `SESSION_ID_ALLOCATE`
    /// then a new session ID should be allocated once when the stream is opened.
    ///
    /// If AAudioStreamBuilder_setSessionId() was called with a previously allocated
    /// session ID then that value should be returned.
    ///
    /// If AAudioStreamBuilder_setSessionId() was not called then this function should
    /// return `SESSION_ID_NONE`.
    ///
    /// The sessionID for a stream should not change once the stream has been opened.
    ///
    /// Available since API level 28.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    pub fn AAudioStream_getSessionId(stream: *mut AAudioStream) -> i32;

    /// Passes back the time at which a particular frame was presented.
    /// This can be used to synchronize audio with video or MIDI.
    /// It can also be used to align a recorded stream with a playback stream.
    /// Returns 0 for OK or a negative error.
    ///
    /// Timestamps are only valid when the stream is in `STREAM_STATE_STARTED`.
    /// `ERROR_INVALID_STATE` will be returned if the stream is not started.
    /// Note that because requestStart() is asynchronous, timestamps will not be valid until
    /// a short time after calling requestStart().
    /// So `ERROR_INVALID_STATE` should not be considered a fatal error.
    /// Just try calling again later.
    ///
    /// If an error occurs, then the position and time will not be modified.
    ///
    /// The position and time passed back are monotonically increasing.
    ///
    /// Available since API level 26.
    ///
    /// # Arguments
    ///
    /// * `stream` - reference provided by AAudioStreamBuilder_openStream()
    /// * `clockid` - CLOCK_MONOTONIC or CLOCK_BOOTTIME
    /// * `frame_position` - pointer to a variable to receive the position
    /// * `time_nanoseconds` - pointer to a variable to receive the time
    pub fn AAudioStream_getTimestamp(
        stream: *mut AAudioStream,
        clockid: libc::clockid_t,
        frame_positon: *mut i64,
        time_nanoseconds: *mut i64,
    ) -> i32;
}
