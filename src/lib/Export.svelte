<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { exists } from '@tauri-apps/api/fs';
	import { workspace_dir, photo_map } from './stores';

	import Heart from '$lib/icons/Heart.svelte';
	import Stars3 from '$lib/icons/Stars3.svelte';
	import Stars2 from '$lib/icons/Stars2.svelte';
	import Stars1 from '$lib/icons/Stars1.svelte';
	import Xcross from '$lib/icons/Xcross.svelte';
	import Trash from '$lib/icons/Trash.svelte';
	import ButtonToggle from '$lib/ButtonToggle.svelte';

	let export_node: HTMLDivElement;

	interface Dirs {
		existing: string[];
		check: () => Promise<void>;
		update: () => void;
	}

	$: $workspace_dir, check_dirs();

	export function check_dirs() {
		dirs.check();
	}

	let dirs: Dirs = {
		existing: [],
		async check() {
			let buf = [];
			if ($workspace_dir != '') {
				if (await exists($workspace_dir + 'love')) {
					buf.push('love');
				}
				if (await exists($workspace_dir + 'star3')) {
					buf.push('star3');
				}
				if (await exists($workspace_dir + 'star2')) {
					buf.push('star2');
				}
				if (await exists($workspace_dir + 'star1')) {
					buf.push('star1');
				}
				if (await exists($workspace_dir + 'hate')) {
					buf.push('hate');
				}
				this.existing = buf;
				console.log('existing: ', this.existing);
				this.update();
			}
		},
		update() {
			dirs = dirs;
		},
	};

	let settings = {
		heart: true,
		star3: true,
		star2: false,
		star1: false,
		hate: false,
		delete_original: false,
		update() {
			settings = settings;
		},
		set_heart(val: boolean) {
			settings.heart = val;
			settings.update();
		},
		set_star3(val: boolean) {
			settings.star3 = val;
			settings.update();
		},
		set_star2(val: boolean) {
			settings.star2 = val;
			settings.update();
		},
		set_star1(val: boolean) {
			settings.star1 = val;
			settings.update();
		},
		set_hate(val: boolean) {
			settings.hate = val;
			settings.update();
		},
		set_delete_original(val: boolean) {
			settings.delete_original = val;
			settings.update();
		},
	};

	enum ExportStatus {
		Ready = 'ready',
		Working = 'working',
		Done = 'done',
		Failed = 'failed',
	}

	let export_status = ExportStatus.Ready;

	function handle_export() {
		// make sure to save user progress before export.

		export_status = ExportStatus.Working;
		const dir = $workspace_dir; // need to make a copy because $workspace_dir is a store; tauri doesn't like it.
		const photos = Array.from($photo_map, ([name, value]) => ({ name, ...value }));

		invoke('export', {
			dir,
			photos,
			settings,
		})
			.then((res) => {
				console.log(res);
				export_status = ExportStatus.Done;
			})
			.catch((err) => {
				console.log(err);
				export_status = ExportStatus.Failed;
			});
	}
</script>

<div bind:this={export_node} class="export_panel">
	<h1>Export</h1>
	<p>Copy selected photos to subdirectories</p>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_heart(is_on_after);
			}}>
			<Heart />
		</ButtonToggle> Copy dir: ./love <br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_star3(is_on_after);
			}}>
			<Stars3 />
		</ButtonToggle> Copy dir: ./star3 <br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_star2(is_on_after);
			}}>
			<Stars2 />
		</ButtonToggle> Copy dir: ./star2 <br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_star1(is_on_after);
			}}>
			<Stars1 />
		</ButtonToggle> Copy dir: ./star1 <br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_hate(is_on_after);
			}}>
			<Xcross />
		</ButtonToggle> Copy dir: ./hate <br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_delete_original(is_on_after);
			}}>
			<Trash />
		</ButtonToggle>
		<span class:warning={settings.delete_original}
			>Original files
			{#if settings.delete_original}
				WILL
			{:else}
				will not
			{/if}
			be deleted.</span> <br />
	</div>

	{#if $workspace_dir != ''}
		{#if dirs.existing.length != 0}
			<p class="warning">
				Target dirs: [./{dirs.existing.join(', ./')}] already exist. Existing files will be
				overwritten and directory will be polluted.
			</p>
		{/if}
	{:else}
		<p>No workspace directory selected.</p>
	{/if}

	<div class="export-container">
		<button on:click={handle_export} class="export"> Export </button>
	</div>
</div>

<style lang="scss">
	.export-container {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	button.export {
		width: 100px;
		padding: 0 16px;
		height: 36px;
		font-size: medium;

		border: 1px solid dodgerblue;

		&:hover {
			background-color: #0b2d47;
			color: #ddd;
		}

		&:active {
			background-color: #065a9a;
		}

	}

	div {
		color: #ccc;
	}

	div.row {
		display: flex;
		align-items: center;
	}

	div.export_panel {
		position: relative;
		overflow-y: scroll;
		overflow-x: hidden;
	}

	.warning {
		color: rgb(255, 191, 0);
	}
</style>
