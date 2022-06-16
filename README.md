# Rust integration with the Verbio Speech Center cloud.

This repository contains a Rust based example of how to use the Verbio Technologies Speech Center cloud.

[![Build Status](https://github.com/verbio-technologies/rust-verbio-speech-center/actions/workflows/ci.yaml/badge.svg)](https://github.com/verbio-technologies/rust-verbio-speech-center/actions/workflows/ci.yaml)

[Website](https://speechcenter.verbio.com) |
[Guides](https://github.com/verbio-technologies) |
[API Docs](https://speechcenter.verbio.com/documentation/)

## How to build

In order to build the system

```
$ cargo build --release --all
```

It will build two binaries: `batch-client` and `cli-client`.

## How to use

### CLI client

The CLI client integrates two sub-commands:
* Recognition: Speech-to-Text operation, using either an ABNF Grammar or a topic for an out-of-the-box statistical model.
* Synthesis: Text-to-Speech operation with customizable audio options.

You can use the `--help` command to find out more about the client.

```
λ ./target/release/cli-client --help
Speech-Center 0.1.0
Verbio Technologies S.L.

USAGE:
    cli-client <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help           Prints this message or the help of the given subcommand(s)
    recognition    Run a Speech Center gRPC recognition client
    synthesis      Run a Speech Center gRPC synthesis client
```


#### CLI client recognition

The CLI client recognition allows you to transcribe a single audio file. To do so, it will require either an ABNF grammar or an out-of-the-box recognition topic such as: `GENERIC`, as of general discussion; `BANKING`, concerning financial-related speech; or `TELCO`, including telecommunications and technology-centered talks.

If you wish to know more about our [Recognition Topics](https://speechcenter.verbio.com/documentation/topics) or [ABNF Grammars](https://speechcenter.verbio.com/documentation/abnf), please check out our documentation.

```
λ ./target/release/cli-client recognition --help
cli-client-recognition 0.1.0

USAGE:
    cli-client recognition [OPTIONS] --audio <audio> --language <language> --token-file <token-file> --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --audio <audio>              Path to a .wav audio in 8kHz and PCM16 encoding to use for the recognition
    -g, --grammar <grammar>          Path to the ABNF grammar file to use for the recognition
    -l, --language <language>        IETF BCP-47 Language to use for the recognition. Supported en-US | es-ES | pt-BR [default: en-US]
    -t, --token-file <token-file>    Path to the JWT authentication token file
    -T, --topic <topic>              Topic to use for the recognition. Must be GENERIC | BANKING | TELCO
    -u, --url <url>                  The URL of the gRPC host or server trying to reach [default: https://csr.api.speechcenter.verbio.com]
```

An example execution could be:

```
λ ./target/debug/cli-client recognition -a example.wav -l en-US -t my.token -T generic
```


#### CLI client synthesis

The CLI client synthesis grants you the ability to create customizable speech from a mere text sentence. You simply need to specify a target text sentence, a destination output file to store the resulting audio, the voice or speaker, the language and some optional parameters such as the speech encoding, header or the sample rate.

If you wish to know more about certain parameters such as [Audio Encoding](https://en.wikipedia.org/wiki/Audio_codec), [Audio Header or Format](https://en.wikipedia.org/wiki/Audio_file_format), or [Sample Rate](https://en.wikipedia.org/wiki/Sampling_(signal_processing)#Audio_sampling), you can take a look at one of the links.

**_Note_**: Regarding Voice - Language combinations, not all of them are possible or available. Find below a table on the supported combinations:

<div align="center">

|Language<br>-<br>Voice|en-US|es-ES|pt-BR|ca-CA|
|:--------------------:|:---:|:---:|:---:|:---:|
|         Tommy        |**X**|     |     |     |
|         Annie        |**X**|     |     |     |
|         David        |     |**X**|     |**X**|
|         Aurora       |     |**X**|     |     |
|         Luma         |     |     |**X**|     |

</div>
  
  
```
λ ./target/release/cli-client synthesis --help
cli-client-synthesis 0.1.0
Run a Speech Center gRPC synthesis client

USAGE:
    cli-client synthesis --encoding <encoding> --header <header> --language <language> --output <output> --sample-rate <sample-rate> --text <text> --token-file <token-file> --url <url> --voice <voice>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --encoding <encoding>          Output audio encoding algorithm. Supported PCM (Signed 16-bit little endian PCM)
                                       [default: PCM]
    -h, --header <header>              Output audio header. Supported: WAV (Wav audio header) | RAW (No header)
                                       [default: WAV]
    -l, --language <language>          IETF BCP-47 Language to use for the recognition. Supported en-US | es-ES | pt-BR
                                       | ca-CA [default: en-US]
    -o, --output <output>              Path to store the synthesis resulting audio
    -s, --sample-rate <sample-rate>    Output audio sample rate in Hz. Available 8000 [default: 8000]
    -T, --text <text>                  Text to synthesize to audio
    -t, --token-file <token-file>      Path to the JWT authentication token file
    -u, --url <url>                    The URL of the gRPC host or server trying to reach [default:
                                       https://tts.api.speechcenter.verbio.com]
    -v, --voice <voice>                Voice to use for the synthesis. Supported Tommy | Annie | Aurora | Luma | David
```

An example execution could be:

```
λ ./target/debug/cli-client synthesis --text "Hello World" --voice Tommy -l en-US -t my.token -o example_tommy_en-US.wav
```


### Batch client (Recognition Only)

The batch client iterates over wav files inside a directory, sends them in parallel to the server and stores the transcription in the specified folder.

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
    -l, --language <language>        IETF BCP-47 Language to use for the recognition. Supported en-US | es-ES | pt-BR [default: en-US]
    -L, --log-level <log-level>      Log level. Must be TRACE | DEBUG | INFO | WARN | ERROR [default: info]
    -d, --dir <source-dir>           Directory containing .wav audios in 8kHz and PCM16 encoding to use for the recognition
    -t, --token-file <token-file>    Path to the JWT authentication token file
    -T, --topic <topic>              Topic to use for the recognition. Must be GENERIC | BANKING | TELCO
    -u, --url <url>                  The URL of the gRPC  host or server trying to reach [default: https://csr.api.speechcenter.verbio.com]
    -w, --workers <workers>          Number of workers to use for the recognition [default: 4]
```

An example execution could be:

```
λ ./target/release/batch-client -w 4 -d ~/tmp/commonvoice/clips -D /tmp/results -t my.token -T generic --log-level debug
```
