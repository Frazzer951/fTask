<div id="top"></div>

<!-- PROJECT SHIELDS -->
<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">fTask</h3>

  <p align="center">
    Frazzer's Task Manager is a CLI application for managing Tasks. 
    <br />
    <a href="https://github.com/frazzer951/ftask"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/frazzer951/ftask/issues">Report Bug</a>
    ·
    <a href="https://github.com/frazzer951/ftask/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

fTask is used to help keep track of all of your tasks and todos. Tasks have three properties, name, description and
priority. The lower the priority number, the higher the priority of the task.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

- [Rust](https://www.rust-lang.org/)
- [Clap](https://github.com/clap-rs/clap)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

- Optional - Cargo
    - https://rustup.rs/
    - This makes the installation process much easier, but is optional

### Installation

#### Option 1 - Easier Method

1. Install the cargo crate directly from GitHub
   ```sh
   cargo install --git https://github.com/Frazzer951/ftask.git
   ```
   To install a specific release version use `--tag <VERSION TAG>`

#### Option 2 - Manual Install

1. Download the binary from the latest release for your platform from
   the [release page](https://github.com/Frazzer951/ftask/releases)
2. Place the binary into a folder seen by your OSes PATH variable, so it can be called from anywhere

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Basic usage can be seen by looking at the help information for each command

```sh
ftask -h
```

To add a simple project use

```shell
ftask new -n "Help Orphans" -d "Goto the orphanage and help the children" -p 0
```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any
contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also
simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes
   using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request to the develop branch

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Your Name - luke343279@gmail.com

Project Link: [https://github.com/frazzer951/ftask](https://github.com/frazzer951/ftask)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/frazzer951/ftask.svg?style=for-the-badge

[contributors-url]: https://github.com/frazzer951/ftask/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/frazzer951/ftask.svg?style=for-the-badge

[forks-url]: https://github.com/frazzer951/ftask/network/members

[stars-shield]: https://img.shields.io/github/stars/frazzer951/ftask.svg?style=for-the-badge

[stars-url]: https://github.com/frazzer951/ftask/stargazers

[issues-shield]: https://img.shields.io/github/issues/frazzer951/ftask.svg?style=for-the-badge

[issues-url]: https://github.com/frazzer951/ftask/issues

[license-shield]: https://img.shields.io/github/license/frazzer951/ftask?style=for-the-badge

[license-url]: https://github.com/Frazzer951/ftask/blob/main/LICENSE

[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555

[linkedin-url]: https://linkedin.com/in/luke-eltiste