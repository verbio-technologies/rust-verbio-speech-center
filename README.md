# Rust Verbio SpeechCenter Client

[![Lint](https://github.com/cquintana92/rust-verbio-speech-center/actions/workflows/lint.yaml/badge.svg)](https://github.com/cquintana92/rust-verbio-speech-center/actions/workflows/lint.yaml)

## How to build

In order to build the system

```
$ cargo build --release --all
```

It will build two binaries: `batch-client` and `cli-client`.

## How to use

### CLI client

The CLI client allows you to launch a single file to the server. It also allows you to use either a grammar or a language model.

```
λ ./target/release/cli-client --help
cli-client 0.1.0

USAGE:
    cli-client [OPTIONS] --audio <audio> --language <language> --token-file <token-file> --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --audio <audio>              Path to a .wav audio in 8kHz and PCM16 encoding to use for the recognition
    -g, --grammar <grammar>          Path to the ABNF grammar file to use for the recognition
    -l, --language <language>        Language to use for the recognition [default: en-US]
    -t, --token-file <token-file>    Path to the authentication token file
    -T, --topic <topic>              Topic to use for the recognition. Must be GENERIC | BANKING | TELCO
    -u, --url <url>                  The URL of the host or server trying to reach [default: https://speechcenter.verbio.com:2424]
```

An example execution could be:

```
λ ./target/debug/cli-client -a example.wav -l en-US -t my.token -T generic
```


### Batch client

The batch client iterates over wav files inside a directory, sends them in parallel to the server and stores the transcription in another directory.

```
λ ./target/release/batch-client --help
batch-client 0.1.0

USAGE:
    batch-client [OPTIONS] --dest-dir <dest-dir> --language <language> --dir <source-dir> --token-file <token-file> --topic <topic> --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -D, --dest-dir <dest-dir>        Destination directory for the transcriptions
    -l, --language <language>        Language to use for the recognition [default: en-US]
    -L, --log-level <log-level>      Log level. Must be TRACE | DEBUG | INFO | WARN | ERROR [default: info]
    -d, --dir <source-dir>           Directory containing .wav audios in 8kHz and PCM16 encoding to use for the recognition
    -t, --token-file <token-file>    Path to the authentication token file
    -T, --topic <topic>              Topic to use for the recognition. Must be GENERIC | BANKING | TELCO
    -u, --url <url>                  The URL of the host or server trying to reach [default: https://speechcenter.verbio.com:2424]
    -w, --workers <workers>          Number of workers to use for the recognition [default: 4]
```

An example execution could be:

```
λ ./target/release/batch-client -w 4 -d ~/tmp/commonvoice/clips -D /tmp/results -t my.token -T generic --log-level debug
```

## Improvements to be done

Right now, the build process generates the `speech-center-client/src/csr_grpc_gateway.rs` file. However, the `tonic-build` is able to generate it into the target directory and include it via macros. Unfortunately, my IDE was not able to detect the file, so autocomplete didn't work and I prioritized developer ergonomy over "correctness" for this PoC (sorry guys).

In order to change it, edit the `build.rs` and see the documentation of [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).

