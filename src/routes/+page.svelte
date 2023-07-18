<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { readDir } from '@tauri-apps/api/fs';
	import type { FileEntry } from '@tauri-apps/api/fs';

	import ImageViewer from '$lib/ImageViewer.svelte';
	import Toolbar from '$lib/Toolbar.svelte';
	import Reel from '$lib/Reel.svelte';
	import {workspace_dir, } from '$lib/stores';

	// let $workspace_dir: string;
	let img_files: FileEntry[];
	let img_idx: number = 0;
	let image_viewer: ImageViewer;
	let reel: Reel;

	async function choose_dir() {
		open({
			directory: true,
			multiple: false,
			title: 'Choose folder', // including this speeds up dialog open time on macOS for unknown reason
		}).then(async (selecton) => {
			if (selecton) {
				$workspace_dir = selecton.toString();

				img_files = await readDir($workspace_dir, { recursive: false });
				img_files = img_files.filter((file) => {
					const imageExtensions = ['.jpg', '.jpeg', '.png', '.webp', '.tif', '.tiff'];
					const extension = file.name.substring(file.name.lastIndexOf('.')).toLowerCase();
					return imageExtensions.includes(extension);
				});
				img_files.sort((a, b) => a.name.localeCompare(b.name));

				img_idx = 0;
				const path_str = convertFileSrc(img_files[img_idx].path);
				image_viewer.set_image(path_str);

				// reel.set_images(img_files.map((file) => convertFileSrc(file.path)));
				reel.set_images(img_files.map((file) => file.path));
			} else {
			}
		});
	}

	function next() {
		if (img_idx < img_files.length - 1) {
			img_idx++;
		} else {
			// img_idx = 0;
			console.log('END OF REEL');
		}

		const path_str = convertFileSrc(img_files[img_idx].path);
		image_viewer.set_image(path_str);
		// console.log("app next");
		reel.next();
	}

	function prev() {
		if (img_idx > 0) {
			img_idx--;
		} else {
			// img_idx = img_files.length - 1;
			console.log('START OF REEL');
		}

		const path_str = convertFileSrc(img_files[img_idx].path);
		image_viewer.set_image(path_str);
		reel.prev();
	}

	function open_settings() {
		const settings_panel = new WebviewWindow('label', {
			url: '/settings',
		});
	}

	window.addEventListener('keydown', (e) => {
		if (e.key === 'Enter') {
			image_viewer.reset();
		}

		if (e.key === 'o' && e.metaKey) {
			choose_dir();
		}

		if (e.key === 'ArrowRight') {
			next();
		}

		if (e.key === 'ArrowLeft') {
			prev();
		}
	});
</script>

<main id="windowframe">
	<ImageViewer bind:this={image_viewer} />

	<Toolbar
		{choose_dir}
		center={() => {
			image_viewer.reset();
		}}
		settings={open_settings} />

	<div id="reel">
		<Reel bind:this={reel} />
	</div>
</main>

<style lang="scss">
	main#windowframe {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
	}
</style>
