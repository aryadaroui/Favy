# `<img>` is a bit faster than `<canvas>`

Loading an img with

```js
img_node.src = path_str
```

is faster than loading the same image with

```js
image.src = path_str
ctx.drawImage(image, ...)
```

This was expected, but now there is empirical evidence. To get times, used the general pattern,

```js
const start_time = new Date().getTime();
image.onload = () => {
  const load_time = new Date().getTime() - start_time;
  console.log(`Image loaded in ${load_time}ms`);
};
```

The `<img>` method feels a bit snappier too.

## `<img>` times in ms

```
4
4
5
6
4
5
5
5
4
5
5
```

## `<canvas>` times in ms

with single Image

```
13
13
13
7
4
5
8
6
5
6
5
```



# Caching thumbnails and fullsize


```
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21
        ^_^_|_^_^_^__^__^
```

will check for `cache[img_idx]` on each load, if it fails, then do a full load. on load end, then replace cache entry/entries.

# Making thumbnails

Rust `backend read -> resize-> base64 jpeg -> frontend img` was surprisingly slower than using `frontend read -> canvas/blob -> img`. I imagine it's beacuse the IPC layer is too slow. Should profile later.

# UI popover

can use CSS `visibility:none`, and then on:click, set the popover div to :focus. which is visible. That way, there's native functionality for clicking outside the div and making it go away.

Might be too many clicks though. May want to also do :hover for visibility instead



## possible dependencies?

when using the https://crates.io/crates/kamadak-exif crate, i had 

