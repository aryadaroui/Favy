<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import ImageBlobReduce from 'image-blob-reduce';
	import { status } from '$lib/stores';

	// TODO: need an evernt dispatch on lazy load done
	// TODO: need to add init .src image so it doesnt constantly re-fetch thumbnails

	const ThumbProcessor = {
		NODE_SHARP: 'NODE_SHARP', // leverages multi-core
		WEB_CANVAS: 'WEB_CANVAS', // leverages GPU
		// RUST_SIMD: 'RUST_SIMD', // leverages CPU-SIMD
	};
	const thumb_processor = ThumbProcessor.NODE_SHARP;

	// // // types
	type PhotoName = string;

	interface PhotoReel {
		buffer: PhotoName[];
		set: (page_idx: number) => void;
	}

	interface Current {
		photo: {
			idx: number;
			name: string;
		};
		page_idx: number;
		set: (photo_idx: number, page_idx: number) => void;
	}

	// // // declarations
	let dir: string;
	let reel_node: HTMLDivElement;
	let photo_table: string[][] = [];

	const photo_reel: PhotoReel = {
		buffer: [],
		set(page_idx: number) {
			photo_reel.buffer = [];
			tick().then(() => {
				photo_reel.buffer = photo_table[page_idx];
			});
		},
	};

	const current: Current = {
		photo: {
			idx: 0,
			name: '',
		},
		page_idx: 0,
		set(photo_idx: number, page_idx: number) {
			// TODO: use svelte's builtin class directives for styling. unfortunately i can't get it to update at the moment.

			document.getElementById(current.photo.name)?.classList.remove('selected');
			this.photo.idx = photo_idx;
			this.photo.name = photo_table[page_idx][photo_idx];
			this.page_idx = page_idx;

			on_current_photo_change(current.photo.name);
			scroll_to_photo(current.photo.name);
			$status = `🌌 ${current.photo.idx + 1}/${photo_table[current.page_idx].length}, 📑 ${current.page_idx + 1}/${
				photo_table.length
			}`;
			document.getElementById(current.photo.name)?.classList.add('selected');
		},
	};
	const reducer = new ImageBlobReduce();

	// // // exports
	export let on_current_photo_change: (photo_name: PhotoName) => void;

	export function set(new_dir: string, new_photo_names: PhotoName[], page_size: number) {
		dir = new_dir;
		// populate photo_table with each subarray being a page of photos with max length of page_size
		for (let i = 0; i < new_photo_names.length; i += page_size) {
			photo_table.push(new_photo_names.slice(i, i + page_size));
		}
		photo_reel.set(0);
		current.set(0, 0);
	}

	export function next_photo(): PhotoName {
		if (current.photo.idx < photo_reel.buffer.length - 1) {
			current.set(current.photo.idx + 1, current.page_idx);
			return current.photo.name;
		} else {
			// if at the end of the current page, try to go to the next page
			return next_page();
		}
	}

	export function next_page(): PhotoName {
		if (current.page_idx < photo_table.length - 1) {
			photo_reel.set(current.page_idx + 1);
			current.set(0, current.page_idx + 1);
			return current.photo.name;
		} else {
			console.log('end of reel');
			return '';
		}
	}

	export function prev_photo(): PhotoName {
		if (current.photo.idx > 0) {
			current.set(current.photo.idx - 1, current.page_idx);
			return current.photo.name;
		} else {
			// if at the beginning of the current page, try to go to the previous page
			return prev_page();
		}
	}

	export function prev_page(): PhotoName {
		if (current.page_idx > 0) {
			photo_reel.set(current.page_idx - 1);
			current.set(photo_table[current.page_idx - 1].length - 1, current.page_idx - 1);
			return current.photo.name;
		} else {
			console.log('beginning of reel');
			return '';
		}
	}

	export function goto_page(page_idx: number): PhotoName {
		if (page_idx >= 0 && page_idx < photo_table.length) {
			photo_reel.set(page_idx);
			current.set(0, page_idx);
			return current.photo.name;
		} else {
			console.error(`goto_page(): page_idx ${page_idx} out of bounds`);
			return '';
		}
	}

	// // // internal functions

	function scroll_to_photo(photo_name: PhotoName) {
		if (photo_name != '') {
			const photo_node = document.getElementById(photo_name);
			if (photo_node != null) {
				photo_node.scrollIntoView({ behavior: 'smooth', inline: 'center' });
			} else {
				console.error(`scroll_to_photo(): photo_name '${photo_name}'' not found`);
			}
		}
	}

	async function make_thumbnail(photo_name: PhotoName, max_size: number) {
		switch (thumb_processor) {
			case ThumbProcessor.NODE_SHARP:
				const response = await fetch('/make-thumb', {
					method: 'POST',
					cache: 'force-cache',
					body: JSON.stringify({ src_url: dir + photo_name, max_size: max_size }),
				});
				const { thumb_base64 } = await response.json();
				return 'data:image/jpeg;base64,' + thumb_base64;

			case ThumbProcessor.WEB_CANVAS:
				const blob = await fetch(convertFileSrc(dir + photo_name)).then((r) => r.blob());
				const thumbnail = await reducer.toBlob(blob, { max: max_size });
				return URL.createObjectURL(thumbnail);

			// case ThumbProcessor.RUST_SIMD:
			// 	break;

			default:
				throw new Error(`make_thumbnail(): unknown thumb_processor ${thumb_processor}`);
		}
	}

	const observer = new IntersectionObserver(
		(entries) => {
			for (let i = 0; i < entries.length; i++) {
				const entry = entries[i];
				if (entry.isIntersecting) {
					const img = entry.target as HTMLImageElement;

					img.addEventListener('load', (event) => {
						if (img.id == current.photo.name) {
							img.classList.add('selected');
							scroll_to_photo(current.photo.name);
						}
						observer.unobserve(img); // stop observing this image
						img.removeEventListener('load', () => {});
					});

					make_thumbnail(img.id, 150).then((url) => {
						img.src = url;
					});
				}
			}
		},
		{ // this is ugly. TODO: clean up
			root: null,
			rootMargin: '0px',
			threshold: 0,
		},
	);

	function lazy_load(img: HTMLImageElement) {
		observer.observe(img); // intersection observer
	}

	function handle_photo_click(event: MouseEvent) {
		// the id of the element is the photo_name
		const photo_name = (event.target as HTMLImageElement).id;
		current.set(photo_table[current.page_idx].indexOf(photo_name), current.page_idx);
	}

	// // // lifecycle

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
	<div class="scroll-item">
		<div class="pad" />
	</div>

	{#each photo_reel.buffer as photo_name}
		<div class="scroll-item">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
			<!-- svelte-ignore a11y-missing-attribute -->
			<img id={photo_name} use:lazy_load on:click={handle_photo_click} />
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
		scroll-behavior: smooth;
		align-items: center;

		background-color: rgba(40, 40, 40, 1);
		height: 200px;
		width: 100%;
		flex-wrap: nowrap;
		overflow-x: scroll;
		overflow-y: hidden;
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
			border: 1px solid transparent;
			// transition: border 0.4s ease-in; // laggy
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
		border: 1px solid white !important;
		// transition: border 0.4s ease-out;
	}
</style>
