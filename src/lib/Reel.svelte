<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import ImageBlobReduce from 'image-blob-reduce';
	import { status } from '$lib/stores';
	import { invoke } from '@tauri-apps/api/tauri';

	import ArrowRight from '$lib/icons/ArrowRight.svelte';
	import ArrowLeft from '$lib/icons/ArrowLeft.svelte';

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
		set: (photo_idx: number, page_idx: number, scroll?: boolean) => void;
	}

	// // // declarations
	// utility
	const ThumbProcessor = {
		NODE_SHARP: 'NODE_SHARP', // leverages multi-core
		WEB_CANVAS: 'WEB_CANVAS', // leverages GPU
		RUST_SIMD: 'RUST_SIMD', // leverages CPU-SIMD
	};
	const thumb_processor = ThumbProcessor.RUST_SIMD;
	const reducer = new ImageBlobReduce();
	let animating = false;

	let dir: string;
	let reel_node: HTMLDivElement;
	let photo_table: string[][] = [];

	const photo_reel: PhotoReel = {
		buffer: [],
		set(page_idx: number) {
			photo_reel.buffer = [];
			// photo_reel.buffer = photo_table[page_idx];

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
		set(photo_idx: number, page_idx: number, scroll: boolean = true) {
			// TODO: use svelte's builtin class directives for styling. unfortunately i can't get it to update at the moment.

			document.getElementById(current.photo.name)?.classList.remove('reel-selected-photo');
			this.photo.idx = photo_idx;
			this.photo.name = photo_table[page_idx][photo_idx];
			this.page_idx = page_idx;

			on_current_photo_change(current.photo.name);
			if (scroll) {
				scroll_to_photo(current.photo.name);
			}

			$status.reel.idx = current.photo.idx + 1;
			$status.reel.len = photo_table[current.page_idx].length;
			$status.page.idx = current.page_idx + 1;
			$status.page.len = photo_table.length;
			$status.text = '';

			document.getElementById(current.photo.name)?.classList.add('reel-selected-photo');
		},
	};

	// // // exports
	export let on_current_photo_change: (photo_name: PhotoName) => void;

	export function set(new_dir: string, new_photo_names: PhotoName[], page_size: number) {
		dir = new_dir;
		photo_table = [];
		// populate photo_table with each subarray being a page of photos with max length of page_size
		for (let i = 0; i < new_photo_names.length; i += page_size) {
			photo_table.push(new_photo_names.slice(i, i + page_size));
		}
		photo_reel.set(0);
		current.set(0, 0);
	}

	export function next_photo(): void {
		if (current.photo.idx < photo_reel.buffer.length - 1) {
			current.set(current.photo.idx + 1, current.page_idx);
		} else {
			// if at the end of the current page, try to go to the next page
			next_page();
		}
	}

	export function prev_photo(): void {
		if (current.photo.idx > 0) {
			current.set(current.photo.idx - 1, current.page_idx);
		} else {
			// if at the beginning of the current page, try to go to the previous page
			prev_page();
		}
	}

	export function next_page(): void {
		if (!animating) {
			if (current.page_idx < photo_table.length - 1 && !animating) {
				animating = true;
				reel_node.animate(
					[
						// keyframes
						{ transform: 'translateX(0)' },
						{ transform: 'translateX(-100%)' },
					],
					{
						// timing options
						duration: 250,
						easing: 'ease-in',
						// fill: 'forwards',
					},
				).onfinish = () => {
					photo_reel.buffer = [];

					reel_node.animate([{ transform: 'translateX(100%)' }, { transform: 'translateX(0)' }], {
						duration: 250,
						easing: 'ease-out',
					}).onfinish = () => {
						photo_reel.set(current.page_idx + 1);
						current.set(0, current.page_idx + 1, false);
						animating = false;
					};
				};
			} else {
				console.log('end of reel');
			}
		} else {
			console.log('next_page(): denied. animating');
		}
	}

	export function prev_page(): void {
		if (!animating) {
			if (current.page_idx > 0) {
				animating = true;
				reel_node.animate([{ transform: 'translateX(0)' }, { transform: 'translateX(100%)' }], {
					duration: 250,
					easing: 'ease-in-out',
				}).onfinish = () => {
					photo_reel.set(current.page_idx - 1);
					current.set(photo_table[current.page_idx - 1].length - 1, current.page_idx - 1, false);

					reel_node.animate([{ transform: 'translateX(-100%)' }, { transform: 'translateX(0)' }], {
						duration: 250,
						easing: 'ease-in-out',
					}).onfinish = () => {
						animating = false;
					};
				};
			} else {
				console.log('beginning of reel');
			}
		} else {
			console.log('prev_page(): denied. animating');
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
				// console.error(`scroll_to_photo(): photo_name '${photo_name}'' not found`);
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
				// TODO: this potentially makes more and more URLs if the user returns to the same photo.
				// should implement a cache for this
				return URL.createObjectURL(thumbnail);

			case ThumbProcessor.RUST_SIMD:
				// console.log('make_thumbnail(): rust simd');
				const thumb_base64_ = await invoke('resize_simd', {
					image_path: dir + photo_name,
				});

				return 'data:image/jpeg;base64,' + thumb_base64_;

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
						observer.unobserve(img); // stop observing this image
						img.removeEventListener('load', () => {});
					});

					make_thumbnail(img.id, 150).then((url) => {
						img.src = url;
					});
				}
			}
		},
		{
			// this is ugly. TODO: clean up
			root: null,
			rootMargin: '0px',
			threshold: 0,
		},
	);

	function lazy_load(img: HTMLImageElement) {
		observer.observe(img); // intersection observer

		if (img.id == current.photo.name) {
			img.classList.add('reel-selected-photo');
			scroll_to_photo(current.photo.name);
		}
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

		// add event listener for shift left
		window.addEventListener('keydown', (event) => {
			if (event.key == 'ArrowLeft' && event.shiftKey) {
				reel_node.scroll({ left: 0, behavior: 'smooth' });

				// TODO: if control key is held, select the first photo in the reel.
			}

			if (event.key == 'ArrowRight' && event.shiftKey) {
				reel_node.scroll({
					left: reel_node.scrollWidth - reel_node.clientWidth,
					behavior: 'smooth',
				});
				// TODO: if control key is held, select the select photo in the reel.
			}
		});
	});
