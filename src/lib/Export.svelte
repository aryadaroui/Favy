<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { exists } from '@tauri-apps/api/fs';
	import { workspace_dir, photo_map } from './stores';
	import { onMount } from 'svelte';

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

		const photos = Array.from($photo_map, ([name, value]) => ({ name, ...value }));

		invoke('export', {
			$workspace_dir,
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

	<button>Heart</button> Copy dir: ./love <br />
	<button>Star3</button> Copy dir: ./star3 <br />
	<button>Star2</button> Copy dir: ./star2 <br />
	<button>Star1</button> Copy dir: ./star1 <br />
	<button>Hate</button> Copy dir: ./hate <br />

	<button
		on:click={() => {
			settings.delete_original = !settings.delete_original;
			settings.update();
		}}
		class="delete-original">Delete original?</button>
	<span class:warning={settings.delete_original}
		>Original files
		{#if settings.delete_original}
			WILL
		{:else}
			will not
		{/if}
		be deleted.</span> <br />

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

	<button on:click={handle_export} class="export"> Export </button>
</div>

<style>
	button {
		margin: 0.5rem;
		height: 30px;
		border: 1px solid #aaa;
	}

	button.export {
		width: 100px;
		/* background-color: rgba(0, 191, 255, 0.2); */
		border: 1px solid dodgerblue;
	}

	div {
		color: #ccc;
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
