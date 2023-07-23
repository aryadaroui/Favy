<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { readDir } from '@tauri-apps/api/fs';
	import type { FileEntry } from '@tauri-apps/api/fs';

	import ImageViewer from '$lib/ImageViewer.svelte';
	import Toolbar from '$lib/Toolbar.svelte';
	import Reel from '$lib/Reel.svelte';
	import { workspace_dir, photo_names, photo_map, current_photo } from '$lib/stores';

	// let $workspace_dir: string;
	let files: string[] | null;
	// let img_idx: number = 0;

	let image_viewer: ImageViewer;
	// let reel: Reel;
	let reel: Reel;

	async function choose_dir() {
		open({
			directory: true,
			multiple: false,
			title: 'Choose folder', // including this speeds up dialog open time on macOS for unknown reason
		}).then(async (selecton) => {
			if (selecton) {
				$workspace_dir = selecton.toString() + '/'; // SET workspace_dir store

				// get only the .name field of each FileEntry
				files = (await readDir($workspace_dir, { recursive: false })).map((file) => file.name as string);
				files = files.filter((file) => {
					const imageExtensions = ['.jpg', '.jpeg', '.png', '.webp', '.tif', '.tiff'];
					const extension = file.substring(file.lastIndexOf('.')).toLowerCase();
					return imageExtensions.includes(extension);
				});
				files.sort((a, b) => a.localeCompare(b));

				$photo_names = files; // SET photo_names store
				files.forEach((file, idx) => {
					// SET photo_map store
					$photo_map.set(file, {
						hate: false,
						rating: 0,
						love: false,
						thumbnail: null,
						idx: idx,
					});
				});
				files = null; // clear files array to prevent bugs. use photo_names instead

				// SET current_photo store
				$current_photo = {
					idx: 0,
					photo_name: $photo_names[0],
				};

				// const path_str = convertFileSrc(files[img_idx].path);
				image_viewer.set_image($current_photo.photo_name);

				// reel.set($photo_names.filter(filter));
				reel.set($workspace_dir, $photo_names.filter(filter), 2);
			} else {
			}
		});
	}

	function update_current_photo_by_idx(idx: number) {
		$current_photo = {
			idx: idx,
			photo_name: $photo_names[idx],
		};
		image_viewer.set_image($current_photo.photo_name);
	}

	function update_current_photo_by_name(photo_name: string) {
		const idx = $photo_map.get(photo_name)?.idx;
		image_viewer.center();

		if (idx != undefined) {
			$current_photo = {
				idx: idx,
				photo_name: photo_name,
			};
			image_viewer.set_image($current_photo.photo_name);
		} else {
			console.error('/:update_current_photo_by_name(): photo_name not found in photo_map');
		}
	}

	function filter(photo_name: string) {
		// placeholder
		return true;
	}

	function next() {
		let next_photo = reel.next_photo();

		if (next_photo != '') {
			update_current_photo_by_name(next_photo);
		} else {
			console.log('/:next(): cannot get next photo; end of reel');
		}
	}

	function prev() {
		let prev_photo = reel.prev_photo();

		if (prev_photo != '') {
			update_current_photo_by_name(prev_photo);
		} else {
			console.log('/:prev(): cannot get prev photo; beginning of reel');
		}
	}

	function open_settings() {
		const settings_panel = new WebviewWindow('label', {
			url: '/settings',
		});
	}

	window.addEventListener('keydown', (e) => {
		if (e.code === 'Space') {
			image_viewer.center();
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
			image_viewer.center();
		}}
		settings={open_settings} />

	<div>
	<Reel bind:this={reel} on_current_photo_change={update_current_photo_by_name} />
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