</script>

<!-- TODO: Add buttons for previous and next pages into the pad -->
<div bind:this={reel_node} class="reel">
	{#if photo_reel.buffer.length == 0}
		<div class="scroll-item">
			<div class="pad-blank" />
		</div>
	{:else}
		<div class="scroll-item">
			<div class="pad pad-left">
				{#if current.page_idx > 0}
					<button class="left" on:click={prev_page}>
						<ArrowLeft />
					</button>
				{/if}
			</div>
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
			<div class="pad pad-right">
				{#if current.page_idx < photo_table.length - 1}
					<button class="right" on:click={next_page}>
						<ArrowRight />
					</button>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style lang="scss">
	// ::-webkit-scrollbar {
	// 	width: 0;
	// 	display: none;
	// 	background: transparent;
	// }

	// ::-webkit-scrollbar-thumb {
	// 	background: #ff0000;
	// }

	.alternate-scrollbar::-webkit-scrollbar {
		width: 8px;
		height: 8px;
		margin: 8px;
		padding: 8px;

		// background-color: red;
	}

	.alternate-scrollbar::-webkit-scrollbar-thumb {
		background: rgb(16, 16, 16);
		border-radius: 5px;
		margin: 8px;
		padding: 8px;
	}

	.alternate-scrollbar::-webkit-scrollbar-track {
		// background: gray;
		width: 8px;
		height: 8px;
		padding: 8px;
		margin: 8px;
	}

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

		// scrollbar-color: black transparent;

		// ::-webkit-scrollbar {
		// 	width: 11px;
		// }

		// ::-webkit-scrollbar-track {
		// 	background: black;
		// }

		// ::-webkit-scrollbar-thumb {
		// 	background-color: black;
		// 	border-radius: 6px;
		// 	border: 3px solid black;
		// }

		.scroll-item {
			// border: 1px solid gray;
		}

		.pad-left {
			justify-content: right;
		}

		.pad-right {
			justify-content: left;
		}

		.pad-blank {
			height: 200px;
			width: 190px;
		}

		.pad {
			width: calc(50vw - 75px - 20px);

			height: 200px;
			background-color: rgba(16, 16, 16, 0.2);

			display: flex;
			align-items: center;

			button {
				width: 64px;
				height: 64px;

				transition: all 0.05s ease-in-out;
			}

			button.right {
				margin-left: 20px;
			}

			button.left {
				margin-right: 20px;
			}
		}

		img {
			max-height: 150px;
			max-width: 150px;
			min-width: 100px;
			min-height: 100px;
			object-fit: cover;
			scroll-snap-align: center;
			cursor: pointer;
			margin: 0 20px;
			image-rendering: optimizeSpeed;
			border: 1px solid transparent;
			transition: all 0.1s ease-in;
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

	:global(.reel-selected-photo) {
		border: 1px solid white !important;
	}
</style>
