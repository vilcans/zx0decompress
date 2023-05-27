# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.1 (2023-05-27)

<csr-id-00da0a307c1a9fa005de436dda3dde59747c4e1f/>
<csr-id-00e82a586345e5a15aa9cfe70f00bd60bd60d0bd/>

### Other

 - <csr-id-00da0a307c1a9fa005de436dda3dde59747c4e1f/> start with Vec::reserve

### Other

 - <csr-id-00e82a586345e5a15aa9cfe70f00bd60bd60d0bd/> Correct repository link in Cargo.toml

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 3 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add CHANGELOG.md ([`f9f5747`](https://github.com/vilcans/zx0decompress/commit/f9f5747c73be0a2e0fb663359a51d29e96ee5c00))
    - Correct repository link in Cargo.toml ([`00e82a5`](https://github.com/vilcans/zx0decompress/commit/00e82a586345e5a15aa9cfe70f00bd60bd60d0bd))
    - Split readme into lib and cli ([`5430598`](https://github.com/vilcans/zx0decompress/commit/54305987e63b3d0cf88d4008b5b16f3812c1577b))
    - One single error for all kinds of corrupt input ([`f6dac11`](https://github.com/vilcans/zx0decompress/commit/f6dac11e1222a9c1e8ba74bf170c5d9926a57c74))
    - Fail with TruncatedInput if input stream ends ([`48f803e`](https://github.com/vilcans/zx0decompress/commit/48f803ebf7b13d09344ad3362757e9d29f0c9442))
    - Heed max_output_size when copying literals ([`ac77db6`](https://github.com/vilcans/zx0decompress/commit/ac77db613df0464876749b40da285697a6ba65d9))
    - Check for more errors caused by invalid input ([`f5d2e9a`](https://github.com/vilcans/zx0decompress/commit/f5d2e9a775682b44aa9444431ce6d6994dfaa716))
    - Use end of data bit instead of bit counter ([`f897b9d`](https://github.com/vilcans/zx0decompress/commit/f897b9d3559636d04a995efdfc2ff28bcb17a8d2))
    - Add settings for classic mode and max output size ([`8bd4ccd`](https://github.com/vilcans/zx0decompress/commit/8bd4ccd2487ff977a2adc88109c6f306462e1866))
    - Add max_output_size, hardcoded to 128 K ([`dd385be`](https://github.com/vilcans/zx0decompress/commit/dd385be4b06f045e20efa03ff6ddd6df6321357d))
    - Error on corrupt input (invalid length) ([`f5f7308`](https://github.com/vilcans/zx0decompress/commit/f5f7308634b6b9e461e7f149c8674c7e77062b6c))
    - Start with Vec::reserve ([`00da0a3`](https://github.com/vilcans/zx0decompress/commit/00da0a307c1a9fa005de436dda3dde59747c4e1f))
    - Comments ([`7069494`](https://github.com/vilcans/zx0decompress/commit/7069494d266c95c7bf88f73ed1793a0169ebaedc))
    - Add metadata to Cargo files ([`4ff5199`](https://github.com/vilcans/zx0decompress/commit/4ff51997ce156fa57149e9fac0a743e6d5b0bf7f))
    - Remove "backtracking", instead modify upcoming bits ([`b684ba3`](https://github.com/vilcans/zx0decompress/commit/b684ba3b4928235106cf51e5d1c589cea0c9378a))
    - First implementation ([`f87de06`](https://github.com/vilcans/zx0decompress/commit/f87de0667650cb6ab69f8acc338154564f4884d5))
</details>

