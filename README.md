# Minios

This is a small operative system made with Rust. It was created to experiment with the Rust programming language and to learn about operating systems. It is not intended to be a fully functional operating system, but rather a small project to learn from.

Is heavily inspired by [Writing an OS in Rust](https://os.phil-opp.com/).

## How to build
1. Install dependencies; Rust, QEMU, and a few other tools.
2. Clone the repository.
3. Install bootloader dependencies with `cargo install bootimage`.
4. Build the project with `cargo bootimage`.
5. Run the project with qemu with `qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-mini_os.bin`.

## Known issues
1. https://github.com/phil-opp/blog_os/issues/1249#issuecomment-1819116460
2. https://github.com/rust-osdev/bootloader/issues/499#issuecomment-2741496549


## Youtube video
This project is part of a video on my Youtube channel. The videos are in Spanish, but you can enable subtitles in English.

[![Youtube video](https://img.youtube.com/vi/r7phHZ-_KEw/0.jpg)](https://www.youtube.com/watch?v=r7phHZ-_KEw)


<br>

<div align="center">
<h3 align="center">Let's connect 😋</h3>
</div>
<p align="center">
<a href="https://www.linkedin.com/in/hector-pulido-17547369/" target="blank">
<img align="center" width="30px" alt="Hector's LinkedIn" src="https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://twitter.com/Hector_Pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitter" src="https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"/></a> &nbsp; &nbsp;
<a href="https://www.twitch.tv/hector_pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitch" src="https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw" target="blank">
<img align="center" width="30px" alt="Hector's Youtube" src="https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://pequesoft.net/" target="blank">
<img align="center" width="30px" alt="Pequesoft website" src="https://github.com/HectorPulido/HectorPulido/blob/master/img/pequesoft-favicon.png?raw=true"/></a> &nbsp; &nbsp;

</p>