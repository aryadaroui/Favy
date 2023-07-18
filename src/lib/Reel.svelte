<script lang="ts">
	import { onMount } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import ImageBlobReduce from 'image-blob-reduce';
	import { workspace_dir, status, settings } from '$lib/stores';
	import { tick } from 'svelte';

	let reel_node: HTMLDivElement;
	let filtered_photo_names: string[] = [];
	let photo_reel: string[] = [];
	// $: photo_reel = [''];

	let reel_idx: number = 0;

	let page_idx: number = 0;
	let num_pages: number = 0;

	export let asdf = 0;

	function set_reel_idx(idx: number) {
		document.getElementById(photo_reel[reel_idx])?.classList.remove('selected');
		reel_idx = idx;
		document.getElementById(photo_reel[reel_idx])?.classList.add('selected');

		$status = `${reel_idx + 1}/${photo_reel.length}`;
	}

	const reducer = new ImageBlobReduce();

	export let parent_updater: (photo_name: string) => void;

	export function set(new_filtered_photo_names: string[]) {
		photo_reel = [];
		// force svelte to update the photo reel before editing it
		tick().then(() => {
			filtered_photo_names = new_filtered_photo_names;
			photo_reel = filtered_photo_names.slice(0, $settings.reel_size);

			page_idx = 0;
			num_pages = Math.floor(filtered_photo_names.length / $settings.reel_size);

			set_reel_idx(0); // BUG: does not update the style on the first image for some reason
			scroll_to_photo(photo_reel[reel_idx]);
		});
	}

	export function next_reel() {
		if (page_idx < num_pages - 1) {
			photo_reel = [];
			tick().then(() => {
				page_idx += 1;
				// i really don't like this.
				// TODO: make photo_reel a 2D array
				photo_reel = filtered_photo_names.slice(page_idx * $settings.reel_size, (page_idx + 1) * $settings.reel_size);
				set_reel_idx(0);
				scroll_to_photo(photo_reel[reel_idx]);
			});
		} else {
			console.log('no more pages');
		}
	}

	export function next(): string {
		if (reel_idx < photo_reel.length - 1) {
			set_reel_idx(reel_idx + 1);
			scroll_to_photo(photo_reel[reel_idx]);
			return photo_reel[reel_idx];
		} else {
			if (page_idx < num_pages - 1) {
				next_reel();
				return photo_reel[reel_idx];
			} else {
				return '';
			}
		}
	}

	export function prev(): string {
		if (reel_idx > 0) {
			set_reel_idx(reel_idx - 1);
			scroll_to_photo(photo_reel[reel_idx]);
			return photo_reel[reel_idx];
		} else {
			return '';
		}
	}

	function scroll_to_photo(id: string) {
		const photo = document.getElementById(id);
		set_reel_idx(photo_reel.indexOf(id));
		if (photo) {
			photo.scrollIntoView({ behavior: 'smooth', inline: 'center' });
		}
	}

	function photo_on_click(event: MouseEvent) {
		// get the id of the element that was clicked
		const id = (event.target as HTMLImageElement).id;
		scroll_to_photo(id);

		parent_updater(id);
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

	const lazy_load = (image: HTMLImageElement, filename: string) => {
		const loaded = () => {
			image.style.opacity = '1'; // REPL hack to apply loading animation

			// if image is first in reel, add selected class
			// this is workaround for the bug where the first image in the reel does not get the selected class immediately
			if (image.id === photo_reel[0] && !image.classList.contains('selected') && reel_idx === 0) {
				image.classList.add('selected');
			}
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

		window.addEventListener('keydown', (event) => {
			// check for ` key
			// if (event.key === '`') {
			// 	console.log('resetting reel');
			// 	// let first = photo_reel[0];
			// 	// photo_reel = [];
			// 	// document.getElementById(first)?.remove();
			// 	// set([])
			// }
		});
	});
</script>

<div bind:this={reel_node} class="reel">
	<div class="scroll-item">
		<div class="pad">
			<!-- <p>photo reel is {photo_reel}</p> -->
		</div>
	</div>

	{#each photo_reel as photo_name}
		<div class="scroll-item">
			<img id={photo_name} use:lazy_load={photo_name} on:click={photo_on_click} />
		</div>
	{/each}

	<div class="scroll-item">
		<div class="pad" />
	</div>

	<!-- <div id="center-marker" /> -->
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

		.scroll-item {
			// border: 1px solid gray;
		}

		.pad {
			width: calc(50vw - 75px - 20px);
			// margin: 0 20px;
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

	:global(.selected) {
		border: 1px solid white;
	}
</style>
