<script lang="ts">
  import { onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import ImageBlobReduce from "image-blob-reduce";

  let reel: HTMLDivElement;
  let img_srcs: string[] = [];

  const reducer = new ImageBlobReduce();

  export function set_images(new_img_srcs: string[]) {
    img_srcs = new_img_srcs;
  }

  export function next() {
    reel.scrollBy(100, 0);
  }

  export function prev() {
    reel.scrollBy(-100, 0);
  }

  let lazy_load_opts = {
    root: null,
    rootMargin: "0px",
    threshold: 0,
  };

  async function make_thumbnail(src_url: string, max_size: number) {
    const blob = await fetch(src_url).then((r) => r.blob());
    const thumbnail = await reducer.toBlob(blob, { max: max_size });
    return URL.createObjectURL(thumbnail);
  }

  export const lazyLoad = (image: HTMLImageElement, src_url: string) => {
    const loaded = () => {
      image.style.opacity = "1"; // REPL hack to apply loading animation
    };
    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting) {

        make_thumbnail(convertFileSrc(src_url), 150).then((url) => {
          image.src = url;
        });

        if (image.complete) {
          // check if instantly loaded
          loaded();
        } else {
          image.addEventListener("load", loaded); // if the image isn't loaded yet, add an event listener
        }
      }
    }, lazy_load_opts);
    observer.observe(image); // intersection observer

    return {
      destroy() {
        image.removeEventListener("load", loaded); // clean up the event listener
      },
    };
  };

  onMount(() => {
    reel.addEventListener("wheel", (event) => {
      if (!event.deltaY) {
        return;
      } else if (Math.abs(event.deltaY) < 20) {
        // ignore small movements, like trackpad scrolling
        return;
      }

      //@ts-ignore - .scrollLeft IS a property of currentTarget
      event.currentTarget.scrollLeft += event.deltaY + event.deltaX;
      event.preventDefault();
    });
  });
</script>

<div bind:this={reel} class="reel">
  <div class="reel-item">
    <div id="pad" />
  </div>

  <div class="reel-item">
    <!-- <img bind:this={rust_test} /> -->
  </div>

  {#each img_srcs as img_src, idx}
    <div class="reel-item">
      <!-- svelte-ignore a11y-missing-attribute -->
      <!-- <img src={image} /> -->
      <img use:lazyLoad={img_src} />
    </div>
  {/each}

  <div class="reel-item">
    <div id="pad" />
  </div>

  <div id="center-marker" />
</div>

<style lang="scss">
  div.reel {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    // justify-content: center;
    scroll-behavior: smooth;
    align-items: center;

    background-color: rgba(40, 40, 40, 1);
    height: 200px;
    width: calc(100vw - 2px);
    flex-wrap: nowrap;
    overflow-x: scroll;
    overflow-y: hidden;
    // height: 100vh;
    // width: 100vw;
    background-color: rgba(32, 32, 32, 0.7);

    scroll-snap-type: x mandatory;

    ::-webkit-scrollbar {
      width: 0;
      display: none;
    }

    .reel-item {
      border: 1px solid gray;
    }

    #pad {
      width: calc(50vw - 75px - 20px - 2px);
      height: 200px;
      background-color: rgba(16, 16, 16, 0.2);
    }

    img {
      max-height: 150px;
      scroll-snap-align: center;
      cursor: pointer;
      margin: 0 20px;
      image-rendering: optimizeSpeed;
      transition: height 0.2s ease-in-out;
    }

    #center-marker {
      position: absolute;
      left: 50vw;
      width: 1px;
      height: 200px;
      border: 1px solid white;
    }
  }
</style>
