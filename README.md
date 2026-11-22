# CLI CPE Encoder/Decoder tools v4.3

## What ?
`CPE` for **C**olor **P**ixel **E**ncoding is a algorithm who convert ascii text to PNG Pixel fully coded in **Rust**

<br>

## Uage 
```bash
# compile code from GitHub
git clone https://github.com/Perrtatter/CPE-Encoder.git cpe_src
cd  cpe_src
cargo build 

## /!\ argument order is crucial ! /!\
## encode
./main --encode data.txt --ouput encode.png
[*] File found: data.txt
[*] Tokenize ...
[*] Convert ...
[*] Write PNG ...

[*] Encode saved: encode.png

## decode
./main --decode encode.png --ouput data.txt
[*] File found: encode.png
[*] Read pixels ...
[*] Convert ...
[*] Untokenize ...
[*] Write TXT ...

[*] Encode saved: data.txt
```

<br>

## How it work 
This encoding algorithm is very simple ( exemple : )

[![](https://mermaid.ink/img/pako:eNo9jUtrwkAUhf_K5a4UJpLJY5LMomAidSdSCoUaF6O5ajCZkXHS-oj_vahtl-dwvu9ccW0qQombxnyvd8o6eJ-UGgBgPBjnxRA87wX6d7Mnfawv1EM-WIwZ5AyK5fA5zJ-bwugvsg6cgbdpzmB1htXZ0bGHYiBiBkIwEMkvU9yZ_sPWjmA-m_YwWcxnU3itG1oiw62tK5TOdsSwJduqe8TrnS3R7ailEiWUaKnqTl6l7N5bm8bYEkt9Q4YHpT-Naf8U1nTbHcqNao7EsDtUytGkVlur2v_Wkq7IFqbTDmWUBg8JyiueUPIsGQVRFkaZ70c8FiJieEbJg2QUchEmXAR-kKXhjeHlceuP4iT2wzTkYSpiIbi4_QDKZ2pQ?type=png)](https://mermaid.live/edit#pako:eNo9jUtrwkAUhf_K5a4UJpJkkomZRcFE6k6kCIUaF6O5ajCZkXHS-oj_vfhol-dwvu9ccW1KQomb2vysd8o6mI8LDQAw6o2yvA-e9wbd3OxJH6sLdZD1FiMGGYN82X8Os-cmN_qbrANn4GOSMVidYXXuIO-JmIEQDETyAvI70H3ayhHMppMOxovZdALvVU1LZLi1VYnS2ZYYNmQbdY94vbMFuh01VKCEAi2V7ckrld17a1MbW2Chb8jwoPSXMc2fwpp2u0O5UfWRGLaHUjkaV2prVfPfWtIl2dy02qGMEv8hQXnFE8ogTQZhlPIo9f0oiIWIGJ5RBmEy4IHgSSBCP0yH_Mbw8rj1B3ES-3zIAz4UsRCBuP0CHYRpAQ) 

<br>

## Resume
It encode text in RGB to write PNG

<br>

## Try your self
You can try your self:

```bash
# encode
./main --encode demo/data.txt --ouput encode.png

# decode 
./main --decode demo/encode.png --output data.txt
```
(download executable file in release)