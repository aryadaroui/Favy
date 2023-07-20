<script lang="ts">
	import { onMount } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { workspace_dir } from '$lib/stores';

	export function set_image(path: string) {
		img_node.src = convertFileSrc($workspace_dir + path);
	}

	export function center() {
		if (zoom < 4) {
			// img_node.style.transition = 'transform 0.2s cubic-bezier(.5, 1.5, .7, .9)';
			img_node.style.transition = 'transform 0.2s cubic-bezier(.5, 1.4, .8, .95';
		} else {
			img_node.style.transition = 'transform 0.2s cubic-bezier(.4, 1.2, .9, .98';
		}

		zoom = 1;
		pan_x = 0;
		pan_y = 0;

		set_transform();
	}

	//   export let src: string;
	let viewer: HTMLElement;
	let img_node: HTMLImageElement;

	let zoom: number = 1;
	let pan_x: number = 0;
	let pan_y: number = 0;

	onMount(() => {
		// img_node.src = src;
		// img_node.src = '/untitled.jpg';
		

		img_node.addEventListener('transitionend', () => {
			img_node.style.transition = '';
		});

		viewer.addEventListener('wheel', (event) => {
			// zoom center based on mouse cursor position
			// BUG: zoom center is not correct when image is panned
			// could use this: https://stackoverflow.com/questions/60190965/zoom-scale-at-mouse-position

			img_node.style.transition = '';

			const cursor_x = event.clientX - viewer.clientLeft;
			const cursor_y = event.clientY - viewer.clientTop;
			const cursor_x_ratio = cursor_x / viewer.clientWidth;
			const cursor_y_ratio = cursor_y / viewer.clientHeight;

			const zoom_factor = 0.1;
			const max_zoom = 20;
			const min_zoom = 0.1;
			const zoom_delta = (event.deltaY > 0 ? -1 : 1) * zoom_factor;
			const new_zoom = Math.min(Math.max(zoom + zoom_delta, min_zoom), max_zoom);

			const pan_x_delta = (cursor_x_ratio - 0.5) * viewer.clientWidth * (zoom - new_zoom);
			const pan_y_delta = (cursor_y_ratio - 0.5) * viewer.clientHeight * (zoom - new_zoom);
			const new_pan_x = pan_x + pan_x_delta;
			const new_pan_y = pan_y + pan_y_delta;

			zoom = new_zoom;
			pan_x = new_pan_x;
			pan_y = new_pan_y;

			// if (zoom > 8) {
			// 	img_node.style.imageRendering = 'pixelated';
			// } else {
			// 	img_node.style.imageRendering = 'auto';
			// }

			set_transform();
		});

		let isDragging = false;
		let lastX = 0;
		let lastY = 0;

		viewer.addEventListener('mousedown', (event) => {
			event.preventDefault();
			isDragging = true;
			lastX = event.clientX;
			lastY = event.clientY;
		});

		viewer.addEventListener('mousemove', (event) => {
			event.preventDefault();
			if (isDragging) {
				const deltaX = event.clientX - lastX;
				const deltaY = event.clientY - lastY;
				pan_x += deltaX;
				pan_y += deltaY;
				set_transform();
				lastX = event.clientX;
				lastY = event.clientY;
			}
		});

		viewer.addEventListener('mouseup', () => {
			isDragging = false;
		});

		viewer.addEventListener('mouseleave', () => {
			isDragging = false;
		});
	});

	function set_transform() {
		img_node.style.transform = 'translate(' + pan_x + 'px,' + pan_y + 'px) scale(' + zoom + ')';
	}
</script>

<div bind:this={viewer}>
	<!-- svelte-ignore a11y-missing-attribute -->
	<img bind:this={img_node} />
</div>

<style lang="scss">
	div {
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		// border: 1px solid black;
		overflow: hidden;
		background-color: rgba(32, 32, 32, 0.7);
		user-select: none;
		-webkit-user-drag: none;
		cursor: grab;
		&:active {
			cursor: grabbing;
		}
	}

	img {
		user-select: none;
		-webkit-user-drag: none;
		height: 100%;
		width: auto;
		image-orientation: none;
		// image-rendering: optimizeQuality;
		// image-rendering: optimizeSpeed;

		translate: translate(0px, 100px);

		cursor: grab;
		&:active {
			cursor: grabbing;
		}
	}
</style>
