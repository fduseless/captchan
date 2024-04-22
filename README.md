# CaptchaN

Captcha for Node, which generates captcha image by Rust.

## Feature

- Native Package base on Rust.
- Simple, Easy to use;
- High performance.

## Install

Install captchan with yarn:

```bash
yarn add captchan
```

## Usage

```javascript
import { createImage, Format } from '../index.js'

// with default value
const {text, image} = createImage();

// with options
const {text, image} = createImage(
    len=4, // num of character
    difficulty=5, // range [1,10]
    line=true, // draw bezier curve or ellipse
    noise=false, // whether add gaussian noise
    format=Format.PNG, // PNG | JPG | JPEG | WEBP
);
```

## Thanks

[rucaptcha](https://github.com/huacnlee/rucaptcha)
