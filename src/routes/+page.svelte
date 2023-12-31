<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { readDir } from '@tauri-apps/api/fs';
	import type { FileEntry } from '@tauri-apps/api/fs';

	import { Sentiment } from '$lib/stores';

	import Modal from '$lib/Modal.svelte';
	import Export from '$lib/Export.svelte';
	import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';

	import ImageViewer from '$lib/ImageViewer.svelte';
	import Toolbar from '$lib/Toolbar.svelte';
	import Reel from '$lib/Reel.svelte';
	import { workspace_dir, photo_names, photo_map, current_photo } from '$lib/stores';

	// let $workspace_dir: string;
	let files: string[] | null;
	// let img_idx: number = 0;

	let image_viewer: ImageViewer;
	let export_modal: Modal;
	let export_panel: Export;
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

				// TODO: turn this load into a function

				try {
					let result = await readTextFile($workspace_dir + 'photo_map.favy');
					$photo_map = new Map(JSON.parse(result));
					console.log('loaded photo_map.favy');
				} catch (error) {
					$photo_map = new Map(); // TODO; add type annotation here
					console.log("couldn't load photo_map.favy");
				}

				// get only the .name field of each FileEntry
				files = (await readDir($workspace_dir, { recursive: false })).map(
					(file) => file.name as string,
				);
				files = files.filter((file) => {
					const imageExtensions = ['.jpg', '.jpeg', '.png', '.webp', '.tif', '.tiff'];
					const extension = file.substring(file.lastIndexOf('.')).toLowerCase();
					return imageExtensions.includes(extension);
				});
				files.sort((a, b) => a.localeCompare(b));

				$photo_names = files; // SET photo_names store

				let new_files = files.filter((file) => !$photo_map.has(file)); // get files not in photo_map. purpose is to prevent duplicates from loaded .favy file

				new_files.forEach((file, idx) => {
					// SET photo_map store
					$photo_map.set(file, {
						rating: 0,
						love: Sentiment.Neutral,
					});
				});
				// files = null; // clear files array to prevent bugs. use photo_names instead

				// SET current_photo store
				$current_photo = {
					idx: 0,
					photo_name: $photo_names[0],
				};

				image_viewer.set_image($current_photo.photo_name);

				// TODO: Filter photos ASAP!
				reel.set($workspace_dir, $photo_names.filter(filter), 100);

				window.favy = {};
				window.favy.workspace_dir = $workspace_dir;
				window.favy.photo_map = $photo_map;
				window.favy.photo_names = $photo_names;
				window.favy.current_photo = $current_photo;
			} else {
			}
		});
	}

	function save() {
		writeTextFile($workspace_dir + 'photo_map.favy', JSON.stringify([...$photo_map]));
		// TODO: update status
	}

	// async function load() {
	// 	readTextFile($workspace_dir + 'photo_map.favy')
	// 		.then((file) => {
	// 			// debugger
	// 			$photo_map = new Map(JSON.parse(file));
	// 			console.log('loaded photo_map.favy');
	// 		})
	// 		.catch((err) => {
	// 			console.log("couldn't load photo_map.favy");
	// 		});
	// }

	function update_current_photo_by_idx(idx: number) {
		$current_photo = {
			idx: idx,
			photo_name: $photo_names[idx],
		};
		image_viewer.set_image($current_photo.photo_name);
	}

	function update_current_photo_by_name(photo_name: string) {
		// const idx = $photo_map.get(photo_name);
		image_viewer.center();

		$current_photo = {
			idx: $photo_names.indexOf(photo_name),
			photo_name: photo_name,
		};
		image_viewer.set_image($current_photo.photo_name);
	}

	function filter(photo_name: string) {
		// placeholder
		return true;
	}

	function next() {
		let next_photo = reel.next_photo();

		// if (next_photo != '') {
		// 	update_current_photo_by_name(next_photo);
		// } else {
		// 	// console.log('/:next(): cannot get next photo; end of reel');
		// }
	}

	function prev() {
		let prev_photo = reel.prev_photo();

		// if (prev_photo != '') {
		// 	update_current_photo_by_name(prev_photo);
		// } else {
		// 	// console.log('/:prev(): cannot get prev photo; beginning of reel');
		// }
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

		// call save if user presses cmd+s
		if (e.key === 's' && e.metaKey) {
			console.log('saving!');
			save();
		}
	});
</script>

<main id="windowframe">
	<Modal bind:this={export_modal}>
		<Export bind:this={export_panel} />
	</Modal>

	<ImageViewer bind:this={image_viewer} />
	

	<Toolbar
		{choose_dir}
		center={() => {
			image_viewer.center();
		}}
		settings={open_settings}
		on_export_clicked={() => {
			export_panel.check_dirs();
			export_modal.open();
		}} />

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
