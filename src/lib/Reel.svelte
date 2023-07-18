<script lang="ts">
	import { onMount } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import ImageBlobReduce from 'image-blob-reduce';
	import { workspace_dir, current_photo } from '$lib/stores';

	let reel_node: HTMLDivElement;
	let photo_reel: string[] = [];

	let reel_idx: number = 0;

	const reducer = new ImageBlobReduce();

	export function set(filtered_photo_names: string[]) {
		photo_reel = filtered_photo_names;
	}

	export function next(): string {
		if (reel_idx < photo_reel.length - 1) {
			reel_idx++;
			reel_node.scrollBy(100, 0);
			return photo_reel[reel_idx];
		} else {
			return '';
		}
	}

	export function prev(): string {
		if (reel_idx > 0) {
			reel_idx--;
			reel_node.scrollBy(-100, 0);
			return photo_reel[reel_idx];
		} else {
			return '';
		}
	}

	let lazy_load_opts = {
		root: null,
		rootMargin: '0px',
		threshold: 0,
	};

	async function make_thumbnail(filename: string, max_size: number) {
		const src_url = convertFileSrc($workspace_dir + filename);
		const blob = await fetch(src_url).then((r) => r.blob());
		const thumbnail = await reducer.toBlob(blob, { max: max_size });
		return URL.createObjectURL(thumbnail);
	}

	export const lazyLoad = (image: HTMLImageElement, filename: string) => {
		const loaded = () => {
			image.style.opacity = '1'; // REPL hack to apply loading animation
		};
		const observer = new IntersectionObserver((entries) => {
			if (entries[0].isIntersecting) {
				make_thumbnail(filename, 150).then((url) => {
					image.src = url;
				});

				if (image.complete) {
					// check if instantly loaded
					loaded();
				} else {
					image.addEventListener('load', loaded); // if the image isn't loaded yet, add an event listener
				}
			}
		}, lazy_load_opts);
		observer.observe(image); // intersection observer

		return {
			destroy() {
				image.removeEventListener('load', loaded); // clean up the event listener
			},
		};
	};

	onMount(() => {
		reel_node.addEventListener('wheel', (event) => {
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

<div bind:this={reel_node} class="reel">
	<div class="reel-item">
		<div id="pad" />
	</div>

	<div class="reel-item">
		<!-- <img bind:this={rust_test} /> -->
	</div>

	{#each photo_reel as photo_name}
		<div class="reel-item">
			<!-- svelte-ignore a11y-missing-attribute -->
			<!-- <img src={image} /> -->
			<img use:lazyLoad={photo_name} />
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
		width: 100%;
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
			max-width: 150px;
			object-fit: cover;
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
